#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: crate::Reg<tasks_resume::TASKS_RESUME_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    pub tasks_preparerx: crate::Reg<tasks_preparerx::TASKS_PREPARERX_SPEC>,
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    pub tasks_preparetx: crate::Reg<tasks_preparetx::TASKS_PREPARETX_SPEC>,
    _reserved5: [u8; 0xcc],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved6: [u8; 0x1c],
    #[doc = "0x124 - TWI error"]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved7: [u8; 0x24],
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x164 - Write command received"]
    pub events_write: crate::Reg<events_write::EVENTS_WRITE_SPEC>,
    #[doc = "0x168 - Read command received"]
    pub events_read: crate::Reg<events_read::EVENTS_READ_SPEC>,
    _reserved11: [u8; 0x94],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved12: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved15: [u8; 0x01c4],
    #[doc = "0x4d0 - Error source"]
    pub errorsrc: crate::Reg<errorsrc::ERRORSRC_SPEC>,
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    pub match_: crate::Reg<match_::MATCH_SPEC>,
    _reserved17: [u8; 0x28],
    #[doc = "0x500 - Enable TWIS"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x508..0x510 - Unspecified"]
    pub psel: PSEL,
    _reserved19: [u8; 0x24],
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved21: [u8; 0x34],
    #[doc = "0x588..0x590 - Description collection: TWI slave address n"]
    pub address: [crate::Reg<address::ADDRESS_SPEC>; 2],
    _reserved22: [u8; 0x04],
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved23: [u8; 0x28],
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    pub orc: crate::Reg<orc::ORC_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL signal"]
    pub scl: crate::Reg<self::psel::scl::SCL_SPEC>,
    #[doc = "0x04 - Pin select for SDA signal"]
    pub sda: crate::Reg<self::psel::sda::SDA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD Data pointer"]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of bytes in RXD buffer"]
    pub maxcnt: crate::Reg<self::rxd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transferred in the last RXD transaction"]
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
    #[doc = "0x00 - TXD Data pointer"]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    pub maxcnt: crate::Reg<self::txd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::txd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "TASKS_PREPARERX register accessor: an alias for `Reg<TASKS_PREPARERX_SPEC>`"]
pub type TASKS_PREPARERX = crate::Reg<tasks_preparerx::TASKS_PREPARERX_SPEC>;
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "TASKS_PREPARETX register accessor: an alias for `Reg<TASKS_PREPARETX_SPEC>`"]
pub type TASKS_PREPARETX = crate::Reg<tasks_preparetx::TASKS_PREPARETX_SPEC>;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_RXSTARTED register accessor: an alias for `Reg<EVENTS_RXSTARTED_SPEC>`"]
pub type EVENTS_RXSTARTED = crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "EVENTS_TXSTARTED register accessor: an alias for `Reg<EVENTS_TXSTARTED_SPEC>`"]
pub type EVENTS_TXSTARTED = crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "EVENTS_WRITE register accessor: an alias for `Reg<EVENTS_WRITE_SPEC>`"]
pub type EVENTS_WRITE = crate::Reg<events_write::EVENTS_WRITE_SPEC>;
#[doc = "Write command received"]
pub mod events_write;
#[doc = "EVENTS_READ register accessor: an alias for `Reg<EVENTS_READ_SPEC>`"]
pub type EVENTS_READ = crate::Reg<events_read::EVENTS_READ_SPEC>;
#[doc = "Read command received"]
pub mod events_read;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ERRORSRC register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "MATCH register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "ADDRESS register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Description collection: TWI slave address n"]
pub mod address;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "ORC register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
