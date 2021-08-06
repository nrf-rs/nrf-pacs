#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200..0x22c - Device info"]
    pub info: INFO,
    _reserved1: [u8; 0xd4],
    #[doc = "0x300..0x400 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
    _reserved2: [u8; 0x50],
    #[doc = "0x450..0x460 - Unspecified"]
    pub nfc: NFC,
    _reserved3: [u8; 0x07a0],
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
    #[doc = "0xc20 - XOSC32M capacitor selection trim values"]
    pub xosc32mtrim: crate::Reg<xosc32mtrim::XOSC32MTRIM_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Configuration identifier"]
    pub configid: crate::Reg<self::info::configid::CONFIGID_SPEC>,
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    pub deviceid: [crate::Reg<self::info::deviceid::DEVICEID_SPEC>; 2],
    #[doc = "0x0c - Part code"]
    pub part: crate::Reg<self::info::part::PART_SPEC>,
    #[doc = "0x10 - Part Variant, Hardware version and Production configuration"]
    pub variant: crate::Reg<self::info::variant::VARIANT_SPEC>,
    #[doc = "0x14 - Package option"]
    pub package: crate::Reg<self::info::package::PACKAGE_SPEC>,
    #[doc = "0x18 - RAM variant"]
    pub ram: crate::Reg<self::info::ram::RAM_SPEC>,
    #[doc = "0x1c - Flash variant"]
    pub flash: crate::Reg<self::info::flash::FLASH_SPEC>,
    #[doc = "0x20 - Code memory page size in bytes"]
    pub codepagesize: crate::Reg<self::info::codepagesize::CODEPAGESIZE_SPEC>,
    #[doc = "0x24 - Code memory size"]
    pub codesize: crate::Reg<self::info::codesize::CODESIZE_SPEC>,
    #[doc = "0x28 - Device type"]
    pub devicetype: crate::Reg<self::info::devicetype::DEVICETYPE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRIMCNF {
    #[doc = "0x00 - Description cluster: Address of the PAR register which will be written"]
    pub addr: crate::Reg<self::trimcnf::addr::ADDR_SPEC>,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: crate::Reg<self::trimcnf::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = r"Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader0: crate::Reg<self::nfc::tagheader0::TAGHEADER0_SPEC>,
    #[doc = "0x04 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader1: crate::Reg<self::nfc::tagheader1::TAGHEADER1_SPEC>,
    #[doc = "0x08 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader2: crate::Reg<self::nfc::tagheader2::TAGHEADER2_SPEC>,
    #[doc = "0x0c - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader3: crate::Reg<self::nfc::tagheader3::TAGHEADER3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRNG90B {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    pub bytes: crate::Reg<self::trng90b::bytes::BYTES_SPEC>,
    #[doc = "0x04 - Repetition counter cutoff"]
    pub rccutoff: crate::Reg<self::trng90b::rccutoff::RCCUTOFF_SPEC>,
    #[doc = "0x08 - Adaptive proportion cutoff"]
    pub apcutoff: crate::Reg<self::trng90b::apcutoff::APCUTOFF_SPEC>,
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    pub startup: crate::Reg<self::trng90b::startup::STARTUP_SPEC>,
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    pub rosc1: crate::Reg<self::trng90b::rosc1::ROSC1_SPEC>,
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    pub rosc2: crate::Reg<self::trng90b::rosc2::ROSC2_SPEC>,
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    pub rosc3: crate::Reg<self::trng90b::rosc3::ROSC3_SPEC>,
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    pub rosc4: crate::Reg<self::trng90b::rosc4::ROSC4_SPEC>,
}
#[doc = r"Register block"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
#[doc = "XOSC32MTRIM register accessor: an alias for `Reg<XOSC32MTRIM_SPEC>`"]
pub type XOSC32MTRIM = crate::Reg<xosc32mtrim::XOSC32MTRIM_SPEC>;
#[doc = "XOSC32M capacitor selection trim values"]
pub mod xosc32mtrim;
