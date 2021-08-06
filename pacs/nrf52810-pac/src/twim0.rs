#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: crate::Reg<tasks_resume::TASKS_RESUME_SPEC>,
    _reserved5: [u8; 0xe0],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved6: [u8; 0x1c],
    #[doc = "0x124 - TWI error"]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved7: [u8; 0x20],
    #[doc = "0x148 - Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
    pub events_suspended: crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>,
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x15c - Byte boundary, starting to receive the last byte"]
    pub events_lastrx: crate::Reg<events_lastrx::EVENTS_LASTRX_SPEC>,
    #[doc = "0x160 - Byte boundary, starting to transmit the last byte"]
    pub events_lasttx: crate::Reg<events_lasttx::EVENTS_LASTTX_SPEC>,
    _reserved12: [u8; 0x9c],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved13: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved16: [u8; 0x01b8],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: crate::Reg<errorsrc::ERRORSRC_SPEC>,
    _reserved17: [u8; 0x38],
    #[doc = "0x500 - Enable TWIM"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x508..0x510 - Unspecified"]
    pub psel: PSEL,
    _reserved19: [u8; 0x14],
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved22: [u8; 0x34],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: crate::Reg<address::ADDRESS_SPEC>,
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
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: crate::Reg<self::txd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::txd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
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
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
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
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_SUSPENDED register accessor: an alias for `Reg<EVENTS_SUSPENDED_SPEC>`"]
pub type EVENTS_SUSPENDED = crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>;
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "EVENTS_RXSTARTED register accessor: an alias for `Reg<EVENTS_RXSTARTED_SPEC>`"]
pub type EVENTS_RXSTARTED = crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "EVENTS_TXSTARTED register accessor: an alias for `Reg<EVENTS_TXSTARTED_SPEC>`"]
pub type EVENTS_TXSTARTED = crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "EVENTS_LASTRX register accessor: an alias for `Reg<EVENTS_LASTRX_SPEC>`"]
pub type EVENTS_LASTRX = crate::Reg<events_lastrx::EVENTS_LASTRX_SPEC>;
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "EVENTS_LASTTX register accessor: an alias for `Reg<EVENTS_LASTTX_SPEC>`"]
pub type EVENTS_LASTTX = crate::Reg<events_lasttx::EVENTS_LASTTX_SPEC>;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
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
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable TWIM"]
pub mod enable;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "ADDRESS register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Address used in the TWI transfer"]
pub mod address;
