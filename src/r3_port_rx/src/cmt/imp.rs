//! The implementation of the timer driver based on Compare Match Timer (CMT).
use r3_core::{
    kernel::{traits, Cfg, InterruptLine, StartupHook, StaticInterruptHandler},
    utils::Init,
};
use r3_kernel::{KernelTraits, PortToKernel, System, UTicks};
use r3_portkit::tickless::{TicklessCfg, TicklessOptions, TicklessStateTrait};
use rsrx::cmt;
use tock_registers::{
    fields::FieldValue,
    interfaces::{ReadWriteable, Readable, Writeable},
};

use crate::{cmt::cfg::CmtOptions, Icu};

/// Implemented on a kernel trait type by [`use_cmt!`].
///
/// # Safety
///
/// Only meant to be implemented by [`use_cmt!`].
pub unsafe trait TimerInstance: KernelTraits + CmtOptions + Icu {
    const TICKLESS_CFG: TicklessCfg = match TicklessCfg::new(TicklessOptions {
        hw_freq_num: <Self as CmtOptions>::FREQUENCY,
        hw_freq_denom: <Self as CmtOptions>::FREQUENCY_DENOMINATOR
            .checked_mul(<Self as CmtOptions>::PREDIVIDER)
            .expect("frequency denominator overflowed"),
        hw_headroom_ticks: <Self as CmtOptions>::HEADROOM as u32,
        force_full_hw_period: true,
        resettable: true,
    }) {
        Ok(x) => x,
        Err(e) => e.panic(),
    };

    type TicklessState: TicklessStateTrait;

    fn timer_state() -> *mut TimerState<Self::TicklessState>;

    const CMT_CONTROL_CKS: FieldValue<u16, cmt::Control::Register> = match Self::PREDIVIDER {
        8 => cmt::Control::CKS::PclkDividedBy8,
        32 => cmt::Control::CKS::PclkDividedBy32,
        128 => cmt::Control::CKS::PclkDividedBy128,
        512 => cmt::Control::CKS::PclkDividedBy512,
        _ => unreachable!(),
    };
}

trait TimerInstanceExt: TimerInstance {
    #[inline(always)]
    fn cmt() -> &'static cmt::Registers {
        // Safety: Verified by the user of `use_cmt!`
        unsafe { &*(Self::CMT_BASE as *const cmt::Registers) }
    }
}
impl<T: TimerInstance> TimerInstanceExt for T {}

pub struct TimerState<TicklessState> {
    tickless_state: TicklessState,
    /// The last known value of the simulated 32-bit counter. The current value
    /// can be calculated using `hw_tick_count32: u32` and `cmt.channels[1].
    /// cmcnt.get(): u32`, provided that the timer hasn't advanced by more than
    /// 2¹⁶ cycles since the previous update of `hw_tick_count32`.
    hw_tick_count32: u32,
    /// For how many cycles should we wait before `timer_tick` should be called?
    hw_tick_remaining: u32,
}

impl<TicklessState: Init> Init for TimerState<TicklessState> {
    const INIT: Self = Self {
        tickless_state: Init::INIT,
        hw_tick_count32: 0,
        hw_tick_remaining: 0,
    };
}

/// The configuration function.
pub const fn configure<C, Traits: TimerInstance>(b: &mut Cfg<C>)
where
    C: ~const traits::CfgBase<System = System<Traits>> + ~const traits::CfgInterruptLine,
{
    InterruptLine::define()
        .line(Traits::INTERRUPT_NUM)
        .enabled(true)
        .finish(b);
    StaticInterruptHandler::define()
        .line(Traits::INTERRUPT_NUM)
        .start(handle_tick::<Traits>)
        .finish(b);

    if <Traits as CmtOptions>::IPR_INDEX.is_some() {
        StartupHook::define()
            .start(|| {
                Traits::set_interrupt_group_priority(
                    <Traits as CmtOptions>::IPR_INDEX.unwrap(),
                    <Traits as CmtOptions>::INTERRUPT_PRIORITY,
                )
                .unwrap()
            })
            .finish(b);
    }
}

/// Implements [`crate::Timer::init`]
#[inline]
pub fn init<Traits: TimerInstance>() {
    let cmt = Traits::cmt();

    // Stop the timers
    cmt.cmstr
        .write(cmt::Start::STR0::Stop + cmt::Start::STR1::Stop);

    // Unit 0: Variable interval, interrupts enabled
    // Unit 1: Free-running (period = 2¹⁶), interrupts disabled
    cmt.channels[0]
        .cmcr
        .write(Traits::CMT_CONTROL_CKS + cmt::Control::CMIE::SET);
    cmt.channels[1].cmcr.write(Traits::CMT_CONTROL_CKS);
    cmt.channels[0].cmcnt.set(0);
    cmt.channels[1].cmcnt.set(0);
    cmt.channels[1].cmcor.set(u16::MAX);

    // Start the timers
    cmt.cmstr
        .write(cmt::Start::STR0::Start + cmt::Start::STR1::Start);
}

/// Calculate the value of the simulated 32-bit counter based on the current
/// CMT state.
#[inline]
fn current_hw_tick_count32(hw_tick_count32: u32, current_hw_tick_count16: u16) -> u32 {
    let hw_tick_count16: u16 = hw_tick_count32 as _;
    hw_tick_count32.wrapping_add(current_hw_tick_count16.wrapping_sub(hw_tick_count16) as u32)
}

/// Implements [`r3_kernel::PortTimer::tick_count`]
///
/// # Safety
///
/// Only meant to be referenced by `use_cmt!`.
pub unsafe fn tick_count<Traits: TimerInstance>() -> UTicks {
    // Safety: CPU Lock protects it from concurrent access
    let tstate = unsafe { &mut *Traits::timer_state() };

    // Calculate the value of the simulated 32-bit counter
    let cmt = Traits::cmt();
    let cur_hw_tick_count32 =
        current_hw_tick_count32(tstate.hw_tick_count32, cmt.channels[1].cmcnt.get());

    let tcfg = &Traits::TICKLESS_CFG;
    tstate.tickless_state.tick_count(tcfg, cur_hw_tick_count32)
}

/// Implements [`r3_kernel::PortTimer::pend_tick`]
///
/// # Safety
///
/// Only meant to be referenced by `use_cmt!`.
pub unsafe fn pend_tick<Traits: TimerInstance>() {
    // Safety: CPU Lock protects it from concurrent access
    let tstate = unsafe { &mut *Traits::timer_state() };

    // Do `timer_tick` on the next interrupt
    tstate.hw_tick_remaining = 0;

    // Configure unit 0 to generate an interrupt on the next cycle.
    // (We can't make this happen immediately, unfortunately.)
    let cmt = Traits::cmt();
    cmt.channels[0].cmcor.set(0);
    cmt.channels[0].cmcnt.set(0);
}

/// Implements [`r3_kernel::PortTimer::pend_tick_after`]
///
/// # Safety
///
/// Only meant to be referenced by `use_cmt!`.
pub unsafe fn pend_tick_after<Traits: TimerInstance>(tick_count_delta: UTicks) {
    // Safety: CPU Lock protects it from concurrent access
    let tstate = unsafe { &mut *Traits::timer_state() };

    // Update the reference time of the simulated 32-bit counter
    let cmt = Traits::cmt();
    let cur_hw_tick_count32 =
        current_hw_tick_count32(tstate.hw_tick_count32, cmt.channels[1].cmcnt.get());
    tstate.hw_tick_count32 = cur_hw_tick_count32;

    let tcfg = &Traits::TICKLESS_CFG;
    let hw_ticks = tstate
        .tickless_state
        .mark_reference_and_measure(tcfg, cur_hw_tick_count32, tick_count_delta)
        .hw_ticks;

    debug_assert!(hw_ticks <= u16::MAX as u32);

    tstate.hw_tick_remaining = hw_ticks;

    // We use the timer interrupts to maintain the simulated 32-bit counter.
    // The interrupt period must be maintained under 2¹⁶ cycles, or we'll lose
    // track of it.
    let max_cmcor = u16::MAX - <Traits as CmtOptions>::HEADROOM;

    // Schedule unit 0
    cmt.cmstr.modify(cmt::Start::STR1::Stop);
    let _ = InterruptLine::<System<Traits>>::from_num(Traits::INTERRUPT_NUM).clear();
    cmt.channels[0]
        .cmcor
        .set(hw_ticks.saturating_sub(1).min(max_cmcor as u32) as u16);
    cmt.channels[0].cmcnt.set(0);
    cmt.cmstr.modify(cmt::Start::STR1::Start);
}

#[inline]
fn handle_tick<Traits: TimerInstance>() {
    // Safety: CPU Lock protects it from concurrent access
    let tstate = unsafe { &mut *Traits::timer_state() };

    // Update the reference time of the simulated 32-bit counter
    let cmt = Traits::cmt();
    let cur_hw_tick_count32 =
        current_hw_tick_count32(tstate.hw_tick_count32, cmt.channels[1].cmcnt.get());
    tstate.hw_tick_remaining = tstate
        .hw_tick_remaining
        .saturating_sub(cur_hw_tick_count32.wrapping_sub(tstate.hw_tick_count32));
    tstate.hw_tick_count32 = cur_hw_tick_count32;

    // Do the rest of the steps only if `tstate.hw_tick_remaining` reaches zero.
    // The maintenance opertaion of `r3_portkit::tickless` is somewhat compute-
    // intensive, so skipping this might save some energy.
    if tstate.hw_tick_remaining == 0 {
        let tcfg = &Traits::TICKLESS_CFG;
        tstate
            .tickless_state
            .mark_reference(tcfg, cur_hw_tick_count32);

        // Safety: CPU Lock inactive, an interrupt context
        unsafe { Traits::timer_tick() };
    }
}
