//! RX63N/RX631 group
#![cfg(feature = "rx63n")]
#![cfg_attr(feature = "doc", doc(cfg(feature = "rx63n")))]
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{cmt, icua, scia};

peripheral_set! {
    /// All the peripherals of RX62N
    pub struct Peripherals {
        pub SYSTEM: SYSTEM,
        pub ICU: ICU,
        pub CMT0_1: CMT0_1,
        pub CMT2_3: CMT2_3,
        pub SCI0: SCI0,
        pub SCI1: SCI1,
        pub SCI2: SCI2,
        pub SCI3: SCI3,
        pub SCI4: SCI4,
        pub SCI5: SCI5,
        pub SCI6: SCI6,
        pub SCI7: SCI7,
        pub SCI8: SCI8,
        pub SCI9: SCI9,
        pub SCI10: SCI10,
        pub SCI11: SCI11,
        pub SCI12: SCI12,
        pub PORTS: PORTS,
    }
}

zero_sized_ref!(pub struct SYSTEM: &system::Registers = 0x0008_0000);
zero_sized_ref!(pub struct ICU: &icua::Registers = 0x0008_7000); // TODO: Actually it's ICUb
zero_sized_ref!(pub struct CMT0_1: &cmt::Registers = 0x0008_8000);
zero_sized_ref!(pub struct CMT2_3: &cmt::Registers = 0x0008_8010);
zero_sized_ref!(pub struct SCI0: &scia::Registers = 0x0008_a000); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI1: &scia::Registers = 0x0008_a020); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI2: &scia::Registers = 0x0008_a040); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI3: &scia::Registers = 0x0008_a060); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI4: &scia::Registers = 0x0008_a080); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI5: &scia::Registers = 0x0008_a0a0); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI6: &scia::Registers = 0x0008_a0c0); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI7: &scia::Registers = 0x0008_a0e0); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI8: &scia::Registers = 0x0008_a100); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI9: &scia::Registers = 0x0008_a120); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI10: &scia::Registers = 0x0008_a140); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI11: &scia::Registers = 0x0008_a160); // TOOD: Actually it's SCIc
zero_sized_ref!(pub struct SCI12: &scia::Registers = 0x0008_b300); // TOOD: Actually it's SCId
zero_sized_ref!(pub struct PORTS: &ports::Registers = 0x0008_C000);

pub mod system {
    use super::*;

    register_structs! {
        /// The memory-mapped registers exposed by the SYSTEM module.
        pub Registers {
            (0x0000 => _pad3),
            (0x000c => pub sbycr: ReadWrite<u32, StandbyControl::Register>),
            (0x0010 => pub mstpcra: ReadWrite<u32, ModuleStopControlA::Register>),
            (0x0014 => pub mstpcrb: ReadWrite<u32, ModuleStopControlB::Register>),
            (0x0018 => pub mstpcrc: ReadWrite<u32, ModuleStopControlC::Register>),
            (0x001c => _pad2),
            /// System clock control register
            (0x0020 => pub sckcr: ReadWrite<u32, SystemClockControl::Register>),
            (0x0024 => pub sckcr2: ReadWrite<u16, SystemClockControl2::Register>),
            (0x0026 => pub sckcr3: ReadWrite<u16, SystemClockControl3::Register>),
            (0x0028 => _pad0),
            /// External bus clock control register
            (0x0030 => pub bckcr: ReadWrite<u8, ExternalBusClockControl::Register>),
            (0x0031 => _pad1),
            /// Oscillation stop detection control register
            (0x0040 => pub ostdcr: ReadWrite<u16, OscillationStopDetectionControl::Register>),
            (0x0042 => _pad4),
            (0x0044 => @END),
        }
    }

    register_bitfields![u32,
        pub StandbyControl [
            /// Output port enable during software standby mode or deep software
            /// standby mode
            OPE OFFSET(14) NUMBITS(1) [
                HiZ = 0,
                Output = 1,
            ],
            /// Software standby
            SSBY OFFSET(15) NUMBITS(1) [
                /// Shifts to sleep mode or all-module clock stop mode after the
                /// WAIT instruction is executed
                SleepOrAllModuleClockStopMode = 0,
                /// Shifts to software standby mode after the WAIT instruction is
                /// executed
                SoftwareStandbyMode = 1,
            ],
        ],

        pub ModuleStopControlA [
            /// 8-bit Timer 3/2 (unit 1) module stop
            MSTPA4 OFFSET(4) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 8-bit Timer 1/0 (unit 0) module stop
            MSTPA5 OFFSET(5) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Multifunction Timer Pulse Unit (unit 2) module stop
            MSTPA9 OFFSET(9) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Programmable Pulse Generator (unit 1) module stop
            MSTPA10 OFFSET(10) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Programmable Pulse Generator (unit 0) module stop
            MSTPA11 OFFSET(11) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 16-bit Timer Pulse Unit (unit 1) module stop
            MSTPA12 OFFSET(12) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 16-bit Timer Pulse Unit (unit 0) module stop
            MSTPA13 OFFSET(13) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Compare Match Timer (unit 1) module stop
            MSTPA14 OFFSET(14) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Compare Match Timer (unit 0) module stop
            MSTPA15 OFFSET(15) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 12-bit A/D Converter module stop
            MSTPA17 OFFSET(17) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// D/A Converter module stop
            MSTPA19 OFFSET(19) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 10-bit D/A Converter module stop
            MSTPA23 OFFSET(23) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Set when engaging all-module clock stop.
            MSTPA24 OFFSET(24) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Set when engaging all-module clock stop.
            MSTPA27 OFFSET(27) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// DMA Controller/Data Transfer Controller module stop
            MSTPA28 OFFSET(28) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// EXDMA Controller module stop
            MSTPA29 OFFSET(29) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// All-module clock stop mode enable
            ACSE OFFSET(31) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
        ],

        pub ModuleStopControlB [
            /// CAN module 0 stop
            MSTPB0 OFFSET(0) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// CAN module 1 stop
            MSTPB1 OFFSET(1) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// CAN module 2 stop
            MSTPB2 OFFSET(2) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 12 module stop
            MSTPB4 OFFSET(4) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Temperature Sensor Module stop
            MSTPB8 OFFSET(8) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Ethernet Controller DMAC module stop
            MSTPB15 OFFSET(15) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 1 module stop
            MSTPB16 OFFSET(16) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 0 module stop
            MSTPB17 OFFSET(17) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Universal Serial Bus Interface (port 1) module stop
            MSTPB18 OFFSET(18) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Universal Serial Bus Interface (port 0) module stop
            MSTPB19 OFFSET(19) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// I²C Bus Interface 1 module stop
            MSTPB20 OFFSET(20) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// I²C Bus Interface 0 module stop
            MSTPB21 OFFSET(21) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Parallel Data Capture Unit stop
            MSTPB22 OFFSET(22) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// CRC Calculator module stop
            MSTPB23 OFFSET(23) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 7 module stop
            MSTPB24 OFFSET(24) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 6 module stop
            MSTPB25 OFFSET(25) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 5 module stop
            MSTPB26 OFFSET(26) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 3 module stop
            MSTPB28 OFFSET(28) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 2 module stop
            MSTPB29 OFFSET(29) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 1 module stop
            MSTPB30 OFFSET(30) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Communication Interface 0 module stop
            MSTPB31 OFFSET(31) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
        ],

        pub ModuleStopControlC [
            /// RAM0 (`0x0000_0000..=0x0000_ffff`) module stop
            MSTPC0 OFFSET(0) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// RAM1 (`0x0001_0000..=0x0001_ffff`) module stop
            MSTPC1 OFFSET(1) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// I²C Bus Interface 3 module stop
            MSTPC16 OFFSET(16) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// I²C Bus Interface 2 module stop
            MSTPC17 OFFSET(17) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// IEBUS module stop
            MSTPC18 OFFSET(18) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Frequency Measurement Circuit module stop
            MSTPC19 OFFSET(19) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 2 module stop
            MSTPC22 OFFSET(22) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 11 module stop
            MSTPC24 OFFSET(24) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 10 module stop
            MSTPC25 OFFSET(25) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 9 module stop
            MSTPC26 OFFSET(26) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Serial Peripheral Interface 8 module stop
            MSTPC27 OFFSET(27) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
        ],

        pub SystemClockControl [
            /// Peripheral Module Clock B select
            PCKB OFFSET(8) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
            /// Peripheral Module Clock A select
            PCKA OFFSET(12) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
            /// External Bus Clock and SDRAM clock select
            BCK OFFSET(16) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
            /// SDCLK pin output control
            PSTOP0 OFFSET(22) NUMBITS(1) [
                EnableOutput = 0,
                PullUp = 1,
            ],
            /// BCLK pin output control
            PSTOP1 OFFSET(23) NUMBITS(1) [
                EnableOutput = 0,
                PullUp = 1,
            ],
            /// System Clock (ICLK) select
            ICK OFFSET(24) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
            /// FlashIF Clock (FCLK) select
            FCK OFFSET(28) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
        ],
    ];

    register_bitfields![u16,
        pub SystemClockControl2 [
            /// IEBUS Clock (IECLK) select
            IEBCK OFFSET(0) NUMBITS(4) [
                DivideBy1 = 0b0000,
                DivideBy2 = 0b0001,
                DivideBy4 = 0b0010,
                DivideBy8 = 0b0011,
                DivideBy16 = 0b0100,
                DivideBy32 = 0b0101,
                DivideBy64 = 0b0110,
            ],
            /// USB Clock (UCLK) select
            UCK OFFSET(4) NUMBITS(4) [
                DivideBy3 = 0b0010,
                DivideBy4 = 0b0011,
            ],
        ],

        pub SystemClockControl3 [
            /// System Clock (ICLK) source select
            CKSEL OFFSET(0) NUMBITS(3) [
                Loco = 0b000,
                Hoco = 0b001,
                MainClockOscillator = 0b010,
                SubClockOscillator = 0b011,
                Pll = 0b100,
            ],
        ],
    ];

    register_bitfields![u8,
        pub ExternalBusClockControl [
            /// BCLK pin output select
            BCLKDIV OFFSET(1) NUMBITS(1) [
                DivideBy1 = 0,
                DivideBy2 = 1,
            ],
        ],
    ];

    register_bitfields![u16,
        pub OscillationStopDetectionControl [
            /// Oscillation stop detection flag (read-only)
            OSTDF OFFSET(6) NUMBITS(1) [
                Ok = 0,
                Stop = 1,
            ],
            /// Oscillation stop detection function enable
            OSTDE OFFSET(7) NUMBITS(1) [
                Disable = 0,
                Enable = 1,
            ],
            /// OSTDCR key code
            KEY OFFSET(8) NUMBITS(8) [
                EnableWrite = 0xac,
            ],
        ],
    ];

    register_bitfields![u8,
        pub SubClockOscillatorControl [
            /// Sub-clock oscillator control
            SUBSTOP OFFSET(0) NUMBITS(1) [
                EnableClock = 0,
                DisableClock = 1,
            ],
        ],
    ];
}

pub mod ports {
    use super::*;
    pub use crate::ports::{
        Data, Direction, DriveCapacityControl, OpenDrainControl, PullUpControl, RouteToPeripheral,
    };

    register_structs! {
        /// The memory-mapped registers exposed by I/O Ports module.
        pub Registers {
            /// PORT0 Port direction register
            (0x000 => pub port0_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT1 Port direction register
            (0x001 => pub port1_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT2 Port direction register
            (0x002 => pub port2_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT3 Port direction register
            (0x003 => pub port3_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT4 Port direction register
            (0x004 => pub port4_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT5 Port direction register
            (0x005 => pub port5_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT6 Port direction register
            (0x006 => pub port6_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT7 Port direction register
            (0x007 => pub port7_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT8 Port direction register
            (0x008 => pub port8_pdr: ReadWrite<u8, Direction::Register>),
            /// PORT9 Port direction register
            (0x009 => pub port9_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTA Port direction register
            (0x00a => pub porta_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTB Port direction register
            (0x00b => pub portb_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTC Port direction register
            (0x00c => pub portc_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTD Port direction register
            (0x00d => pub portd_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTE Port direction register
            (0x00e => pub porte_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTF Port direction register
            (0x00f => pub portf_pdr: ReadWrite<u8, Direction::Register>),
            /// PORTG Port direction register
            (0x010 => pub portg_pdr: ReadWrite<u8, Direction::Register>),
            (0x011 => _pad0),
            /// PORTJ Port direction register
            (0x012 => pub portj_pdr: ReadWrite<u8, Direction::Register>),
            (0x013 => _pad1),

            /// PORT0 Port output data register
            (0x020 => pub port0_podr: ReadWrite<u8, Data::Register>),
            /// PORT1 Port output data register
            (0x021 => pub port1_podr: ReadWrite<u8, Data::Register>),
            /// PORT2 Port output data register
            (0x022 => pub port2_podr: ReadWrite<u8, Data::Register>),
            /// PORT3 Port output data register
            (0x023 => pub port3_podr: ReadWrite<u8, Data::Register>),
            /// PORT4 Port output data register
            (0x024 => pub port4_podr: ReadWrite<u8, Data::Register>),
            /// PORT5 Port output data register
            (0x025 => pub port5_podr: ReadWrite<u8, Data::Register>),
            /// PORT6 Port output data register
            (0x026 => pub port6_podr: ReadWrite<u8, Data::Register>),
            /// PORT7 Port output data register
            (0x027 => pub port7_podr: ReadWrite<u8, Data::Register>),
            /// PORT8 Port output data register
            (0x028 => pub port8_podr: ReadWrite<u8, Data::Register>),
            /// PORT9 Port output data register
            (0x029 => pub port9_podr: ReadWrite<u8, Data::Register>),
            /// PORTA Port output data register
            (0x02a => pub porta_podr: ReadWrite<u8, Data::Register>),
            /// PORTB Port output data register
            (0x02b => pub portb_podr: ReadWrite<u8, Data::Register>),
            /// PORTC Port output data register
            (0x02c => pub portc_podr: ReadWrite<u8, Data::Register>),
            /// PORTD Port output data register
            (0x02d => pub portd_podr: ReadWrite<u8, Data::Register>),
            /// PORTE Port output data register
            (0x02e => pub porte_podr: ReadWrite<u8, Data::Register>),
            /// PORTF Port output data register
            (0x02f => pub portf_podr: ReadWrite<u8, Data::Register>),
            /// PORTG Port output data register
            (0x030 => pub portg_podr: ReadWrite<u8, Data::Register>),
            (0x031 => _pad2),
            /// PORTJ Port output data register
            (0x032 => pub portj_podr: ReadWrite<u8, Data::Register>),
            (0x033 => _pad3),

            /// PORT0 Port input data register
            (0x040 => pub port0_pidr: ReadOnly<u8, Data::Register>),
            /// PORT1 Port input data register
            (0x041 => pub port1_pidr: ReadOnly<u8, Data::Register>),
            /// PORT2 Port input data register
            (0x042 => pub port2_pidr: ReadOnly<u8, Data::Register>),
            /// PORT3 Port input data register
            (0x043 => pub port3_pidr: ReadOnly<u8, Data::Register>),
            /// PORT4 Port input data register
            (0x044 => pub port4_pidr: ReadOnly<u8, Data::Register>),
            /// PORT5 Port input data register
            (0x045 => pub port5_pidr: ReadOnly<u8, Data::Register>),
            /// PORT6 Port input data register
            (0x046 => pub port6_pidr: ReadOnly<u8, Data::Register>),
            /// PORT7 Port input data register
            (0x047 => pub port7_pidr: ReadOnly<u8, Data::Register>),
            /// PORT8 Port input data register
            (0x048 => pub port8_pidr: ReadOnly<u8, Data::Register>),
            /// PORT9 Port input data register
            (0x049 => pub port9_pidr: ReadOnly<u8, Data::Register>),
            /// PORTA Port input data register
            (0x04a => pub porta_pidr: ReadOnly<u8, Data::Register>),
            /// PORTB Port input data register
            (0x04b => pub portb_pidr: ReadOnly<u8, Data::Register>),
            /// PORTC Port input data register
            (0x04c => pub portc_pidr: ReadOnly<u8, Data::Register>),
            /// PORTD Port input data register
            (0x04d => pub portd_pidr: ReadOnly<u8, Data::Register>),
            /// PORTE Port input data register
            (0x04e => pub porte_pidr: ReadOnly<u8, Data::Register>),
            /// PORTF Port input data register
            (0x04f => pub portf_pidr: ReadOnly<u8, Data::Register>),
            /// PORTG Port input data register
            (0x050 => pub portg_pidr: ReadOnly<u8, Data::Register>),
            (0x051 => _pad4),
            /// PORTJ Port input data register
            (0x052 => pub portj_pidr: ReadOnly<u8, Data::Register>),
            (0x053 => _pad5),

            /// PORT0 Port mode register
            (0x060 => pub port0_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT1 Port mode register
            (0x061 => pub port1_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT2 Port mode register
            (0x062 => pub port2_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT3 Port mode register
            (0x063 => pub port3_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT4 Port mode register
            (0x064 => pub port4_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT5 Port mode register
            (0x065 => pub port5_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT6 Port mode register
            (0x066 => pub port6_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT7 Port mode register
            (0x067 => pub port7_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT8 Port mode register
            (0x068 => pub port8_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORT9 Port mode register
            (0x069 => pub port9_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTA Port mode register
            (0x06a => pub porta_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTB Port mode register
            (0x06b => pub portb_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTC Port mode register
            (0x06c => pub portc_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTD Port mode register
            (0x06d => pub portd_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTE Port mode register
            (0x06e => pub porte_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTF Port mode register
            (0x06f => pub portf_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            /// PORTG Port mode register
            (0x070 => pub portg_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            (0x071 => _pad6),
            /// PORTJ Port mode register
            (0x072 => pub portj_pmr: ReadWrite<u8, RouteToPeripheral::Register>),
            (0x073 => _pad7),

            /// PORT0 Open drain control register
            (0x080 => pub port0_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT1 Open drain control register
            (0x082 => pub port1_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT2 Open drain control register
            (0x084 => pub port2_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT3 Open drain control register
            (0x086 => pub port3_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT4 Open drain control register
            (0x088 => pub port4_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT5 Open drain control register
            (0x08a => pub port5_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT6 Open drain control register
            (0x08c => pub port6_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT7 Open drain control register
            (0x08e => pub port7_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT8 Open drain control register
            (0x090 => pub port8_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORT9 Open drain control register
            (0x092 => pub port9_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTA Open drain control register
            (0x094 => pub porta_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTB Open drain control register
            (0x096 => pub portb_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTC Open drain control register
            (0x098 => pub portc_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTD Open drain control register
            (0x09a => pub portd_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTE Open drain control register
            (0x09c => pub porte_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTF Open drain control register
            (0x09e => pub portf_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            /// PORTG Open drain control register
            (0x0a0 => pub portg_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            (0x0a2 => _pad8),
            /// PORTJ Open drain control register
            (0x0a4 => pub portj_odr: [ReadWrite<u8, OpenDrainControl::Register>; 2]),
            (0x0a6 => _pad9),

            /// PORT0 Pull-up resistor control register
            (0x0c0 => pub port0_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT1 Pull-up resistor control register
            (0x0c1 => pub port1_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT2 Pull-up resistor control register
            (0x0c2 => pub port2_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT3 Pull-up resistor control register
            (0x0c3 => pub port3_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT4 Pull-up resistor control register
            (0x0c4 => pub port4_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT5 Pull-up resistor control register
            (0x0c5 => pub port5_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT6 Pull-up resistor control register
            (0x0c6 => pub port6_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT7 Pull-up resistor control register
            (0x0c7 => pub port7_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT8 Pull-up resistor control register
            (0x0c8 => pub port8_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORT9 Pull-up resistor control register
            (0x0c9 => pub port9_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTA Pull-up resistor control register
            (0x0ca => pub porta_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTB Pull-up resistor control register
            (0x0cb => pub portb_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTC Pull-up resistor control register
            (0x0cc => pub portc_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTD Pull-up resistor control register
            (0x0cd => pub portd_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTE Pull-up resistor control register
            (0x0ce => pub porte_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTF Pull-up resistor control register
            (0x0cf => pub portf_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTG Pull-up resistor control register
            (0x0d0 => pub portg_pcr: ReadWrite<u8, PullUpControl::Register>),
            (0x0d1 => _pad10),
            /// PORTJ Pull-up resistor control register
            (0x0d2 => pub portj_pcr: ReadWrite<u8, PullUpControl::Register>),
            (0x0d3 => _pad11),

            /// PORT0 Drive capacity control register
            (0x0e0 => pub port0_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            (0x0e1 => _pad12),
            /// PORT2 Drive capacity control register
            (0x0e2 => pub port2_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            (0x0e3 => _pad13),
            /// PORT5 Drive capacity control register
            (0x0e5 => pub port5_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            (0x0e6 => _pad14),
            /// PORT9 Drive capacity control register
            (0x0e9 => pub port9_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            /// PORTA Drive capacity control register
            (0x0ea => pub porta_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            /// PORTB Drive capacity control register
            (0x0eb => pub portb_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            /// PORTC Drive capacity control register
            (0x0ec => pub portc_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            /// PORTD Drive capacity control register
            (0x0ed => pub portd_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            /// PORTE Drive capacity control register
            (0x0ee => pub porte_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            (0x0ef => _pad15),
            /// PORTG Drive capacity control register
            (0x0f0 => pub portg_dscr: ReadWrite<u8, DriveCapacityControl::Register>),
            (0x0f1 => _pad16),

            /// Port switching register B (48-pin packages only)
            (0x120 => pub psrb: ReadWrite<u8, PortSwitchingB::Register>),
            /// Port switching register A (64-pin packages only)
            (0x121 => pub psra: ReadWrite<u8, PortSwitchingA::Register>),

            (0x122 => @END),
        }
    }

    register_bitfields![u8,
        pub PortSwitchingA [
            PSEL6 OFFSET(6) NUMBITS(1) [
                /// Designate PB6 as a general I/O pin
                PB6 = 0,
                /// Designate PC0 as a general I/O pin
                PC0 = 1,
            ],
            PSEL7 OFFSET(7) NUMBITS(1) [
                /// Desginate PB7 as a general I/O pin
                PB7 = 0,
                /// Desginate PC1 as a general I/O pin
                PC1 = 1,
            ],
        ],
    ];

    register_bitfields![u8,
        pub PortSwitchingB [
            PSEL0 OFFSET(0) NUMBITS(1) [
                /// Designate PB0 as a general I/O pin
                PB0 = 0,
                /// Designate PC0 as a general I/O pin
                PC0 = 1,
            ],
            PSEL1 OFFSET(1) NUMBITS(1) [
                /// Designate PB1 as a general I/O pin
                PB1 = 0,
                /// Designate PC1 as a general I/O pin
                PC1 = 1,
            ],
            PSEL3 OFFSET(3) NUMBITS(1) [
                /// Desginate PB3 as a general I/O pin
                PB3 = 0,
                /// Desginate PC1 as a general I/O pin
                PC1 = 1,
            ],
            PSEL5 OFFSET(5) NUMBITS(1) [
                /// Desginate PB5 as a general I/O pin
                PB5 = 0,
                /// Desginate PC3 as a general I/O pin
                PC3 = 1,
            ],
        ],
    ];
}
