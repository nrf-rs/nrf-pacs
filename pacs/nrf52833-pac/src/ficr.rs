#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Code memory page size"]
    pub codepagesize: crate::Reg<codepagesize::CODEPAGESIZE_SPEC>,
    #[doc = "0x14 - Code memory size"]
    pub codesize: crate::Reg<codesize::CODESIZE_SPEC>,
    _reserved2: [u8; 0x48],
    #[doc = "0x60..0x68 - Description collection: Device identifier"]
    pub deviceid: [crate::Reg<deviceid::DEVICEID_SPEC>; 2],
    _reserved3: [u8; 0x18],
    #[doc = "0x80..0x90 - Description collection: Encryption root, word n"]
    pub er: [crate::Reg<er::ER_SPEC>; 4],
    #[doc = "0x90..0xa0 - Description collection: Identity Root, word n"]
    pub ir: [crate::Reg<ir::IR_SPEC>; 4],
    #[doc = "0xa0 - Device address type"]
    pub deviceaddrtype: crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>,
    #[doc = "0xa4..0xac - Description collection: Device address n"]
    pub deviceaddr: [crate::Reg<deviceaddr::DEVICEADDR_SPEC>; 2],
    _reserved7: [u8; 0x54],
    #[doc = "0x100..0x114 - Device info"]
    pub info: INFO,
    _reserved8: [u8; 0x023c],
    #[doc = "0x350..0x35c - Description collection: Production test signature n"]
    pub prodtest: [crate::Reg<prodtest::PRODTEST_SPEC>; 3],
    _reserved9: [u8; 0xa8],
    #[doc = "0x404..0x448 - Registers storing factory TEMP module linearization coefficients"]
    pub temp: TEMP,
    _reserved10: [u8; 0x08],
    #[doc = "0x450..0x460 - Unspecified"]
    pub nfc: NFC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Part code"]
    pub part: crate::Reg<self::info::part::PART_SPEC>,
    #[doc = "0x04 - Build code (hardware version and production configuration)"]
    pub variant: crate::Reg<self::info::variant::VARIANT_SPEC>,
    #[doc = "0x08 - Package option"]
    pub package: crate::Reg<self::info::package::PACKAGE_SPEC>,
    #[doc = "0x0c - RAM variant"]
    pub ram: crate::Reg<self::info::ram::RAM_SPEC>,
    #[doc = "0x10 - Flash variant"]
    pub flash: crate::Reg<self::info::flash::FLASH_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r"Register block"]
#[repr(C)]
pub struct TEMP {
    #[doc = "0x00 - Slope definition A0"]
    pub a0: crate::Reg<self::temp::a0::A0_SPEC>,
    #[doc = "0x04 - Slope definition A1"]
    pub a1: crate::Reg<self::temp::a1::A1_SPEC>,
    #[doc = "0x08 - Slope definition A2"]
    pub a2: crate::Reg<self::temp::a2::A2_SPEC>,
    #[doc = "0x0c - Slope definition A3"]
    pub a3: crate::Reg<self::temp::a3::A3_SPEC>,
    #[doc = "0x10 - Slope definition A4"]
    pub a4: crate::Reg<self::temp::a4::A4_SPEC>,
    #[doc = "0x14 - Slope definition A5"]
    pub a5: crate::Reg<self::temp::a5::A5_SPEC>,
    #[doc = "0x18 - Y-intercept B0"]
    pub b0: crate::Reg<self::temp::b0::B0_SPEC>,
    #[doc = "0x1c - Y-intercept B1"]
    pub b1: crate::Reg<self::temp::b1::B1_SPEC>,
    #[doc = "0x20 - Y-intercept B2"]
    pub b2: crate::Reg<self::temp::b2::B2_SPEC>,
    #[doc = "0x24 - Y-intercept B3"]
    pub b3: crate::Reg<self::temp::b3::B3_SPEC>,
    #[doc = "0x28 - Y-intercept B4"]
    pub b4: crate::Reg<self::temp::b4::B4_SPEC>,
    #[doc = "0x2c - Y-intercept B5"]
    pub b5: crate::Reg<self::temp::b5::B5_SPEC>,
    #[doc = "0x30 - Segment end T0"]
    pub t0: crate::Reg<self::temp::t0::T0_SPEC>,
    #[doc = "0x34 - Segment end T1"]
    pub t1: crate::Reg<self::temp::t1::T1_SPEC>,
    #[doc = "0x38 - Segment end T2"]
    pub t2: crate::Reg<self::temp::t2::T2_SPEC>,
    #[doc = "0x3c - Segment end T3"]
    pub t3: crate::Reg<self::temp::t3::T3_SPEC>,
    #[doc = "0x40 - Segment end T4"]
    pub t4: crate::Reg<self::temp::t4::T4_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub mod temp;
#[doc = r"Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    pub tagheader0: crate::Reg<self::nfc::tagheader0::TAGHEADER0_SPEC>,
    #[doc = "0x04 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    pub tagheader1: crate::Reg<self::nfc::tagheader1::TAGHEADER1_SPEC>,
    #[doc = "0x08 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    pub tagheader2: crate::Reg<self::nfc::tagheader2::TAGHEADER2_SPEC>,
    #[doc = "0x0c - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    pub tagheader3: crate::Reg<self::nfc::tagheader3::TAGHEADER3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = "CODEPAGESIZE register accessor: an alias for `Reg<CODEPAGESIZE_SPEC>`"]
pub type CODEPAGESIZE = crate::Reg<codepagesize::CODEPAGESIZE_SPEC>;
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "CODESIZE register accessor: an alias for `Reg<CODESIZE_SPEC>`"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "DEVICEID register accessor: an alias for `Reg<DEVICEID_SPEC>`"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Description collection: Device identifier"]
pub mod deviceid;
#[doc = "ER register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "Description collection: Encryption root, word n"]
pub mod er;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Description collection: Identity Root, word n"]
pub mod ir;
#[doc = "DEVICEADDRTYPE register accessor: an alias for `Reg<DEVICEADDRTYPE_SPEC>`"]
pub type DEVICEADDRTYPE = crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR register accessor: an alias for `Reg<DEVICEADDR_SPEC>`"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Description collection: Device address n"]
pub mod deviceaddr;
#[doc = "PRODTEST register accessor: an alias for `Reg<PRODTEST_SPEC>`"]
pub type PRODTEST = crate::Reg<prodtest::PRODTEST_SPEC>;
#[doc = "Description collection: Production test signature n"]
pub mod prodtest;
