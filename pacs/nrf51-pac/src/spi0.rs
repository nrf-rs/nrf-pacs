#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0108],
    #[doc = "0x108 - TXD byte sent and RXD byte received."]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    _reserved1: [u8; 0x01f8],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved3: [u8; 0x01f4],
    #[doc = "0x500 - Enable SPI."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x508 - Pin select for SCK."]
    pub pselsck: crate::Reg<pselsck::PSELSCK_SPEC>,
    #[doc = "0x50c - Pin select for MOSI."]
    pub pselmosi: crate::Reg<pselmosi::PSELMOSI_SPEC>,
    #[doc = "0x510 - Pin select for MISO."]
    pub pselmiso: crate::Reg<pselmiso::PSELMISO_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x518 - RX data."]
    pub rxd: crate::Reg<rxd::RXD_SPEC>,
    #[doc = "0x51c - TX data."]
    pub txd: crate::Reg<txd::TXD_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x524 - SPI frequency"]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved10: [u8; 0x2c],
    #[doc = "0x554 - Configuration register."]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved11: [u8; 0x0aa4],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "TXD byte sent and RXD byte received."]
pub mod events_ready;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPI."]
pub mod enable;
#[doc = "PSELSCK register accessor: an alias for `Reg<PSELSCK_SPEC>`"]
pub type PSELSCK = crate::Reg<pselsck::PSELSCK_SPEC>;
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "PSELMOSI register accessor: an alias for `Reg<PSELMOSI_SPEC>`"]
pub type PSELMOSI = crate::Reg<pselmosi::PSELMOSI_SPEC>;
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "PSELMISO register accessor: an alias for `Reg<PSELMISO_SPEC>`"]
pub type PSELMISO = crate::Reg<pselmiso::PSELMISO_SPEC>;
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "RXD register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RX data."]
pub mod rxd;
#[doc = "TXD register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TX data."]
pub mod txd;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "SPI frequency"]
pub mod frequency;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
