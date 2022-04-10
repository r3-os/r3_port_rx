//! Interrupt Control Unit (ICUa)
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::utils::Align4;

register_structs! {
    /// The memory-mapped registers exposed by Interrupt Control Unit.
    pub Registers {
        /// Interrupt request register
        (0x0000 => pub ir: [ReadWrite<u8, InterruptRequest::Register>; 256]),
        /// DTC activation enable register
        (0x0100 => pub dtcer: [ReadWrite<u8, DtcActivationEnable::Register>; 256]),
        /// Interrupt request enable register
        (0x0200 => pub ier: [ReadWrite<u8, InterruptRequestEnable::Register>; 32]),
        (0x0220 => _pad0),
        /// Software interrupt activation register
        (0x02e0 => pub swintr: ReadWrite<u8, SoftwareInterruptActivation::Register>),
        (0x02e1 => _pad1),
        /// Fast interrupt set register
        (0x02f0 => pub fir: ReadWrite<u16, FastInterrupt::Register>),
        (0x02f2 => _pad2),
        /// Interrupt source priority register
        (0x0300 => pub ipr: [ReadWrite<u8, InterruptPriority::Register>; 256]),
        /// DMACA activation source register
        (0x0400 => pub dmrsr: [Align4<ReadWrite<u8, InterruptPriority::Register>>; 8]),
        (0x0420 => _pad3),
        /// IRQ control register
        (0x0500 => pub irqcr: [ReadWrite<u8, IrqControl::Register>; 16]),
        (0x0510 => _pad4),
        /// NMI status register
        (0x0580 => pub nmisr: ReadOnly<u8, NmiStatus::Register>),
        /// NMI enable register
        (0x0581 => pub nmier: ReadWrite<u8, NmiEnable::Register>),
        /// NMI clear register
        (0x0582 => pub nmiclr: ReadWrite<u8, NmiClear::Register>),
        /// NMI pin interrupt control register
        (0x0583 => pub nmicr: ReadWrite<u8, NmiPinInterruptControl::Register>),
        (0x0584 => @END),
    }
}

register_bitfields![u8,
    pub InterruptRequest [
        /// Interrupt status flag
        IR OFFSET(0) NUMBITS(1) [],
    ],

    pub InterruptRequestEnable [
        /// Interrupt request enable 0
        IEN0 OFFSET(0) NUMBITS(1) [],
        /// Interrupt request enable 1
        IEN1 OFFSET(1) NUMBITS(1) [],
        /// Interrupt request enable 2
        IEN2 OFFSET(2) NUMBITS(1) [],
        /// Interrupt request enable 3
        IEN3 OFFSET(3) NUMBITS(1) [],
        /// Interrupt request enable 4
        IEN4 OFFSET(4) NUMBITS(1) [],
        /// Interrupt request enable 5
        IEN5 OFFSET(5) NUMBITS(1) [],
        /// Interrupt request enable 6
        IEN6 OFFSET(6) NUMBITS(1) [],
        /// Interrupt request enable 7
        IEN7 OFFSET(7) NUMBITS(1) [],
    ],

    pub InterruptPriority [
        /// Interrupt priority level select
        IPR OFFSET(0) NUMBITS(4) [],
    ],

    pub SoftwareInterruptActivation [
        SWINT OFFSET(0) NUMBITS(1) [
            Request = 1,
        ],
    ],

    pub DtcActivationEnable [
        /// DTC activation enable
        DTCE OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
    ],

    pub IrqControl [
        /// IRQ detection sense select
        IRQMD OFFSET(2) NUMBITS(2) [
            LowLevel = 0b00,
            FallingEdge = 0b01,
            RisingEdge = 0b10,
            RisingAndFallingEdges = 0b11,
        ],
    ],

    pub NmiStatus [
        /// NMI status flag
        NMIST OFFSET(0) NUMBITS(1) [
            NotRequested = 0,
            Requested = 1,
        ],
        /// Voltage monitoring interrupt status flag
        LVDST OFFSET(1) NUMBITS(1) [
            NotRequested = 0,
            Requested = 1,
        ],
        /// Oscillation stop detection interrupt status flag
        OSTST OFFSET(2) NUMBITS(1) [
            NotRequested = 0,
            Requested = 1,
        ],
    ],

    pub NmiEnable [
        /// NMI enable
        NMIEN OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        /// Voltage monitoring interrupt enable
        LVDEN OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        /// Oscillation stop detection interrupt enable
        OSTEN OFFSET(2) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
    ],

    pub NmiClear [
        /// NMI clear
        NMICLR OFFSET(0) NUMBITS(1) [
            Clear = 1,
        ],
        /// OST clear
        OSTCLR OFFSET(2) NUMBITS(1) [
            Clear = 1,
        ],
    ],

    pub NmiPinInterruptControl [
        /// NMI detection set
        NMIMD OFFSET(3) NUMBITS(1) [
            FallingEdge = 0,
            RisingEdge = 1,
        ],
    ],
];

register_bitfields![u16,
    pub FastInterrupt [
        /// Fast interrupt vector number
        FVCT OFFSET(0) NUMBITS(8) [],
        /// Fast interrupt enable
        FIEN OFFSET(15) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
    ],
];
