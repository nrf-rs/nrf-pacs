#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - A security violation has been detected for the RAM memory space"]
    pub events_ramaccerr: EVENTS_RAMACCERR,
    #[doc = "0x104 - A security violation has been detected for the flash memory space"]
    pub events_flashaccerr: EVENTS_FLASHACCERR,
    #[doc = "0x108 - A security violation has been detected on one or several peripherals"]
    pub events_periphaccerr: EVENTS_PERIPHACCERR,
    _reserved3: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event RAMACCERR"]
    pub publish_ramaccerr: PUBLISH_RAMACCERR,
    #[doc = "0x184 - Publish configuration for event FLASHACCERR"]
    pub publish_flashaccerr: PUBLISH_FLASHACCERR,
    #[doc = "0x188 - Publish configuration for event PERIPHACCERR"]
    pub publish_periphaccerr: PUBLISH_PERIPHACCERR,
    _reserved6: [u8; 0x0174],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0xf4],
    #[doc = "0x400 - Show implemented features for the current device"]
    pub cap: CAP,
    _reserved10: [u8; 0x3c],
    #[doc = "0x440 - Unspecified"]
    pub extdomain: [EXTDOMAIN; 1],
    _reserved11: [u8; 0x3c],
    #[doc = "0x480..0x488 - Unspecified"]
    pub dppi: [DPPI; 1],
    _reserved12: [u8; 0x38],
    #[doc = "0x4c0..0x4c8 - Unspecified"]
    pub gpioport: [GPIOPORT; 1],
    _reserved13: [u8; 0x38],
    #[doc = "0x500..0x510 - Unspecified"]
    pub flashnsc: [FLASHNSC; 2],
    _reserved14: [u8; 0x30],
    #[doc = "0x540..0x550 - Unspecified"]
    pub ramnsc: [RAMNSC; 2],
    _reserved15: [u8; 0xb0],
    #[doc = "0x600..0x680 - Unspecified"]
    pub flashregion: [FLASHREGION; 32],
    _reserved16: [u8; 0x80],
    #[doc = "0x700..0x780 - Unspecified"]
    pub ramregion: [RAMREGION; 32],
    _reserved17: [u8; 0x80],
    #[doc = "0x800..0x90c - Unspecified"]
    pub periphid: [PERIPHID; 67],
}
#[doc = "EVENTS_RAMACCERR (rw) register accessor: an alias for `Reg<EVENTS_RAMACCERR_SPEC>`"]
pub type EVENTS_RAMACCERR = crate::Reg<events_ramaccerr::EVENTS_RAMACCERR_SPEC>;
#[doc = "A security violation has been detected for the RAM memory space"]
pub mod events_ramaccerr;
#[doc = "EVENTS_FLASHACCERR (rw) register accessor: an alias for `Reg<EVENTS_FLASHACCERR_SPEC>`"]
pub type EVENTS_FLASHACCERR = crate::Reg<events_flashaccerr::EVENTS_FLASHACCERR_SPEC>;
#[doc = "A security violation has been detected for the flash memory space"]
pub mod events_flashaccerr;
#[doc = "EVENTS_PERIPHACCERR (rw) register accessor: an alias for `Reg<EVENTS_PERIPHACCERR_SPEC>`"]
pub type EVENTS_PERIPHACCERR = crate::Reg<events_periphaccerr::EVENTS_PERIPHACCERR_SPEC>;
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
#[doc = "PUBLISH_RAMACCERR (rw) register accessor: an alias for `Reg<PUBLISH_RAMACCERR_SPEC>`"]
pub type PUBLISH_RAMACCERR = crate::Reg<publish_ramaccerr::PUBLISH_RAMACCERR_SPEC>;
#[doc = "Publish configuration for event RAMACCERR"]
pub mod publish_ramaccerr;
#[doc = "PUBLISH_FLASHACCERR (rw) register accessor: an alias for `Reg<PUBLISH_FLASHACCERR_SPEC>`"]
pub type PUBLISH_FLASHACCERR = crate::Reg<publish_flashaccerr::PUBLISH_FLASHACCERR_SPEC>;
#[doc = "Publish configuration for event FLASHACCERR"]
pub mod publish_flashaccerr;
#[doc = "PUBLISH_PERIPHACCERR (rw) register accessor: an alias for `Reg<PUBLISH_PERIPHACCERR_SPEC>`"]
pub type PUBLISH_PERIPHACCERR = crate::Reg<publish_periphaccerr::PUBLISH_PERIPHACCERR_SPEC>;
#[doc = "Publish configuration for event PERIPHACCERR"]
pub mod publish_periphaccerr;
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
#[doc = "CAP (r) register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Show implemented features for the current device"]
pub mod cap;
#[doc = "Unspecified"]
pub use extdomain::EXTDOMAIN;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extdomain;
#[doc = "Unspecified"]
pub use dppi::DPPI;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dppi;
#[doc = "Unspecified"]
pub use gpioport::GPIOPORT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod gpioport;
#[doc = "Unspecified"]
pub use flashnsc::FLASHNSC;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod flashnsc;
#[doc = "Unspecified"]
pub use ramnsc::RAMNSC;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ramnsc;
#[doc = "Unspecified"]
pub use flashregion::FLASHREGION;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod flashregion;
#[doc = "Unspecified"]
pub use ramregion::RAMREGION;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ramregion;
#[doc = "Unspecified"]
pub use periphid::PERIPHID;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod periphid;
