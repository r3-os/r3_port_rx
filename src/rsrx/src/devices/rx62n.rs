//! RX62N/RX621 group
#![cfg(feature = "rx62n")]
#![cfg_attr(feature = "doc", doc(cfg(feature = "rx62n")))]
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
        pub PORTS: PORTS,
        pub IOPORT: IOPORT,
    }
}

zero_sized_ref!(pub struct SYSTEM: &system::Registers = 0x0008_0000);
zero_sized_ref!(pub struct ICU: &icua::Registers = 0x0008_7000);
zero_sized_ref!(pub struct CMT0_1: &cmt::Registers = 0x0008_8000);
zero_sized_ref!(pub struct CMT2_3: &cmt::Registers = 0x0008_8010);
zero_sized_ref!(pub struct SCI0: &scia::Registers = 0x0008_8240);
zero_sized_ref!(pub struct SCI1: &scia::Registers = 0x0008_8248);
zero_sized_ref!(pub struct SCI2: &scia::Registers = 0x0008_8250);
zero_sized_ref!(pub struct SCI3: &scia::Registers = 0x0008_8258);
zero_sized_ref!(pub struct SCI4: &scia::Registers = 0x0008_8260);
zero_sized_ref!(pub struct SCI5: &scia::Registers = 0x0008_8268);
zero_sized_ref!(pub struct SCI6: &scia::Registers = 0x0008_8270);
zero_sized_ref!(pub struct PORTS: &ports::Registers = 0x0008_C000);
zero_sized_ref!(pub struct IOPORT: &ioport::Registers = 0x0008_C100);

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
            (0x0024 => _pad0),
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
            /// Standby timer select
            STS OFFSET(8) NUMBITS(5) [
                WaitFor64States = 0b00101,
                WaitFor512States = 0b00110,
                WaitFor1024States = 0b00111,
                WaitFor2048States = 0b01000,
                WaitFor4096States = 0b01001,
                WaitFor16384States = 0b01010,
                WaitFor32768States = 0b01011,
                WaitFor65536States = 0b01100,
                WaitFor131072States = 0b01101,
                WaitFor262144States = 0b01110,
                WaitFor524288States = 0b01111,
            ],
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
            /// Multifunction Timer Pulse Unit (unit 1) module stop
            MSTPA8 OFFSET(8) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// Multifunction Timer Pulse Unit (unit 0) module stop
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
            /// 10-bit D/A Converter (unit 1) module stop
            MSTPA22 OFFSET(22) NUMBITS(1) [
                Run = 0,
                Stop = 1,
            ],
            /// 10-bit D/A Converter (unit 0) module stop
            MSTPA23 OFFSET(23) NUMBITS(1) [
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
            /// CAN module stop
            MSTPB0 OFFSET(0) NUMBITS(1) [
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
            /// CRC Calculator module stop
            MSTPB23 OFFSET(23) NUMBITS(1) [
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
        ],

        pub SystemClockControl [
            /// Peripheral Module Clock select
            PCK OFFSET(8) NUMBITS(4) [
                MultiplyBy8 = 0b0000,
                MultiplyBy4 = 0b0001,
                MultiplyBy2 = 0b0010,
                MultiplyBy1 = 0b0011,
            ],
            /// External Bus Clock and SDRAM clock select
            BCK OFFSET(16) NUMBITS(4) [
                MultiplyBy8 = 0b0000,
                MultiplyBy4 = 0b0001,
                MultiplyBy2 = 0b0010,
                MultiplyBy1 = 0b0011,
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
            /// System clock select
            ICLK OFFSET(24) NUMBITS(4) [
                MultiplyBy8 = 0b0000,
                MultiplyBy4 = 0b0001,
                MultiplyBy2 = 0b0010,
                MultiplyBy1 = 0b0011,
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
        Data, Direction, InputBufferControl, NmosOpenDrainControl, PullUpControl,
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
            (0x031 => _pad1),

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
            (0x051 => _pad2),

            /// PORT0 Port input buffer control
            (0x060 => pub port0_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT1 Port input buffer control
            (0x061 => pub port1_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT2 Port input buffer control
            (0x062 => pub port2_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT3 Port input buffer control
            (0x063 => pub port3_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT4 Port input buffer control
            (0x064 => pub port4_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT5 Port input buffer control
            (0x065 => pub port5_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT6 Port input buffer control
            (0x066 => pub port6_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT7 Port input buffer control
            (0x067 => pub port7_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT8 Port input buffer control
            (0x068 => pub port8_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORT9 Port input buffer control
            (0x069 => pub port9_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTA Port input buffer control
            (0x06a => pub porta_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTB Port input buffer control
            (0x06b => pub portb_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTC Port input buffer control
            (0x06c => pub portc_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTD Port input buffer control
            (0x06d => pub portd_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTE Port input buffer control
            (0x06e => pub porte_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTF Port input buffer control
            (0x06f => pub portf_icr: ReadWrite<u8, InputBufferControl::Register>),
            /// PORTG Port input buffer control
            (0x070 => pub portg_icr: ReadWrite<u8, InputBufferControl::Register>),
            (0x071 => _pad3),

            /// PORT0 open drain control register
            (0x080 => pub port0_odr: ReadWrite<u8, NmosOpenDrainControl::Register>),
            /// PORT1 open drain control register
            (0x081 => pub port1_odr: ReadWrite<u8, NmosOpenDrainControl::Register>),
            /// PORT2 open drain control register
            (0x082 => pub port2_odr: ReadWrite<u8, NmosOpenDrainControl::Register>),
            /// PORT3 open drain control register
            (0x083 => pub port3_odr: ReadWrite<u8, NmosOpenDrainControl::Register>),
            (0x084 => _pad4),
            /// PORTC open drain control register
            (0x08c => pub portc_odr: ReadWrite<u8, NmosOpenDrainControl::Register>),
            (0x08d => _pad5),

            /// PORT9 pull-up resistor control register
            (0x0c9 => pub port9_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTA pull-up resistor control register
            (0x0ca => pub porta_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTB pull-up resistor control register
            (0x0cb => pub portb_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTC pull-up resistor control register
            (0x0cc => pub portc_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTD pull-up resistor control register
            (0x0cd => pub portd_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTE pull-up resistor control register
            (0x0ce => pub porte_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTF pull-up resistor control register
            (0x0cf => pub portf_pcr: ReadWrite<u8, PullUpControl::Register>),
            /// PORTG pull-up resistor control register
            (0x0d0 => pub portg_pcr: ReadWrite<u8, PullUpControl::Register>),

            (0x0d1 => @END),
        }
    }
}

pub mod ioport {
    use super::*;

    register_structs! {
        /// The memory-mapped registers exposed by the IOPORT module.
        pub Registers {
            (0x00 => _todo),
            /// Port function control register F
            (0x0f => pub pffsci: ReadWrite<u8, PortFunctionF::Register>),
            (0x10 => @END),
        }
    }

    register_bitfields![u8,
        pub PortFunctionF [
            SCI1S OFFSET(1) NUMBITS(1) [
                /// P30 = RxD1-A, P27 = SCK1-A, P26 = TxD1-A
                A = 0,
                /// PF2 = RxD1-B, PF1 = SCK1-B, PF0 = TxD1-B
                B = 1,
            ],
            SCI2S OFFSET(2) NUMBITS(1) [
                /// P12 = RxD2-A, P11 = SCK2-A, P13 = TxD2-A
                A = 0,
                /// P52 = RxD2-B, P51 = SCK2-B, P50 = TxD2-B
                B = 1,
            ],
            SCI3S OFFSET(3) NUMBITS(1) [
                /// P16 = RxD3-A, P15 = SCK3-A, P17 = TxD3-A
                A = 0,
                /// P25 = RxD3-B, P24 = SCK3-B, P23 = TxD3-B
                B = 1,
            ],
            SCI6S OFFSET(6) NUMBITS(1) [
                /// P01 = RxD6-A, P02 = SCK6-A, P03 = TxD6-A
                A = 0,
                /// P33 = RxD6-B, P34 = SCK6-B, P32 = TxD6-B
                B = 1,
            ],
        ],
    ];
}
