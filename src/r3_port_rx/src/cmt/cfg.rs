//! The public interface for the Compare Match Timer (CMT) timer driver.
use r3_core::kernel::{InterruptNum, InterruptPriority};

/// Attach the implementation of [`PortTimer`] based on Compare Match Timer
/// (CMT) to a given kernel trait type. This macro also implements [`Timer`] on
/// the kernel trait type.
/// **Requires [`CmtOptions`].**
///
/// [`PortTimer`]: r3_kernel::PortTimer
/// [`Timer`]: crate::Timer
///
/// You should do the following:
///
///  - Implement [`CmtOptions`] on the kernel trait type `$Traits`.
///  - Call `$Traits::configure_timer()` in your configuration function.
///    See the following example.
///
/// ```rust,ignore
/// r3_port_rx::use_cmt!(unsafe impl PortTimer for SystemTraits);
///
/// impl r3_port_rx::CmtOptions for SystemTraits {
///     const FREQUENCY: u64 = 96_000_000;
///     const PREDIVIDER: u64 = 512;
/// }
///
/// const fn configure_app(b: &mut r3_kernel::Cfg<SystemTraits>) -> Objects {
///     SystemTraits::configure_timer(b);
///     /* ... */
/// }
/// ```
///
/// # Safety
///
///  - `CmtOptions` must be configured correctly.
///
#[macro_export]
macro_rules! use_cmt {
    (unsafe impl PortTimer for $Traits:ty) => {
        const _: () = {
            use $crate::r3_core::{
                kernel::{traits, Cfg},
                utils::Init,
            };
            use $crate::r3_kernel::{PortTimer, System, UTicks};
            use $crate::r3_portkit::tickless;
            use $crate::{cmt, CmtOptions, Timer};

            impl PortTimer for $Traits {
                const MAX_TICK_COUNT: UTicks = u32::MAX;
                const MAX_TIMEOUT: UTicks = u32::MAX;

                unsafe fn tick_count() -> UTicks {
                    // Safety: We are just forwarding the call
                    unsafe { cmt::imp::tick_count::<Self>() }
                }

                unsafe fn pend_tick() {
                    // Safety: We are just forwarding the call
                    unsafe { cmt::imp::pend_tick::<Self>() }
                }

                unsafe fn pend_tick_after(tick_count_delta: UTicks) {
                    // Safety: We are just forwarding the call
                    unsafe { cmt::imp::pend_tick_after::<Self>(tick_count_delta) }
                }
            }

            impl Timer for $Traits {
                unsafe fn init() {
                    unsafe { cmt::imp::init::<Self>() }
                }
            }

            static mut TIMER_STATE: cmt::imp::TimerState<
                <$Traits as cmt::imp::TimerInstance>::TicklessState,
            > = Init::INIT;

            // Safety: Only `use_cmt!` is allowed to `impl` this
            unsafe impl cmt::imp::TimerInstance for $Traits {
                type TicklessState = tickless::TicklessState<{ Self::TICKLESS_CFG }>;

                fn timer_state() -> *mut cmt::imp::TimerState<Self::TicklessState> {
                    unsafe { core::ptr::addr_of_mut!(TIMER_STATE) }
                }
            }

            impl $Traits {
                pub const fn configure_timer<C>(b: &mut Cfg<C>)
                where
                    C: ~const traits::CfgBase<System = System<Self>>
                        + ~const traits::CfgInterruptLine,
                {
                    cmt::imp::configure(b);
                }
            }
        };
    };
}

/// The options for [`use_cmt!`].
pub trait CmtOptions {
    /// The base address of the memory-mapped registers exposed by a Compare
    /// Match Timer instance. Defaults to `0x0008_8000` (CMT0/1) when
    /// unspecified.
    const CMT_BASE: *mut () = 0x0008_8000 as _;

    /// The numerator of the effective input clock rate (usually PCLK) of the
    /// timer unit. This will be further divided by [`Self::PREDIVIDER`] to
    /// determine the actual timer clock.
    const FREQUENCY: u64;

    /// The denominator of the effective input clock rate (usually PCLK) of the
    /// timer unit. Defaults to `1`.
    const FREQUENCY_DENOMINATOR: u64 = 1;

    /// Set the divider ratio for the predivider (`CMCR.CKS`). Must be one of
    /// `[8, 32, 128, 512]`.
    const PREDIVIDER: u64;

    /// The maximum permissible timer interrupt latency, measured in hardware
    /// timer cycles.
    ///
    /// Defaults to `min(FREQUENCY / FREQUENCY_DENOMINATOR / PREDIVIDER / 100,
    /// 0x8000)` (10 milliseconds maximum).
    const HEADROOM: u16 = min128(
        Self::FREQUENCY as u128
            / Self::FREQUENCY_DENOMINATOR as u128
            / Self::PREDIVIDER as u128
            / 100,
        0x8000,
    ) as u16;

    /// The interrupt number of the first channel of the specified CMT instance.
    /// Defaults to `28` (CMT0).
    const INTERRUPT_NUM: InterruptNum = 28;

    /// The IPR register used to set the specified interrupt line's
    /// priority. Defaults to `Some(4)`.
    const IPR_INDEX: Option<usize> = Some(4);

    /// The interrupt priority. Defaults to `4`.
    const INTERRUPT_PRIORITY: InterruptPriority = 4;
}

const fn min128(x: u128, y: u128) -> u128 {
    if x < y {
        x
    } else {
        y
    }
}
