#![feature(asm_experimental_arch)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]
#![feature(const_mut_refs)]
#![feature(asm_sym)]
#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

// Fixed vector table
// -----------------------------------------------------------------------

#[link_section = ".fixed_vector_table"]
#[used]
static _FIXED_VECTOR_TABLE: [unsafe extern "C" fn() -> !; 32] = {
    let mut table = [unhandled_exception as _; 32];
    table[31] = start as _;
    table
};

unsafe extern "C" fn unhandled_exception() -> ! {
    panic!("unhandled exception")
}

// Startup
// -----------------------------------------------------------------------

#[no_mangle]
#[link_section = ".text.start"]
#[naked]
unsafe extern "C" fn start() -> ! {
    unsafe {
        core::arch::asm!(
            "
                # Initialize .data
                mov #__sidata, r2
                mov #__sdata, r1
                mov #(__edata - __sdata), r3
                smovf

                # Initialize .bss
                mov #__sbss, r1
                mov #(__ebss - __sbss), r3
                mov #0, r2
                sstr

                # Set the stack pointers. The initial USP is only used during
                # the boot phase, so it can be identical to ISP.
                mvtc #_stack_start, usp
                mvtc #_stack_start, isp

                mvtc #0, fpsw

                bra _{main}
            ",
            main = sym main,
            options(noreturn)
        );
    }
}

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "
                # Select USP
                setpsw u
            ",
        );
    }
    unsafe { <SystemTraits as port::EntryPoint>::start() };
}

// FIXME: Why does `core::intrinsics::const_eval_select::<(&str, usize, usize),
// core::str::slice_error_fail_ct, core::str::slice_error_fail_rt, !>` contain
// a reference to this symbol?
core::arch::global_asm!(
    "
    .global _abort
_abort:
    bra _abort
    "
);

// Panic handler
// -----------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Port instantiation
// -----------------------------------------------------------------------

use r3_port_rx as port;

type System = r3_kernel::System<SystemTraits>;
port::use_port!(unsafe struct SystemTraits);

impl port::ThreadingOptions for SystemTraits {}

impl r3_kernel::PortTimer for SystemTraits {
    const MAX_TICK_COUNT: r3_kernel::UTicks = u32::MAX;
    const MAX_TIMEOUT: r3_kernel::UTicks = u32::MAX;

    unsafe fn tick_count() -> r3_kernel::UTicks {
        0 // TODO
    }

    unsafe fn pend_tick() {
        // TODO
    }

    unsafe fn pend_tick_after(_tick_count_delta: r3_kernel::UTicks) {
        // TODO
    }
}

// Application code
// -----------------------------------------------------------------------

use r3::{kernel::StaticTask, prelude::*};

/// Port direction register
const PORTA_PDR: *mut u8 = 0x0008C00A as *mut u8;
/// Port mode register
const PORTA_PMR: *mut u8 = 0x0008C06A as *mut u8;
/// Port output data register
const PORTA_PODR: *mut u8 = 0x0008C02A as *mut u8;

const _: Objects = r3_kernel::build!(SystemTraits, configure_app => Objects);

struct Objects {}

const fn configure_app(b: &mut r3_kernel::Cfg<SystemTraits>) -> Objects {
    b.num_task_priority_levels(4);

    StaticTask::define()
        .start(task1_body)
        .priority(2)
        .active(true)
        .finish(b);

    Objects {}
}

#[no_mangle]
fn task1_body() {
    unsafe {
        // Use PA0 (LED on GR-CITRUS) as a GPIO port
        PORTA_PMR.write_volatile(PORTA_PMR.read_volatile() & !0b00000001);
        // Use PA0 as an output port
        PORTA_PDR.write_volatile(PORTA_PDR.read_volatile() | 0b00000001);
    }

    loop {
        unsafe {
            // Toggle PA0
            PORTA_PODR.write_volatile(PORTA_PODR.read_volatile() ^ 0b00000001);
        }

        // Wait for a bit
        for _ in 0..5 * 1024 * 1024 {
            unsafe { core::arch::asm!("") };
        }
        // TODO: `PortTimer` is still unimplemented
        // System::sleep(r3::time::Duration::from_millis(200)).unwrap();
    }
}
