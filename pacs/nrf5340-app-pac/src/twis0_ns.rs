#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 0x04],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    pub tasks_preparerx: TASKS_PREPARERX,
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    pub tasks_preparetx: TASKS_PREPARETX,
    _reserved5: [u8; 0x5c],
    #[doc = "0x94 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved6: [u8; 0x04],
    #[doc = "0x9c - Subscribe configuration for task SUSPEND"]
    pub subscribe_suspend: SUBSCRIBE_SUSPEND,
    #[doc = "0xa0 - Subscribe configuration for task RESUME"]
    pub subscribe_resume: SUBSCRIBE_RESUME,
    _reserved8: [u8; 0x0c],
    #[doc = "0xb0 - Subscribe configuration for task PREPARERX"]
    pub subscribe_preparerx: SUBSCRIBE_PREPARERX,
    #[doc = "0xb4 - Subscribe configuration for task PREPARETX"]
    pub subscribe_preparetx: SUBSCRIBE_PREPARETX,
    _reserved10: [u8; 0x4c],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved11: [u8; 0x1c],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved12: [u8; 0x24],
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved14: [u8; 0x10],
    #[doc = "0x164 - Write command received"]
    pub events_write: EVENTS_WRITE,
    #[doc = "0x168 - Read command received"]
    pub events_read: EVENTS_READ,
    _reserved16: [u8; 0x18],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    _reserved17: [u8; 0x1c],
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved18: [u8; 0x24],
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    pub publish_rxstarted: PUBLISH_RXSTARTED,
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    pub publish_txstarted: PUBLISH_TXSTARTED,
    _reserved20: [u8; 0x10],
    #[doc = "0x1e4 - Publish configuration for event WRITE"]
    pub publish_write: PUBLISH_WRITE,
    #[doc = "0x1e8 - Publish configuration for event READ"]
    pub publish_read: PUBLISH_READ,
    _reserved22: [u8; 0x14],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved23: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved26: [u8; 0x01c4],
    #[doc = "0x4d0 - Error source"]
    pub errorsrc: ERRORSRC,
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    pub match_: MATCH,
    _reserved28: [u8; 0x28],
    #[doc = "0x500 - Enable TWIS"]
    pub enable: ENABLE,
    _reserved29: [u8; 0x04],
    #[doc = "0x508..0x510 - Unspecified"]
    pub psel: PSEL,
    _reserved30: [u8; 0x24],
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved32: [u8; 0x34],
    #[doc = "0x588..0x590 - Description collection: TWI slave address n"]
    pub address: [ADDRESS; 2],
    _reserved33: [u8; 0x04],
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    pub config: CONFIG,
    _reserved34: [u8; 0x28],
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    pub orc: ORC,
}
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "TASKS_PREPARERX (w) register accessor: an alias for `Reg<TASKS_PREPARERX_SPEC>`"]
pub type TASKS_PREPARERX = crate::Reg<tasks_preparerx::TASKS_PREPARERX_SPEC>;
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "TASKS_PREPARETX (w) register accessor: an alias for `Reg<TASKS_PREPARETX_SPEC>`"]
pub type TASKS_PREPARETX = crate::Reg<tasks_preparetx::TASKS_PREPARETX_SPEC>;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SUSPEND (rw) register accessor: an alias for `Reg<SUBSCRIBE_SUSPEND_SPEC>`"]
pub type SUBSCRIBE_SUSPEND = crate::Reg<subscribe_suspend::SUBSCRIBE_SUSPEND_SPEC>;
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "SUBSCRIBE_RESUME (rw) register accessor: an alias for `Reg<SUBSCRIBE_RESUME_SPEC>`"]
pub type SUBSCRIBE_RESUME = crate::Reg<subscribe_resume::SUBSCRIBE_RESUME_SPEC>;
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "SUBSCRIBE_PREPARERX (rw) register accessor: an alias for `Reg<SUBSCRIBE_PREPARERX_SPEC>`"]
pub type SUBSCRIBE_PREPARERX = crate::Reg<subscribe_preparerx::SUBSCRIBE_PREPARERX_SPEC>;
#[doc = "Subscribe configuration for task PREPARERX"]
pub mod subscribe_preparerx;
#[doc = "SUBSCRIBE_PREPARETX (rw) register accessor: an alias for `Reg<SUBSCRIBE_PREPARETX_SPEC>`"]
pub type SUBSCRIBE_PREPARETX = crate::Reg<subscribe_preparetx::SUBSCRIBE_PREPARETX_SPEC>;
#[doc = "Subscribe configuration for task PREPARETX"]
pub mod subscribe_preparetx;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_RXSTARTED (rw) register accessor: an alias for `Reg<EVENTS_RXSTARTED_SPEC>`"]
pub type EVENTS_RXSTARTED = crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "EVENTS_TXSTARTED (rw) register accessor: an alias for `Reg<EVENTS_TXSTARTED_SPEC>`"]
pub type EVENTS_TXSTARTED = crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "EVENTS_WRITE (rw) register accessor: an alias for `Reg<EVENTS_WRITE_SPEC>`"]
pub type EVENTS_WRITE = crate::Reg<events_write::EVENTS_WRITE_SPEC>;
#[doc = "Write command received"]
pub mod events_write;
#[doc = "EVENTS_READ (rw) register accessor: an alias for `Reg<EVENTS_READ_SPEC>`"]
pub type EVENTS_READ = crate::Reg<events_read::EVENTS_READ_SPEC>;
#[doc = "Read command received"]
pub mod events_read;
#[doc = "PUBLISH_STOPPED (rw) register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_ERROR (rw) register accessor: an alias for `Reg<PUBLISH_ERROR_SPEC>`"]
pub type PUBLISH_ERROR = crate::Reg<publish_error::PUBLISH_ERROR_SPEC>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_RXSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_RXSTARTED_SPEC>`"]
pub type PUBLISH_RXSTARTED = crate::Reg<publish_rxstarted::PUBLISH_RXSTARTED_SPEC>;
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "PUBLISH_TXSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_TXSTARTED_SPEC>`"]
pub type PUBLISH_TXSTARTED = crate::Reg<publish_txstarted::PUBLISH_TXSTARTED_SPEC>;
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "PUBLISH_WRITE (rw) register accessor: an alias for `Reg<PUBLISH_WRITE_SPEC>`"]
pub type PUBLISH_WRITE = crate::Reg<publish_write::PUBLISH_WRITE_SPEC>;
#[doc = "Publish configuration for event WRITE"]
pub mod publish_write;
#[doc = "PUBLISH_READ (rw) register accessor: an alias for `Reg<PUBLISH_READ_SPEC>`"]
pub type PUBLISH_READ = crate::Reg<publish_read::PUBLISH_READ_SPEC>;
#[doc = "Publish configuration for event READ"]
pub mod publish_read;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ERRORSRC (rw) register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "MATCH (r) register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "RXD EasyDMA channel"]
pub use rxd::RXD;
#[doc = r"Cluster"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = "TXD EasyDMA channel"]
pub use txd::TXD;
#[doc = r"Cluster"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "ADDRESS (rw) register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Description collection: TWI slave address n"]
pub mod address;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "ORC (rw) register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
