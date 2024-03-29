#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x24],
    #[doc = "0x24 - Acquire SPI semaphore."]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore."]
    pub tasks_release: TASKS_RELEASE,
    _reserved2: [u8; 0xd8],
    #[doc = "0x104 - Granted transaction completed."]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 0x08],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 0x14],
    #[doc = "0x128 - Semaphore acquired."]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 0xd4],
    #[doc = "0x200 - Shortcuts for SPIS."]
    pub shorts: SHORTS,
    _reserved6: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 0xf4],
    #[doc = "0x400 - Semaphore status."]
    pub semstat: SEMSTAT,
    _reserved9: [u8; 0x3c],
    #[doc = "0x440 - Status from last transaction."]
    pub status: STATUS,
    _reserved10: [u8; 0xbc],
    #[doc = "0x500 - Enable SPIS."]
    pub enable: ENABLE,
    _reserved11: [u8; 0x04],
    #[doc = "0x508 - Pin select for SCK."]
    pub pselsck: PSELSCK,
    #[doc = "0x50c - Pin select for MISO."]
    pub pselmiso: PSELMISO,
    #[doc = "0x510 - Pin select for MOSI."]
    pub pselmosi: PSELMOSI,
    #[doc = "0x514 - Pin select for CSN."]
    pub pselcsn: PSELCSN,
    _reserved15: [u8; 0x1c],
    #[doc = "0x534 - RX data pointer."]
    pub rxdptr: RXDPTR,
    #[doc = "0x538 - Maximum number of bytes in the receive buffer."]
    pub maxrx: MAXRX,
    #[doc = "0x53c - Number of bytes received in last granted transaction."]
    pub amountrx: AMOUNTRX,
    _reserved18: [u8; 0x04],
    #[doc = "0x544 - TX data pointer."]
    pub txdptr: TXDPTR,
    #[doc = "0x548 - Maximum number of bytes in the transmit buffer."]
    pub maxtx: MAXTX,
    #[doc = "0x54c - Number of bytes transmitted in last granted transaction."]
    pub amounttx: AMOUNTTX,
    _reserved21: [u8; 0x04],
    #[doc = "0x554 - Configuration register."]
    pub config: CONFIG,
    _reserved22: [u8; 0x04],
    #[doc = "0x55c - Default character."]
    pub def: DEF,
    _reserved23: [u8; 0x60],
    #[doc = "0x5c0 - Over-read character."]
    pub orc: ORC,
    _reserved24: [u8; 0x0a38],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_ACQUIRE (w) register accessor: an alias for `Reg<TASKS_ACQUIRE_SPEC>`"]
pub type TASKS_ACQUIRE = crate::Reg<tasks_acquire::TASKS_ACQUIRE_SPEC>;
#[doc = "Acquire SPI semaphore."]
pub mod tasks_acquire;
#[doc = "TASKS_RELEASE (w) register accessor: an alias for `Reg<TASKS_RELEASE_SPEC>`"]
pub type TASKS_RELEASE = crate::Reg<tasks_release::TASKS_RELEASE_SPEC>;
#[doc = "Release SPI semaphore."]
pub mod tasks_release;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Granted transaction completed."]
pub mod events_end;
#[doc = "EVENTS_ENDRX (rw) register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_ACQUIRED (rw) register accessor: an alias for `Reg<EVENTS_ACQUIRED_SPEC>`"]
pub type EVENTS_ACQUIRED = crate::Reg<events_acquired::EVENTS_ACQUIRED_SPEC>;
#[doc = "Semaphore acquired."]
pub mod events_acquired;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for SPIS."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "SEMSTAT (r) register accessor: an alias for `Reg<SEMSTAT_SPEC>`"]
pub type SEMSTAT = crate::Reg<semstat::SEMSTAT_SPEC>;
#[doc = "Semaphore status."]
pub mod semstat;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status from last transaction."]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPIS."]
pub mod enable;
#[doc = "PSELSCK (rw) register accessor: an alias for `Reg<PSELSCK_SPEC>`"]
pub type PSELSCK = crate::Reg<pselsck::PSELSCK_SPEC>;
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "PSELMISO (rw) register accessor: an alias for `Reg<PSELMISO_SPEC>`"]
pub type PSELMISO = crate::Reg<pselmiso::PSELMISO_SPEC>;
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "PSELMOSI (rw) register accessor: an alias for `Reg<PSELMOSI_SPEC>`"]
pub type PSELMOSI = crate::Reg<pselmosi::PSELMOSI_SPEC>;
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "PSELCSN (rw) register accessor: an alias for `Reg<PSELCSN_SPEC>`"]
pub type PSELCSN = crate::Reg<pselcsn::PSELCSN_SPEC>;
#[doc = "Pin select for CSN."]
pub mod pselcsn;
#[doc = "RXDPTR (rw) register accessor: an alias for `Reg<RXDPTR_SPEC>`"]
pub type RXDPTR = crate::Reg<rxdptr::RXDPTR_SPEC>;
#[doc = "RX data pointer."]
pub mod rxdptr;
#[doc = "MAXRX (rw) register accessor: an alias for `Reg<MAXRX_SPEC>`"]
pub type MAXRX = crate::Reg<maxrx::MAXRX_SPEC>;
#[doc = "Maximum number of bytes in the receive buffer."]
pub mod maxrx;
#[doc = "AMOUNTRX (r) register accessor: an alias for `Reg<AMOUNTRX_SPEC>`"]
pub type AMOUNTRX = crate::Reg<amountrx::AMOUNTRX_SPEC>;
#[doc = "Number of bytes received in last granted transaction."]
pub mod amountrx;
#[doc = "TXDPTR (rw) register accessor: an alias for `Reg<TXDPTR_SPEC>`"]
pub type TXDPTR = crate::Reg<txdptr::TXDPTR_SPEC>;
#[doc = "TX data pointer."]
pub mod txdptr;
#[doc = "MAXTX (rw) register accessor: an alias for `Reg<MAXTX_SPEC>`"]
pub type MAXTX = crate::Reg<maxtx::MAXTX_SPEC>;
#[doc = "Maximum number of bytes in the transmit buffer."]
pub mod maxtx;
#[doc = "AMOUNTTX (r) register accessor: an alias for `Reg<AMOUNTTX_SPEC>`"]
pub type AMOUNTTX = crate::Reg<amounttx::AMOUNTTX_SPEC>;
#[doc = "Number of bytes transmitted in last granted transaction."]
pub mod amounttx;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "DEF (rw) register accessor: an alias for `Reg<DEF_SPEC>`"]
pub type DEF = crate::Reg<def::DEF_SPEC>;
#[doc = "Default character."]
pub mod def;
#[doc = "ORC (rw) register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character."]
pub mod orc;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
