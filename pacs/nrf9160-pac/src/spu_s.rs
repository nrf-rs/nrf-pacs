#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - A security violation has been detected for the RAM memory space"]
    pub events_ramaccerr: crate::Reg<events_ramaccerr::EVENTS_RAMACCERR_SPEC>,
    #[doc = "0x104 - A security violation has been detected for the flash memory space"]
    pub events_flashaccerr: crate::Reg<events_flashaccerr::EVENTS_FLASHACCERR_SPEC>,
    #[doc = "0x108 - A security violation has been detected on one or several peripherals"]
    pub events_periphaccerr: crate::Reg<events_periphaccerr::EVENTS_PERIPHACCERR_SPEC>,
    _reserved3: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event RAMACCERR"]
    pub publish_ramaccerr: crate::Reg<publish_ramaccerr::PUBLISH_RAMACCERR_SPEC>,
    #[doc = "0x184 - Publish configuration for event FLASHACCERR"]
    pub publish_flashaccerr: crate::Reg<publish_flashaccerr::PUBLISH_FLASHACCERR_SPEC>,
    #[doc = "0x188 - Publish configuration for event PERIPHACCERR"]
    pub publish_periphaccerr: crate::Reg<publish_periphaccerr::PUBLISH_PERIPHACCERR_SPEC>,
    _reserved6: [u8; 0x0174],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0xf4],
    #[doc = "0x400 - Show implemented features for the current device"]
    pub cap: crate::Reg<cap::CAP_SPEC>,
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
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTDOMAIN {
    #[doc = "0x00 - Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    pub perm: crate::Reg<self::extdomain::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extdomain;
#[doc = r"Register block"]
#[repr(C)]
pub struct DPPI {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
    pub perm: crate::Reg<self::dppi::perm::PERM_SPEC>,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: crate::Reg<self::dppi::lock::LOCK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod dppi;
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIOPORT {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
    pub perm: crate::Reg<self::gpioport::perm::PERM_SPEC>,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: crate::Reg<self::gpioport::lock::LOCK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod gpioport;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHNSC {
    #[doc = "0x00 - Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
    pub region: crate::Reg<self::flashnsc::region::REGION_SPEC>,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: crate::Reg<self::flashnsc::size::SIZE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod flashnsc;
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMNSC {
    #[doc = "0x00 - Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
    pub region: crate::Reg<self::ramnsc::region::REGION_SPEC>,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: crate::Reg<self::ramnsc::size::SIZE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ramnsc;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHREGION {
    #[doc = "0x00 - Description cluster: Access permissions for flash region n"]
    pub perm: crate::Reg<self::flashregion::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod flashregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMREGION {
    #[doc = "0x00 - Description cluster: Access permissions for RAM region n"]
    pub perm: crate::Reg<self::ramregion::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ramregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct PERIPHID {
    #[doc = "0x00 - Description cluster: List capabilities and access permissions for the peripheral with ID n"]
    pub perm: crate::Reg<self::periphid::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod periphid;
#[doc = "EVENTS_RAMACCERR register accessor: an alias for `Reg<EVENTS_RAMACCERR_SPEC>`"]
pub type EVENTS_RAMACCERR = crate::Reg<events_ramaccerr::EVENTS_RAMACCERR_SPEC>;
#[doc = "A security violation has been detected for the RAM memory space"]
pub mod events_ramaccerr;
#[doc = "EVENTS_FLASHACCERR register accessor: an alias for `Reg<EVENTS_FLASHACCERR_SPEC>`"]
pub type EVENTS_FLASHACCERR = crate::Reg<events_flashaccerr::EVENTS_FLASHACCERR_SPEC>;
#[doc = "A security violation has been detected for the flash memory space"]
pub mod events_flashaccerr;
#[doc = "EVENTS_PERIPHACCERR register accessor: an alias for `Reg<EVENTS_PERIPHACCERR_SPEC>`"]
pub type EVENTS_PERIPHACCERR = crate::Reg<events_periphaccerr::EVENTS_PERIPHACCERR_SPEC>;
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
#[doc = "PUBLISH_RAMACCERR register accessor: an alias for `Reg<PUBLISH_RAMACCERR_SPEC>`"]
pub type PUBLISH_RAMACCERR = crate::Reg<publish_ramaccerr::PUBLISH_RAMACCERR_SPEC>;
#[doc = "Publish configuration for event RAMACCERR"]
pub mod publish_ramaccerr;
#[doc = "PUBLISH_FLASHACCERR register accessor: an alias for `Reg<PUBLISH_FLASHACCERR_SPEC>`"]
pub type PUBLISH_FLASHACCERR = crate::Reg<publish_flashaccerr::PUBLISH_FLASHACCERR_SPEC>;
#[doc = "Publish configuration for event FLASHACCERR"]
pub mod publish_flashaccerr;
#[doc = "PUBLISH_PERIPHACCERR register accessor: an alias for `Reg<PUBLISH_PERIPHACCERR_SPEC>`"]
pub type PUBLISH_PERIPHACCERR = crate::Reg<publish_periphaccerr::PUBLISH_PERIPHACCERR_SPEC>;
#[doc = "Publish configuration for event PERIPHACCERR"]
pub mod publish_periphaccerr;
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
#[doc = "CAP register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Show implemented features for the current device"]
pub mod cap;
