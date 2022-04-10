//! Serial Communications Interface (SCIa)
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    /// The memory-mapped registers exposed by Serial Communications Interface.
    pub Registers {
        /// Serial mode register
        (0x0 => pub smr: ReadWrite<u8, Mode::Register>),
        /// Bit rate register
        (0x1 => pub brr: ReadWrite<u8>),
        /// Serial control register
        (0x2 => pub scr: ReadWrite<u8, Control::Register>),
        /// Transmit data register
        (0x3 => pub tdr: ReadWrite<u8>),
        /// Serial status register
        (0x4 => pub ssr: ReadWrite<u8, Status::Register>),
        /// Receive data register
        (0x5 => pub rdr: ReadOnly<u8>),
        /// Smart card mode register
        (0x6 => pub scmr: ReadWrite<u8, SmartCardMode::Register>),
        /// Serial extended mode register
        (0x7 => pub semr: ReadWrite<u8>),
        (0x8 => @END),
    }
}

register_bitfields![u8,
    pub Mode [
        /// Clock select
        CKS OFFSET(0) NUMBITS(2) [
            Pclk = 0,
            PclkDividedBy4 = 1,
            PclkDividedBy16 = 2,
            PclkDividedBy64 = 3,
        ],
        /// Multi-processor mode
        MP OFFSET(2) NUMBITS(1) [],
        /// Stop bit length
        STOP OFFSET(3) NUMBITS(1) [
            OneStopBit = 0,
            TwoStopBits = 1,
        ],
        /// Parity mode
        PM OFFSET(4) NUMBITS(1) [
            EvenParity = 0,
            OddParity = 1,
        ],
        /// Parity enable
        PE OFFSET(5) NUMBITS(1) [
            NoParity = 0,
            AddParity = 1,
        ],
        /// Character length
        CHR OFFSET(6) NUMBITS(1) [
            EightBits = 0,
            SevenBits = 1,
        ],
        /// Communications mode
        CM OFFSET(7) NUMBITS(1) [
            Asynchronous = 0,
            ClockSynchronous = 1,
        ],
    ],
    pub Control [
        /// Clock enable
        CKE OFFSET(0) NUMBITS(2) [],
        /// Transmit end interrupt enable
        TEIE OFFSET(2) NUMBITS(1) [],
        /// Multi-processor interrupt enable
        MPIE OFFSET(2) NUMBITS(1) [],
        /// Receive enable
        RE OFFSET(2) NUMBITS(1) [],
        /// Transmit enable
        TE OFFSET(2) NUMBITS(1) [],
        /// Receive interrupt enable
        RIE OFFSET(2) NUMBITS(1) [],
        /// Transmit interrupt enable
        TIE OFFSET(2) NUMBITS(1) [],
    ],
    pub Status [
        /// Multi-processor bit transfer
        MPBT OFFSET(0) NUMBITS(1) [],
        /// Multi-processor
        MPB OFFSET(1) NUMBITS(1) [],
        /// Transmit end flag
        TEND OFFSET(2) NUMBITS(1) [],
        /// Parity error flag
        PER OFFSET(3) NUMBITS(1) [],
        /// Framing error flag
        FER OFFSET(4) NUMBITS(1) [],
        /// Overrun error flag
        ORER OFFSET(5) NUMBITS(1) [],
        /// Receive data full flag
        RDRF OFFSET(6) NUMBITS(1) [],
        /// Transmit data empty flag
        TDRE OFFSET(7) NUMBITS(1) [],
    ],
    pub SmartCardMode [
        /// Smart card interface mode select
        SMIF OFFSET(0) NUMBITS(1) [
            SerialCommunications = 0,
            SmartCard = 1,
        ],
        SINV OFFSET(2) NUMBITS(1) [],
        SDIR OFFSET(3) NUMBITS(1) [
            LsbFirst = 0,
            MsbFirst = 1,
        ],
        BCP2 OFFSET(7) NUMBITS(1) [],
    ],
    pub Semr [
        /// Asynchronous mode clock source select
        ACS0 OFFSET(0) NUMBITS(1) [],
        /// Asynchronous mode base clock select
        ABCS OFFSET(4) NUMBITS(1) [],
    ],
];
