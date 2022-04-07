use core::{cell::UnsafeCell, mem::MaybeUninit, slice};
use r3_core::{
    kernel::{
        traits, ClearInterruptLineError, EnableInterruptLineError, InterruptNum, InterruptPriority,
        PendInterruptLineError, QueryInterruptLineError, SetInterruptLinePriorityError,
    },
    utils::Init,
};
use r3_kernel::{KernelTraits, Port, PortToKernel, System, TaskCb};
use r3_portkit::pptext::pp_asm;

use crate::ThreadingOptions;

/// Implemented on a kernel trait type by [`use_port!`].
///
/// # Safety
///
/// Only meant to be implemented by [`use_port!`].
pub unsafe trait PortInstance:
    KernelTraits + Port<PortTaskState = TaskState> + ThreadingOptions
{
}

/// Stores the value of `Traits::state().running_task_ptr()` so that it can
/// be accessed in naked functions. This field is actually of type
/// `*mut Option<&'static TaskCb<Traits>>`.
///
/// A global variable suffices because there can be only one instance of the
/// port. `[ref:rx_single_instance]`
static mut RUNNING_TASK_PTR: usize = 0;

static mut DISPATCH_PENDING: bool = false;

#[export_name = "r3_port_rx_INTERRUPTS"]
static INTERRUPTS: [unsafe extern "C" fn(); 256] = {
    // TODO
    #[naked]
    unsafe extern "C" fn noop_int_handler() {
        unsafe { pp_asm!("rte", options(noreturn)) };
    }
    [noop_int_handler; 256]
};

#[used]
static mut DUMMY: usize = 0;

/// Processor Status Word
mod psw {
    /// `PSW.I` - Interrpt enable bit
    pub const I: u32 = 1 << 16;
    /// `PSW.U` - Stack pointer select bit
    pub const U: u32 = 1 << 17;
    /// `PSW.IPL` - Processor interrupt priority level
    pub const IPL_MASK: u32 = 0b1111;

    // FIXME: Register operands aren't supported for cg_gcc + RX
    #[cfg(any())]
    #[inline]
    pub fn read() -> u32 {
        let psw: u32;
        unsafe {
            core::arch::asm!(
                "mvfc psw, {}",
                out(reg) psw,
                options(preserves_flags, nostack, nomem),
            );
        }
        psw
    }

    #[naked]
    pub extern "C" fn read() -> u32 {
        unsafe {
            core::arch::asm!(
                "mvfc psw, r1
                rts",
                options(noreturn),
            );
        }
    }
}

/// The initial PSW value for a task thread. Interrupt enabled and User Stack
/// Pointer selected.
const TASK_DEFAULT_PSW: u32 = psw::I | psw::U;

const TASK_DEFAULT_FPSW: u32 = 0;

pub struct State {}

impl const Default for State {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TaskState {
    sp: UnsafeCell<u32>,
}

unsafe impl Sync for TaskState {}

impl Init for TaskState {
    #[allow(clippy::declare_interior_mutable_const)] // it's intentional
    const INIT: Self = Self {
        sp: UnsafeCell::new(0),
    };
}

impl State {
    #[inline(always)]
    pub unsafe fn port_boot<Traits: PortInstance>(&self) -> ! {
        // Update PSW for the boot phase
        unsafe { pp_asm!("mvtipl #15", options(nomem, preserves_flags)) };
        unsafe { pp_asm!("clrpsw i", options(nomem, preserves_flags)) };

        // Set the interrupt vector table base
        unsafe {
            pp_asm!(
                "mvtc #_r3_port_rx_INTERRUPTS, intb",
                options(nomem, preserves_flags)
            )
        };

        // FIXME: Work-around for DCE not honoring `sym` operands
        //        <https://github.com/rust-lang/rustc_codegen_gcc/issues/157>
        unsafe {
            DUMMY = Self::push_second_level_state_and_dispatch::<Traits> as usize
                + Self::idle_task::<Traits> as usize
                + Self::choose_and_get_next_task::<Traits> as usize
                + Self::yield_cpu_inner::<Traits> as usize;
        }

        // Safety: We are a port, so it's okay to call this
        unsafe {
            <Traits as PortToKernel>::boot();
        }
    }

    #[inline(always)]
    pub unsafe fn dispatch_first_task<Traits: PortInstance>(&'static self) -> ! {
        // [tag:running_task_ptr_set_in_dft]
        unsafe { RUNNING_TASK_PTR = Traits::state().running_task_ptr() as usize };

        unsafe {
            pp_asm!("
                # Enter a dispatcher context
                setpsw u

                # `dispatch` needs stack, so borrow ISP. This is safe because of
                # [ref:flexible_unmanaged_interrupts].
                mvfc isp, r0

                bra _{push_second_level_state_and_dispatch}.dispatch
                ",
                push_second_level_state_and_dispatch =
                    sym Self::push_second_level_state_and_dispatch::<Traits>,
                options(noreturn),
            )
        }
    }

    ///
    /// Reset MSP to `interrupt_stack_top()`, release CPU Lock, and start
    /// executing the idle loop.
    ///
    /// # Safety
    ///
    /// Dispatcher context.
    #[inline(never)]
    unsafe extern "C" fn idle_task<Traits: PortInstance>() -> ! {
        // TODO: Use `Traits::USE_WAIT`
        unsafe {
            pp_asm!(
                "
                # Zero SP
                mov #0, r0

                # Transition to a task context
                mvtipl #0
                setpsw i

            0:
                wait
                bra 0b
                ",
                options(noreturn)
            );
        }
    }

    #[inline(always)]
    pub unsafe fn yield_cpu<Traits: PortInstance>(&'static self) {
        unsafe {
            pp_asm!(
                "
                # Push the last two fields ot the first-level context state
                # and branch to the inner function.
                #
                #       sp -= 2
                #       sp[0] = return_site
                #       sp[1] = PSW
                #       pc = yield_cpu_inner
                #   return_site:
                #
                pushc psw
                bsr _{yield_cpu_inner}
                ",
                yield_cpu_inner = sym Self::yield_cpu_inner::<Traits>,
                options(preserves_flags),
            );
        }
    }

    /// The inner function of [`Self::yield_cpu`]. Uses a non-standard calling
    /// convention.
    #[naked]
    unsafe extern "C" fn yield_cpu_inner<Traits: PortInstance>() {
        unsafe {
            pp_asm!(
                "
                # Push [r14..=15] early to make room for temporaries.
                pushm r14-r15

                # If we are in an interrupt context, pend dispatch and return.
                #
                #   if PSW.I:
                #       goto InInterruptContext
                #
                mvfc psw, r14
                tst #{PSW_I}, r14
                bne 0f

                # Enter a dispatcher context
                mvtipl #15

                # Push the rest of the first level context state.
                pushm r1-r5
                pushc fpsw

                bra _{push_second_level_state_and_dispatch}

            0:              # InInterruptContext
                #
                #   if PSW.I:
                #       DISPATCH_PENDING = true
                #       return
                #
                mov #_{DISPATCH_PENDING}, r14
                mov.b #1, [r14]
                pop r14
                add #4, r0
                rte
                ",
                PSW_I = const psw::I,
                DISPATCH_PENDING = sym DISPATCH_PENDING,
                push_second_level_state_and_dispatch =
                    sym Self::push_second_level_state_and_dispatch::<Traits>,
                options(noreturn),
            );
        }
    }

    extern "C" fn choose_and_get_next_task<Traits: PortInstance>() -> Option<&'static TaskCb<Traits>>
    {
        // Safety: CPU Lock active
        unsafe { Traits::choose_running_task() };

        unsafe { *Traits::state().running_task_ptr() }
    }

    /// Do the following steps:
    ///
    ///  - **Don't** push the first-level state.
    ///  - If the current task is not an idle task,
    ///     - Push the second-level state.
    ///     - Store SP to the current task's `TaskState`.
    ///  - **`dispatch:`** (alternate entry point)
    ///     - Call [`r3_kernel::PortToKernel::choose_running_task`].
    ///     - Restore SP from the next scheduled task's `TaskState`.
    ///  - If there's no task to schedule, branch to [`Self::idle_task`].
    ///  - Pop the second-level state of the next scheduled task.
    ///  - **`pop_first_level_state:`** (alternate entry point)
    ///     - Pop the first-level state of the next scheduled task.
    ///
    /// # Safety
    ///
    ///  - The processor state should be in a dispatcher context.
    ///  - If the current task is an idle task, SP should point to the
    ///    first-level state on the current task's stack. Otherwise, SP must be
    ///    zero.
    /// - `dispatch:` needs a stack space.
    ///
    #[naked]
    unsafe extern "C" fn push_second_level_state_and_dispatch<Traits: PortInstance>() -> ! {
        unsafe {
            pp_asm!("
                # Skip saving the second-level state if the current context
                # is an idle task. Also, in this case, we don't have a stack,
                # but `choose_and_get_next_task` needs one. Therefore we borrow
                # the interrupt stack. Otherwise, push the second-level state.
                #
                #   if usp == 0:
                #       <running_task is None>
                #       goto WasIdleTask
                #   else:
                #       /* ... */
                #
                cmp #0, r0
                beq 0f

                #   r1 = &running_task;
                #   r2 = running_task;
                mov #_{RUNNING_TASK_PTR}, r1
                mov [r1], r1

                # Push the second-level context state.
                # TODO: RXv1 lacks `mvfaclo`
                # mvfaclo r4
                mvfachi r5
                pushm r4-r13

                # Store SP to `TaskState`
                #
                #    r1.port_task_state.sp = usp
                #
                mov r0, [r1]

            .global _{push_second_level_state_and_dispatch}.dispatch
            _{push_second_level_state_and_dispatch}.dispatch:
            1:
                # Choose the next task to run. `choose_and_get_next_task`
                # returns the new value of `running_task`.
                bsr _{choose_and_get_next_task}

                # Restore SP from `TaskState`
                #
                #    <r1 = running_task>
                #    if r1.is_none():
                #        goto idle_task;
                #
                #    usp = r1.port_task_state.sp
                #
                cmp #0, r1
                beq _{idle_task}
                mov [r1], r0

                # Pop the second-level context state.
                popm r4-r13
                mvtaclo r4
                mvtachi r5

                # Resume the next task by restoring the first-level state
                popc fpsw
                popm r1-r5
                popm r14-r15
                rte

            0:      # WasIdleTask
                # Copy ISP to USP. This is safe because of
                # [ref:flexible_unmanaged_interrupts].
                #
                #    usp = isp;
                #    goto {push_second_level_state_and_dispatch}.dispatch;
                #
                mvfc isp, r0
                bra 1b
            ",
                choose_and_get_next_task = sym Self::choose_and_get_next_task::<Traits>,
                push_second_level_state_and_dispatch =
                    sym Self::push_second_level_state_and_dispatch::<Traits>,
                idle_task = sym Self::idle_task::<Traits>,
                RUNNING_TASK_PTR = sym RUNNING_TASK_PTR,
                options(noreturn),
            );
        }
    }

    #[inline(always)]
    pub unsafe fn exit_and_dispatch<Traits: PortInstance>(
        &'static self,
        _task: &'static TaskCb<Traits>,
    ) -> ! {
        unsafe {
            pp_asm!("
                bra _{push_second_level_state_and_dispatch}.dispatch
                ",
                push_second_level_state_and_dispatch =
                    sym Self::push_second_level_state_and_dispatch::<Traits>,
                options(noreturn),
            );
        }
    }

    #[inline(always)]
    pub unsafe fn enter_cpu_lock<Traits: PortInstance>(&self) {
        unsafe { pp_asm!("mvtipl #15", options(preserves_flags, nostack)) };
    }

    #[inline(always)]
    pub unsafe fn leave_cpu_lock<Traits: PortInstance>(&'static self) {
        unsafe { pp_asm!("mvtipl #0", options(preserves_flags, nostack)) };
    }

    pub unsafe fn initialize_task_state<Traits: PortInstance>(
        &self,
        task: &'static TaskCb<Traits>,
    ) {
        let stack = task.attr.stack.as_ptr();
        let mut sp = (stack as *mut u8).wrapping_add(stack.len()) as *mut MaybeUninit<u32>;
        // TODO: Enforce minimum stack size

        let preload_all = cfg!(feature = "preload-registers");

        // The return target of the entry point call
        sp = sp.wrapping_sub(1);
        unsafe {
            *sp = MaybeUninit::new(
                <System<Traits> as traits::KernelBase>::raw_exit_task as usize as u32,
            )
        };

        // First-level state (always saved and restored as part of our exception
        // entry/return sequence)
        let first_level = unsafe {
            sp = sp.wrapping_sub(10);
            slice::from_raw_parts_mut(sp, 10)
        };

        // FPSW
        first_level[0] = MaybeUninit::new(TASK_DEFAULT_FPSW);
        // R1: Parameter to the entry point
        first_level[1] = unsafe { core::mem::transmute(task.attr.entry_param) };
        // R2-R5, R14-R15: Uninitialized
        if preload_all {
            first_level[2] = MaybeUninit::new(0x02020202);
            first_level[3] = MaybeUninit::new(0x03030303);
            first_level[4] = MaybeUninit::new(0x04040404);
            first_level[5] = MaybeUninit::new(0x05050505);
            first_level[6] = MaybeUninit::new(0x14141414);
            first_level[7] = MaybeUninit::new(0x15151515);
        }
        // PC: The entry point
        first_level[8] = MaybeUninit::new(task.attr.entry_point as usize as u32);
        // PSW
        first_level[9] = MaybeUninit::new(TASK_DEFAULT_PSW);

        // Second-level state (saved and restored only when we are doing context
        // switching)
        let second_level = unsafe {
            sp = sp.wrapping_sub(10);
            slice::from_raw_parts_mut(sp, 10)
        };

        // A0, R6-R13: Uninitialized
        if preload_all {
            second_level[0] = MaybeUninit::new(0xa0a0a0a0);
            second_level[1] = MaybeUninit::new(0xa1a1a1a1);
            second_level[2] = MaybeUninit::new(0x06060606);
            second_level[3] = MaybeUninit::new(0x07070707);
            second_level[4] = MaybeUninit::new(0x08080808);
            second_level[5] = MaybeUninit::new(0x09090909);
            second_level[6] = MaybeUninit::new(0x10101010);
            second_level[7] = MaybeUninit::new(0x11111111);
            second_level[8] = MaybeUninit::new(0x12121212);
            second_level[9] = MaybeUninit::new(0x13131313);
        }

        let task_state = &task.port_task_state;
        unsafe { *task_state.sp.get() = sp as _ };
    }

    #[inline(always)]
    pub fn is_cpu_lock_active<Traits: PortInstance>(&self) -> bool {
        (psw::read() & psw::IPL_MASK) == 15
    }

    pub fn is_task_context<Traits: PortInstance>(&self) -> bool {
        (psw::read() & psw::I) != 0
    }

    #[inline]
    pub fn is_interrupt_context<Traits: PortInstance>(&self) -> bool {
        self.is_scheduler_active::<Traits>() && !self.is_task_context::<Traits>()
    }

    #[inline]
    pub fn is_scheduler_active<Traits: PortInstance>(&self) -> bool {
        // `RUNNING_TASK_PTR` is assigned by `dispatch_first_task`
        // [ref:running_task_ptr_set_in_dft]
        unsafe { RUNNING_TASK_PTR != 0 }
    }

    pub fn set_interrupt_line_priority<Traits: PortInstance>(
        &'static self,
        num: InterruptNum,
        priority: InterruptPriority,
    ) -> Result<(), SetInterruptLinePriorityError> {
        todo!()
    }

    #[inline]
    pub fn enable_interrupt_line<Traits: PortInstance>(
        &'static self,
        num: InterruptNum,
    ) -> Result<(), EnableInterruptLineError> {
        todo!()
    }

    #[inline]
    pub fn disable_interrupt_line<Traits: PortInstance>(
        &self,
        num: InterruptNum,
    ) -> Result<(), EnableInterruptLineError> {
        todo!()
    }

    #[inline]
    pub fn pend_interrupt_line<Traits: PortInstance>(
        &'static self,
        num: InterruptNum,
    ) -> Result<(), PendInterruptLineError> {
        todo!()
    }

    #[inline]
    pub fn clear_interrupt_line<Traits: PortInstance>(
        &self,
        num: InterruptNum,
    ) -> Result<(), ClearInterruptLineError> {
        todo!()
    }

    #[inline]
    pub fn is_interrupt_line_pending<Traits: PortInstance>(
        &self,
        num: InterruptNum,
    ) -> Result<bool, QueryInterruptLineError> {
        todo!()
    }
}

/// Used by `use_port!`
pub const fn validate<Traits: PortInstance>() {
    // [tag:flexible_unmanaged_interrupts]
    assert!(
        Traits::CPU_LOCK_PRIORITY_MASK == 15,
        "`CPU_LOCK_PRIORITY_MASK` having a value other than `15` is not supported yet"
    );

    // [tag:rx_nested_interrupts]
    assert!(
        !Traits::SUPPORT_NESTING,
        "nested interrupts aren't supported yet"
    );
}
