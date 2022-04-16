use core::ops::Range;
use r3_core::kernel::{InterruptNum, InterruptPriority};

// TODO: "Fast interrupts"
/// The valid interrupt priority values.
pub const INTERRUPT_PRIORITY_RANGE: Range<InterruptPriority> = 1..16;

/// The range of valid `InterruptNum`s.
pub const INTERRUPT_NUM_RANGE: Range<InterruptNum> = 16..256;

/// The configuration of the port.
pub trait ThreadingOptions {
    /// The priority value to which CPU Lock boosts the processor interrupt
    /// priority level. Must be in range `0..16`. Defaults to `15` when
    /// unspecified.
    ///
    /// The lower bound of [`MANAGED_INTERRUPT_PRIORITY_RANGE`] is calculated
    /// using this value as `0..CPU_LOCK_PRIORITY_MASK + 1`.
    ///
    /// [`MANAGED_INTERRUPT_PRIORITY_RANGE`]: r3_kernel::PortInterrupts::MANAGED_INTERRUPT_PRIORITY_RANGE
    const CPU_LOCK_PRIORITY_MASK: u8 = 15;

    /// Enables nested interrupts.
    const SUPPORT_NESTING: bool = false;

    /// Enables the use of the `wait` instruction in the idle task to save power.
    /// Defaults to `true`.
    const USE_WAIT: bool = true;
}

/// Defines the entry points of a port instantiation. Implemented by
/// [`use_port!`].
///
/// # Safety
///
/// This trait is not intended to be implemented in any other means.
pub unsafe trait EntryPoint {
    /// Proceed with the boot process.
    ///
    /// # Safety
    ///
    ///  - This method must not have been entered yet in the program. This
    ///    prohibits harboring multiple port instances in a single program.
    ///    <!-- [tag:rx_single_instance] -->
    ///
    ///  - `PSW.I` and `PSW.IPL` must be configured to at least disable
    ///    unmanaged interrupts. However the port is configured, it's
    ///    recommended to clear `PSW.I` because the port updates the relocatable
    ///    vector table base (`INTB`) during the boot process.
    ///
    ///  - `ISP` (interrupt stack pointer) must be initialized. There's no
    ///    requirement for the current stack pointer selection (`PSW.U`).
    ///
    ///  - This function needs a stack space to operate. It can overlap with
    ///    task stacks. If [`CPU_LOCK_PRIORITY_MASK`][]` != 15`, it can overlap
    ///    with the interrupt stack.
    ///
    ///  - The processor must be in Supervisor mode (`PSW.PM == 0`).
    ///
    /// [`CPU_LOCK_PRIORITY_MASK`]: ThreadingOptions::CPU_LOCK_PRIORITY_MASK
    unsafe fn start() -> !;
}

/// Instantiate the port. Implements the port traits ([`PortThreading`], etc.)
/// and [`EntryPoint`].
///
/// This macro doesn't provide an implementation of [`PortTimer`], which you
/// must supply one through other ways.
/// See [the crate-level documentation](crate#kernel-timing) for possible
/// options.
///
/// [`PortThreading`]: r3_kernel::PortThreading
/// [`PortTimer`]: r3_kernel::PortTimer
///
/// # Safety
///
///  - The target must really be a bare-metal RX environment.
///  - You shouldn't interfere with the port's operrations. For example, you
///    shouldn't manually modify `IPL` or `INTB` unless you know what you are
///    doing.
///  - Other components should not execute the `int` instruction.
///
#[macro_export]
macro_rules! use_port {
    (unsafe $vis:vis struct $Traits:ident) => {
        $vis struct $Traits;

        mod port_rx_impl {
            use super::$Traits;
            use $crate::r3_core::kernel::{
                ClearInterruptLineError, EnableInterruptLineError, InterruptNum, InterruptPriority,
                PendInterruptLineError, QueryInterruptLineError, SetInterruptLinePriorityError,
            };
            use $crate::r3_kernel::{
                Port, TaskCb, PortToKernel, PortInterrupts, PortThreading, UTicks, PortTimer,
            };
            use $crate::core::ops::Range;
            use $crate::threading::{
                imp::{State, TaskState, PortInstance},
                cfg::{ThreadingOptions, EntryPoint},
            };

            static PORT_STATE: State = $crate::core::default::Default::default();

            #[export_name = "r3_port_rx_INTERRUPTS"]
            #[used]
            static INTERRUPTS: $crate::threading::imp::ivt::Table =
                <$Traits as PortInstance>::IVT;

            unsafe impl PortInstance for $Traits {}

            // Assume `$Traits: KernelTraits`
            unsafe impl PortThreading for $Traits {
                type PortTaskState = TaskState;
                #[allow(clippy::declare_interior_mutable_const)]
                const PORT_TASK_STATE_INIT: Self::PortTaskState =
                    $crate::r3_core::utils::Init::INIT;

                const STACK_DEFAULT_SIZE: usize = 1024;

                // FIXME: Couldn't find any description on the stack alignment requirement
                const STACK_ALIGN: usize = 4;

                #[inline(always)]
                unsafe fn dispatch_first_task() -> ! {
                    PORT_STATE.dispatch_first_task::<Self>()
                }

                #[inline(always)]
                unsafe fn yield_cpu() {
                    PORT_STATE.yield_cpu::<Self>()
                }

                #[inline(always)]
                unsafe fn exit_and_dispatch(task: &'static TaskCb<Self>) -> ! {
                    PORT_STATE.exit_and_dispatch::<Self>(task);
                }

                #[inline(always)]
                unsafe fn enter_cpu_lock() {
                    PORT_STATE.enter_cpu_lock::<Self>()
                }

                #[inline(always)]
                unsafe fn leave_cpu_lock() {
                    PORT_STATE.leave_cpu_lock::<Self>()
                }

                #[inline(always)]
                unsafe fn initialize_task_state(task: &'static TaskCb<Self>) {
                    PORT_STATE.initialize_task_state::<Self>(task)
                }

                #[inline(always)]
                fn is_cpu_lock_active() -> bool {
                    PORT_STATE.is_cpu_lock_active::<Self>()
                }

                #[inline(always)]
                fn is_task_context() -> bool {
                    PORT_STATE.is_task_context::<Self>()
                }

                #[inline(always)]
                fn is_interrupt_context() -> bool {
                    PORT_STATE.is_interrupt_context::<Self>()
                }

                #[inline(always)]
                fn is_scheduler_active() -> bool {
                    PORT_STATE.is_scheduler_active::<Self>()
                }
            }

            unsafe impl PortInterrupts for $Traits {
                const MANAGED_INTERRUPT_PRIORITY_RANGE: Range<InterruptPriority> =
                    0..(<$Traits as ThreadingOptions>::CPU_LOCK_PRIORITY_MASK as InterruptPriority + 1);

                unsafe fn set_interrupt_line_priority(
                    line: InterruptNum,
                    priority: InterruptPriority,
                ) -> Result<(), SetInterruptLinePriorityError> {
                    PORT_STATE.set_interrupt_line_priority::<Self>(line, priority)
                }

                unsafe fn enable_interrupt_line(line: InterruptNum) -> Result<(), EnableInterruptLineError> {
                    PORT_STATE.enable_interrupt_line::<Self>(line)
                }

                unsafe fn disable_interrupt_line(line: InterruptNum) -> Result<(), EnableInterruptLineError> {
                    PORT_STATE.disable_interrupt_line::<Self>(line)
                }

                unsafe fn pend_interrupt_line(line: InterruptNum) -> Result<(), PendInterruptLineError> {
                    PORT_STATE.pend_interrupt_line::<Self>(line)
                }

                unsafe fn clear_interrupt_line(line: InterruptNum) -> Result<(), ClearInterruptLineError> {
                    PORT_STATE.clear_interrupt_line::<Self>(line)
                }

                unsafe fn is_interrupt_line_pending(
                    line: InterruptNum,
                ) -> Result<bool, QueryInterruptLineError> {
                    PORT_STATE.is_interrupt_line_pending::<Self>(line)
                }
            }

            unsafe impl EntryPoint for $Traits {
                #[inline]
                unsafe fn start() -> ! {
                    unsafe { PORT_STATE.port_boot::<$Traits>() }
                }
            }
        }

        const _: () = $crate::threading::imp::validate::<$Traits>();
    };
}
