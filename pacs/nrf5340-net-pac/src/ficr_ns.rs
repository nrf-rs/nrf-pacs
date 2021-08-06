#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200..0x22c - Device info"]
    pub info: INFO,
    _reserved1: [u8; 0x54],
    #[doc = "0x280..0x290 - Description collection: Encryption Root, word n"]
    pub er: [crate::Reg<er::ER_SPEC>; 4],
    #[doc = "0x290..0x2a0 - Description collection: Identity Root, word n"]
    pub ir: [crate::Reg<ir::IR_SPEC>; 4],
    #[doc = "0x2a0 - Device address type"]
    pub deviceaddrtype: crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>,
    #[doc = "0x2a4..0x2ac - Description collection: Device address n"]
    pub deviceaddr: [crate::Reg<deviceaddr::DEVICEADDR_SPEC>; 2],
    _reserved5: [u8; 0x54],
    #[doc = "0x300..0x400 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
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
    #[doc = "0x00 - Description cluster: Address"]
    pub addr: crate::Reg<self::trimcnf::addr::ADDR_SPEC>,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: crate::Reg<self::trimcnf::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = "ER register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "Description collection: Encryption Root, word n"]
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
