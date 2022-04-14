//! Compare Match Timer
use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

register_structs! {
    /// The memory-mapped registers exposed by Compare Match Timer.
    pub Registers {
        /// Compare match timer start register
        (0x0 => pub cmstr: ReadWrite<u16, Start::Register>),
        /// Compare match timer channels
        (0x2 => pub channels: [channel::Registers; 2]),
        (0xe => _pad1),
        (0x10 => @END),
    }
}

pub mod channel {
    use super::*;

    register_structs! {
        /// The memory-mapped registers exposed by one channel of Compare Match
        /// Timer.
        pub Registers {
            /// Compare match timer control register
            (0x0 => pub cmcr: ReadWrite<u16, Control::Register>),
            /// Compare match timer counter
            (0x2 => pub cmcnt: ReadWrite<u16>),
            /// Compare match timer constant register
            (0x4 => pub cmcor: ReadWrite<u16>),
            (0x6 => @END),
        }
    }
}

register_bitfields![u16,
    pub Control [
        /// Clock select
        CKS OFFSET(0) NUMBITS(2) [
            PclkDividedBy8 = 0,
            PclkDividedBy32 = 1,
            PclkDividedBy128 = 2,
            PclkDividedBy512 = 3,
        ],
        /// Compare match interrupt enable
        CMIE OFFSET(6) NUMBITS(1) [],
    ],
    pub Start [
        /// Count start channel 0
        STR0 OFFSET(0) NUMBITS(1) [
            Stop = 0,
            Start = 1,
        ],
        /// Count start channel 1
        STR1 OFFSET(1) NUMBITS(1) [
            Stop = 0,
            Start = 1,
        ],
    ],
];
