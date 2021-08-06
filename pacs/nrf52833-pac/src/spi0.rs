#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0108],
    #[doc = "0x108 - TXD byte sent and RXD byte received"]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    _reserved1: [u8; 0x01f8],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved3: [u8; 0x01f4],
    #[doc = "0x500 - Enable SPI"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x508..0x514 - Unspecified"]
    pub psel: PSEL,
    _reserved5: [u8; 0x04],
    #[doc = "0x518 - RXD register"]
    pub rxd: crate::Reg<rxd::RXD_SPEC>,
    #[doc = "0x51c - TXD register"]
    pub txd: crate::Reg<txd::TXD_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved8: [u8; 0x2c],
    #[doc = "0x554 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
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
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "TXD byte sent and RXD byte received"]
pub mod events_ready;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPI"]
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
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
