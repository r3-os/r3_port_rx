#![feature(asm_experimental_arch)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]
#![feature(const_mut_refs)]
#![feature(asm_sym)]
#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

// Startup
// -----------------------------------------------------------------------

use rx_supplementary_builtins as _;

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
port::use_cmt!(unsafe impl PortTimer for SystemTraits);

impl port::ThreadingOptions for SystemTraits {}

impl port::CmtOptions for SystemTraits {
    const FREQUENCY: u64 = 96_000_000;
    const PREDIVIDER: u64 = 32;
}

// Application code
// -----------------------------------------------------------------------

use r3::{kernel::StaticTask, prelude::*};
use rsrx::devices::rx63n as device;
use tock_registers::interfaces::ReadWriteable;

const _: Objects = r3_kernel::build!(SystemTraits, configure_app => Objects);

struct Objects {}

const fn configure_app(b: &mut r3_kernel::Cfg<SystemTraits>) -> Objects {
    b.num_task_priority_levels(4);
    SystemTraits::configure_timer(b);

    StaticTask::define()
        .start(task1_body)
        .priority(2)
        .active(true)
        .finish(b);

    Objects {}
}

#[no_mangle]
fn task1_body() {
    let device::Peripherals { PORTS, .. } = unsafe { device::Peripherals::steal() };

    // Use PA0 (LED on GR-CITRUS) as a GPIO output port
    PORTS
        .porta_pmr
        .modify(device::ports::RouteToPeripheral::B0::Disable);
    PORTS.porta_pdr.modify(device::ports::Direction::B0::Output);

    let mut state = false;

    loop {
        // Toggle PA0
        state = !state;
        PORTS.porta_podr.modify(if state {
            device::ports::Data::B0::SET
        } else {
            device::ports::Data::B0::CLEAR
        });

        System::sleep(r3::time::Duration::from_millis(200)).unwrap();
    }
}
