#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200..0x22c - Device info"]
    pub info: INFO,
    _reserved1: [u8; 0x54],
    #[doc = "0x280..0x290 - Description collection: Encryption Root, word n"]
    pub er: [ER; 4],
    #[doc = "0x290..0x2a0 - Description collection: Identity Root, word n"]
    pub ir: [IR; 4],
    #[doc = "0x2a0 - Device address type"]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0x2a4..0x2ac - Description collection: Device address n"]
    pub deviceaddr: [DEVICEADDR; 2],
    _reserved5: [u8; 0x54],
    #[doc = "0x300..0x400 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
}
#[doc = "Device info"]
pub use info::INFO;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "ER (r) register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "Description collection: Encryption Root, word n"]
pub mod er;
#[doc = "IR (r) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Description collection: Identity Root, word n"]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: an alias for `Reg<DEVICEADDRTYPE_SPEC>`"]
pub type DEVICEADDRTYPE = crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: an alias for `Reg<DEVICEADDR_SPEC>`"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Description collection: Device address n"]
pub mod deviceaddr;
#[doc = "Unspecified"]
pub use trimcnf::TRIMCNF;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod trimcnf;
