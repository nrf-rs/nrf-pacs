#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: crate::Reg<tasks_resume::TASKS_RESUME_SPEC>,
    _reserved4: [u8; 0x6c],
    #[doc = "0x90 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x94 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x9c - Subscribe configuration for task SUSPEND"]
    pub subscribe_suspend: crate::Reg<subscribe_suspend::SUBSCRIBE_SUSPEND_SPEC>,
    #[doc = "0xa0 - Subscribe configuration for task RESUME"]
    pub subscribe_resume: crate::Reg<subscribe_resume::SUBSCRIBE_RESUME_SPEC>,
    _reserved8: [u8; 0x60],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: crate::Reg<events_end::EVENTS_END_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>,
    _reserved12: [u8; 0x28],
    #[doc = "0x14c - Transaction started"]
    pub events_started: crate::Reg<events_started::EVENTS_STARTED_SPEC>,
    _reserved13: [u8; 0x34],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: crate::Reg<publish_endrx::PUBLISH_ENDRX_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x198 - Publish configuration for event END"]
    pub publish_end: crate::Reg<publish_end::PUBLISH_END_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    pub publish_endtx: crate::Reg<publish_endtx::PUBLISH_ENDTX_SPEC>,
    _reserved17: [u8; 0x28],
    #[doc = "0x1cc - Publish configuration for event STARTED"]
    pub publish_started: crate::Reg<publish_started::PUBLISH_STARTED_SPEC>,
    _reserved18: [u8; 0x30],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved19: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved21: [u8; 0xf4],
    #[doc = "0x400 - Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
    pub stallstat: crate::Reg<stallstat::STALLSTAT_SPEC>,
    _reserved22: [u8; 0xfc],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x508..0x518 - Unspecified"]
    pub psel: PSEL,
    _reserved24: [u8; 0x0c],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x560..0x568 - Unspecified"]
    pub iftiming: IFTIMING,
    #[doc = "0x568 - Polarity of CSN output"]
    pub csnpol: crate::Reg<csnpol::CSNPOL_SPEC>,
    #[doc = "0x56c - Pin select for DCX signal"]
    pub pseldcx: crate::Reg<pseldcx::PSELDCX_SPEC>,
    #[doc = "0x570 - DCX configuration"]
    pub dcxcnt: crate::Reg<dcxcnt::DCXCNT_SPEC>,
    _reserved32: [u8; 0x4c],
    #[doc = "0x5c0 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
    pub orc: crate::Reg<orc::ORC_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x04 - Pin select for MOSI signal"]
    pub mosi: crate::Reg<self::psel::mosi::MOSI_SPEC>,
    #[doc = "0x08 - Pin select for MISO signal"]
    pub miso: crate::Reg<self::psel::miso::MISO_SPEC>,
    #[doc = "0x0c - Pin select for CSN"]
    pub csn: crate::Reg<self::psel::csn::CSN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: crate::Reg<self::rxd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: crate::Reg<self::rxd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::rxd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Number of bytes in transmit buffer"]
    pub maxcnt: crate::Reg<self::txd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::txd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct IFTIMING {
    #[doc = "0x00 - Sample delay for input serial data on MISO"]
    pub rxdelay: crate::Reg<self::iftiming::rxdelay::RXDELAY_SPEC>,
    #[doc = "0x04 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
    pub csndur: crate::Reg<self::iftiming::csndur::CSNDUR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod iftiming;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SUSPEND register accessor: an alias for `Reg<SUBSCRIBE_SUSPEND_SPEC>`"]
pub type SUBSCRIBE_SUSPEND = crate::Reg<subscribe_suspend::SUBSCRIBE_SUSPEND_SPEC>;
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "SUBSCRIBE_RESUME register accessor: an alias for `Reg<SUBSCRIBE_RESUME_SPEC>`"]
pub type SUBSCRIBE_RESUME = crate::Reg<subscribe_resume::SUBSCRIBE_RESUME_SPEC>;
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ENDRX register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_END register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "EVENTS_ENDTX register accessor: an alias for `Reg<EVENTS_ENDTX_SPEC>`"]
pub type EVENTS_ENDTX = crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>;
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "EVENTS_STARTED register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "PUBLISH_STOPPED register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_ENDRX register accessor: an alias for `Reg<PUBLISH_ENDRX_SPEC>`"]
pub type PUBLISH_ENDRX = crate::Reg<publish_endrx::PUBLISH_ENDRX_SPEC>;
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "PUBLISH_END register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_ENDTX register accessor: an alias for `Reg<PUBLISH_ENDTX_SPEC>`"]
pub type PUBLISH_ENDTX = crate::Reg<publish_endtx::PUBLISH_ENDTX_SPEC>;
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "PUBLISH_STARTED register accessor: an alias for `Reg<PUBLISH_STARTED_SPEC>`"]
pub type PUBLISH_STARTED = crate::Reg<publish_started::PUBLISH_STARTED_SPEC>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "STALLSTAT register accessor: an alias for `Reg<STALLSTAT_SPEC>`"]
pub type STALLSTAT = crate::Reg<stallstat::STALLSTAT_SPEC>;
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
pub mod stallstat;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "CSNPOL register accessor: an alias for `Reg<CSNPOL_SPEC>`"]
pub type CSNPOL = crate::Reg<csnpol::CSNPOL_SPEC>;
#[doc = "Polarity of CSN output"]
pub mod csnpol;
#[doc = "PSELDCX register accessor: an alias for `Reg<PSELDCX_SPEC>`"]
pub type PSELDCX = crate::Reg<pseldcx::PSELDCX_SPEC>;
#[doc = "Pin select for DCX signal"]
pub mod pseldcx;
#[doc = "DCXCNT register accessor: an alias for `Reg<DCXCNT_SPEC>`"]
pub type DCXCNT = crate::Reg<dcxcnt::DCXCNT_SPEC>;
#[doc = "DCX configuration"]
pub mod dcxcnt;
#[doc = "ORC register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub mod orc;
