#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Configuration identifier"]
    pub configid: CONFIGID,
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    pub deviceid: [DEVICEID; 2],
    #[doc = "0x0c - Part code"]
    pub part: PART,
    #[doc = "0x10 - Part Variant, Hardware version and Production configuration"]
    pub variant: VARIANT,
    #[doc = "0x14 - Package option"]
    pub package: PACKAGE,
    #[doc = "0x18 - RAM variant"]
    pub ram: RAM,
    #[doc = "0x1c - Flash variant"]
    pub flash: FLASH,
    #[doc = "0x20 - Code memory page size in bytes"]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x24 - Code memory size"]
    pub codesize: CODESIZE,
    #[doc = "0x28 - Device type"]
    pub devicetype: DEVICETYPE,
}
#[doc = "CONFIGID (r) register accessor: an alias for `Reg<CONFIGID_SPEC>`"]
pub type CONFIGID = crate::Reg<configid::CONFIGID_SPEC>;
#[doc = "Configuration identifier"]
pub mod configid;
#[doc = "DEVICEID (r) register accessor: an alias for `Reg<DEVICEID_SPEC>`"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Description collection: Device identifier"]
pub mod deviceid;
#[doc = "PART (r) register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: an alias for `Reg<VARIANT_SPEC>`"]
pub type VARIANT = crate::Reg<variant::VARIANT_SPEC>;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "PACKAGE (r) register accessor: an alias for `Reg<PACKAGE_SPEC>`"]
pub type PACKAGE = crate::Reg<package::PACKAGE_SPEC>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM (r) register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "FLASH (r) register accessor: an alias for `Reg<FLASH_SPEC>`"]
pub type FLASH = crate::Reg<flash::FLASH_SPEC>;
#[doc = "Flash variant"]
pub mod flash;
#[doc = "CODEPAGESIZE (r) register accessor: an alias for `Reg<CODEPAGESIZE_SPEC>`"]
pub type CODEPAGESIZE = crate::Reg<codepagesize::CODEPAGESIZE_SPEC>;
#[doc = "Code memory page size in bytes"]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: an alias for `Reg<CODESIZE_SPEC>`"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "DEVICETYPE (r) register accessor: an alias for `Reg<DEVICETYPE_SPEC>`"]
pub type DEVICETYPE = crate::Reg<devicetype::DEVICETYPE_SPEC>;
#[doc = "Device type"]
pub mod devicetype;
