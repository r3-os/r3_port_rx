//! Interrupt vector table and interrupt handler generation
use super::{PortInstance, State};

pub type Table = [unsafe extern "C" fn() -> !; 256];

/// Generate the interrupt vector table for the specified system trait type.
pub(super) const fn new_table<Traits: PortInstance>() -> Table {
    seq_macro::seq!(I in 0..256 {
        [ #(
            if Traits::INTERRUPT_HANDLERS.get(I).is_some() {
                zl_handler_stage1::<Traits, I>
            } else {
                unhandled_interrupt
            },
        )* ]
    })
}

/// FIXME: Work-around for DCE not honoring `sym` operands
/// <https://github.com/rust-lang/rustc_codegen_gcc/issues/157>
#[inline]
pub(super) fn keep_handlers<Traits: PortInstance>() -> usize {
    let mut i = 0;

    seq_macro::seq!(I in 0..256 {
        if Traits::INTERRUPT_HANDLERS.get(I).is_some() {
            i += fl_handler_trampoline::<Traits, I> as usize;
        }
    });

    i
}

#[naked]
#[repr(align(4))]
unsafe extern "C" fn zl_handler_stage1<Traits: PortInstance, const I: usize>() -> ! {
    unsafe {
        core::arch::asm!(
            "
            bsr.a _{zl_handler_stage1}

            # The zeroth-level stage2 handler will find this via the last return address
            .word _{fl_handler}
            ",
            zl_handler_stage1 = sym State::zl_handler_stage2::<Traits>,
            fl_handler = sym fl_handler_trampoline::<Traits, I>,
            options(noreturn),
        );
    }
}

unsafe extern "C" fn fl_handler_trampoline<Traits: PortInstance, const I: usize>() {
    // Very likely to be devirtualized and hopefully be inlined
    unsafe { Traits::INTERRUPT_HANDLERS.get(I).unwrap()() }
}

unsafe extern "C" fn unhandled_interrupt() -> ! {
    panic!("unhandled interrupt")
}