//! Interrupt vector table and interrupt handler generation
use super::{PortInstance, State};

pub type Table = [unsafe extern "C" fn() -> !; 256];

/// Generate the interrupt vector table for the specified system trait type.
pub(super) const fn new_table<Traits: PortInstance>() -> Table {
    seq_macro::seq!(I in 0..256 {
        [ #(
            if Traits::INTERRUPT_HANDLERS.get(I).is_some() {
                fl_handler_stage1::<Traits, I>
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
    use core::mem::ManuallyDrop;
    use r3_core::{kernel::interrupt::InterruptHandlerFn, utils::Frozen};
    use staticvec::StaticVec;

    trait Inner {
        const FNS: &'static [Frozen<InterruptHandlerFn>];
    }

    impl<Traits: PortInstance> Inner for Traits {
        const FNS: &'static [Frozen<InterruptHandlerFn>] = {
            let fns: StaticVec<InterruptHandlerFn, 256> = StaticVec::new();
            // FIXME: `StaticVec: !~const Destruct`
            let mut fns = ManuallyDrop::new(fns);
            seq_macro::seq!(I in 0..256 {
                if Traits::INTERRUPT_HANDLERS.get(I).is_some() {
                    fns.push(sl_handler_trampoline::<Traits, I>);
                }
            });
            Frozen::leak_slice(&fns)
        };
    }

    <Traits as Inner>::FNS.as_ptr() as usize
}

#[naked]
#[repr(align(4))]
unsafe extern "C" fn fl_handler_stage1<Traits: PortInstance, const I: usize>() -> ! {
    unsafe {
        core::arch::asm!(
            "
            bsr.a _{fl_handler_stage1}

            # The first-level stage2 handler will find this via the last return address
            .word _{sl_handler}
            ",
            fl_handler_stage1 = sym State::fl_handler_stage2::<Traits>,
            sl_handler = sym sl_handler_trampoline::<Traits, I>,
            options(noreturn),
        );
    }
}

unsafe extern "C" fn sl_handler_trampoline<Traits: PortInstance, const I: usize>() {
    // I hoped that this would be devirtualized and inlined, but neither
    // did happen
    //     unsafe { Traits::INTERRUPT_HANDLERS.get(I).unwrap()() }

    use r3_core::kernel::interrupt::InterruptHandlerFn;

    trait Inner<const I: usize> {
        const FN: InterruptHandlerFn;
    }

    impl<Traits: PortInstance, const I: usize> Inner<I> for Traits {
        const FN: InterruptHandlerFn = Traits::INTERRUPT_HANDLERS.get(I).unwrap_or(noop as _);
    }

    extern "C" fn noop() {}

    unsafe { <Traits as Inner<I>>::FN() }
}

unsafe extern "C" fn unhandled_interrupt() -> ! {
    panic!("unhandled interrupt")
}
