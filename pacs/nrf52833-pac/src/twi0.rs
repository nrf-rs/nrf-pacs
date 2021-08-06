#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: crate::Reg<tasks_resume::TASKS_RESUME_SPEC>,
    _reserved5: [u8; 0xe0],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    #[doc = "0x108 - TWI RXD byte received"]
    pub events_rxdready: crate::Reg<events_rxdready::EVENTS_RXDREADY_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x11c - TWI TXD byte sent"]
    pub events_txdsent: crate::Reg<events_txdsent::EVENTS_TXDSENT_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x124 - TWI error"]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x138 - TWI byte boundary, generated before each byte that is sent or received"]
    pub events_bb: crate::Reg<events_bb::EVENTS_BB_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x148 - TWI entered the suspended state"]
    pub events_suspended: crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>,
    _reserved11: [u8; 0xb4],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved12: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved14: [u8; 0x01b8],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: crate::Reg<errorsrc::ERRORSRC_SPEC>,
    _reserved15: [u8; 0x38],
    #[doc = "0x500 - Enable TWI"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x508..0x510 - Unspecified"]
    pub psel: PSEL,
    _reserved17: [u8; 0x08],
    #[doc = "0x518 - RXD register"]
    pub rxd: crate::Reg<rxd::RXD_SPEC>,
    #[doc = "0x51c - TXD register"]
    pub txd: crate::Reg<txd::TXD_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved20: [u8; 0x60],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: crate::Reg<address::ADDRESS_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL"]
    pub scl: crate::Reg<self::psel::scl::SCL_SPEC>,
    #[doc = "0x04 - Pin select for SDA"]
    pub sda: crate::Reg<self::psel::sda::SDA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_STARTRX register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "TASKS_STARTTX register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
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
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_RXDREADY register accessor: an alias for `Reg<EVENTS_RXDREADY_SPEC>`"]
pub type EVENTS_RXDREADY = crate::Reg<events_rxdready::EVENTS_RXDREADY_SPEC>;
#[doc = "TWI RXD byte received"]
pub mod events_rxdready;
#[doc = "EVENTS_TXDSENT register accessor: an alias for `Reg<EVENTS_TXDSENT_SPEC>`"]
pub type EVENTS_TXDSENT = crate::Reg<events_txdsent::EVENTS_TXDSENT_SPEC>;
#[doc = "TWI TXD byte sent"]
pub mod events_txdsent;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_BB register accessor: an alias for `Reg<EVENTS_BB_SPEC>`"]
pub type EVENTS_BB = crate::Reg<events_bb::EVENTS_BB_SPEC>;
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub mod events_bb;
#[doc = "EVENTS_SUSPENDED register accessor: an alias for `Reg<EVENTS_SUSPENDED_SPEC>`"]
pub type EVENTS_SUSPENDED = crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>;
#[doc = "TWI entered the suspended state"]
pub mod events_suspended;
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
#[doc = "ERRORSRC register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable TWI"]
pub mod enable;
#[doc = "RXD register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TXD register"]
pub mod txd;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "ADDRESS register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Address used in the TWI transfer"]
pub mod address;
