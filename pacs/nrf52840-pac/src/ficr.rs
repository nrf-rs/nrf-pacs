#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Code memory page size"]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size"]
    pub codesize: CODESIZE,
    _reserved2: [u8; 0x48],
    #[doc = "0x60..0x68 - Description collection\\[n\\]: Device identifier"]
    pub deviceid: [DEVICEID; 2],
    _reserved3: [u8; 0x18],
    #[doc = "0x80..0x90 - Description collection\\[n\\]: Encryption root, word n"]
    pub er: [ER; 4],
    #[doc = "0x90..0xa0 - Description collection\\[n\\]: Identity Root, word n"]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type"]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4..0xac - Description collection\\[n\\]: Device address n"]
    pub deviceaddr: [DEVICEADDR; 2],
    _reserved7: [u8; 0x54],
    #[doc = "0x100..0x120 - Device info"]
    pub info: INFO,
    _reserved8: [u8; 0x0230],
    #[doc = "0x350..0x35c - Description collection\\[n\\]: Production test signature n"]
    pub prodtest: [PRODTEST; 3],
    _reserved9: [u8; 0xa8],
    #[doc = "0x404..0x448 - Registers storing factory TEMP module linearization coefficients"]
    pub temp: TEMP,
    _reserved10: [u8; 0x08],
    #[doc = "0x450..0x460 - Unspecified"]
    pub nfc: NFC,
    _reserved11: [u8; 0x07a0],
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
}
#[doc = "CODEPAGESIZE (r) register accessor: an alias for `Reg<CODEPAGESIZE_SPEC>`"]
pub type CODEPAGESIZE = crate::Reg<codepagesize::CODEPAGESIZE_SPEC>;
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: an alias for `Reg<CODESIZE_SPEC>`"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "DEVICEID (r) register accessor: an alias for `Reg<DEVICEID_SPEC>`"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Description collection\\[n\\]: Device identifier"]
pub mod deviceid;
#[doc = "ER (r) register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "Description collection\\[n\\]: Encryption root, word n"]
pub mod er;
#[doc = "IR (r) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Description collection\\[n\\]: Identity Root, word n"]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: an alias for `Reg<DEVICEADDRTYPE_SPEC>`"]
pub type DEVICEADDRTYPE = crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: an alias for `Reg<DEVICEADDR_SPEC>`"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Description collection\\[n\\]: Device address n"]
pub mod deviceaddr;
#[doc = "Device info"]
pub use info::INFO;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "PRODTEST (r) register accessor: an alias for `Reg<PRODTEST_SPEC>`"]
pub type PRODTEST = crate::Reg<prodtest::PRODTEST_SPEC>;
#[doc = "Description collection\\[n\\]: Production test signature n"]
pub mod prodtest;
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub use temp::TEMP;
#[doc = r"Cluster"]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub mod temp;
#[doc = "Unspecified"]
pub use nfc::NFC;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = "NIST800-90B RNG calibration data"]
pub use trng90b::TRNG90B;
#[doc = r"Cluster"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
