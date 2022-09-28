#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start 2-Wire master receive sequence."]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Start 2-Wire master transmit sequence."]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Stop 2-Wire transaction."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c - Suspend 2-Wire transaction."]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume 2-Wire transaction."]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 0xe0],
    #[doc = "0x104 - Two-wire stopped."]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - Two-wire ready to deliver new RXD byte received."]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 0x10],
    #[doc = "0x11c - Two-wire finished sending last TXD byte."]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 0x04],
    #[doc = "0x124 - Two-wire error detected."]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 0x10],
    #[doc = "0x138 - Two-wire byte boundary."]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 0x0c],
    #[doc = "0x148 - Two-wire suspended."]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 0xb4],
    #[doc = "0x200 - Shortcuts for TWI."]
    pub shorts: SHORTS,
    _reserved12: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 0x01b8],
    #[doc = "0x4c4 - Two-wire error source. Write error field to 1 to clear error."]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 0x38],
    #[doc = "0x500 - Enable two-wire master."]
    pub enable: ENABLE,
    _reserved16: [u8; 0x04],
    #[doc = "0x508 - Pin select for SCL."]
    pub pselscl: PSELSCL,
    #[doc = "0x50c - Pin select for SDA."]
    pub pselsda: PSELSDA,
    _reserved18: [u8; 0x08],
    #[doc = "0x518 - RX data register."]
    pub rxd: RXD,
    #[doc = "0x51c - TX data register."]
    pub txd: TXD,
    _reserved20: [u8; 0x04],
    #[doc = "0x524 - Two-wire frequency."]
    pub frequency: FREQUENCY,
    _reserved21: [u8; 0x60],
    #[doc = "0x588 - Address used in the two-wire transfer."]
    pub address: ADDRESS,
    _reserved22: [u8; 0x0a70],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_STARTRX (w) register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start 2-Wire master receive sequence."]
pub mod tasks_startrx;
#[doc = "TASKS_STARTTX (w) register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start 2-Wire master transmit sequence."]
pub mod tasks_starttx;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop 2-Wire transaction."]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend 2-Wire transaction."]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume 2-Wire transaction."]
pub mod tasks_resume;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "Two-wire stopped."]
pub mod events_stopped;
#[doc = "EVENTS_RXDREADY (rw) register accessor: an alias for `Reg<EVENTS_RXDREADY_SPEC>`"]
pub type EVENTS_RXDREADY = crate::Reg<events_rxdready::EVENTS_RXDREADY_SPEC>;
#[doc = "Two-wire ready to deliver new RXD byte received."]
pub mod events_rxdready;
#[doc = "EVENTS_TXDSENT (rw) register accessor: an alias for `Reg<EVENTS_TXDSENT_SPEC>`"]
pub type EVENTS_TXDSENT = crate::Reg<events_txdsent::EVENTS_TXDSENT_SPEC>;
#[doc = "Two-wire finished sending last TXD byte."]
pub mod events_txdsent;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Two-wire error detected."]
pub mod events_error;
#[doc = "EVENTS_BB (rw) register accessor: an alias for `Reg<EVENTS_BB_SPEC>`"]
pub type EVENTS_BB = crate::Reg<events_bb::EVENTS_BB_SPEC>;
#[doc = "Two-wire byte boundary."]
pub mod events_bb;
#[doc = "EVENTS_SUSPENDED (rw) register accessor: an alias for `Reg<EVENTS_SUSPENDED_SPEC>`"]
pub type EVENTS_SUSPENDED = crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>;
#[doc = "Two-wire suspended."]
pub mod events_suspended;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for TWI."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ERRORSRC (rw) register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Two-wire error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable two-wire master."]
pub mod enable;
#[doc = "PSELSCL (rw) register accessor: an alias for `Reg<PSELSCL_SPEC>`"]
pub type PSELSCL = crate::Reg<pselscl::PSELSCL_SPEC>;
#[doc = "Pin select for SCL."]
pub mod pselscl;
#[doc = "PSELSDA (rw) register accessor: an alias for `Reg<PSELSDA_SPEC>`"]
pub type PSELSDA = crate::Reg<pselsda::PSELSDA_SPEC>;
#[doc = "Pin select for SDA."]
pub mod pselsda;
#[doc = "RXD (r) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RX data register."]
pub mod rxd;
#[doc = "TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TX data register."]
pub mod txd;
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "Two-wire frequency."]
pub mod frequency;
#[doc = "ADDRESS (rw) register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Address used in the two-wire transfer."]
pub mod address;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
