#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x04 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    _reserved2: [u8; 0x01f8],
    #[doc = "0x200..0x280 - Description collection: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 32],
    _reserved3: [u8; 0x80],
    #[doc = "0x300..0x380 - Description collection: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
}
#[doc = "APPROTECT (rw) register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "ERASEPROTECT (rw) register accessor: an alias for `Reg<ERASEPROTECT_SPEC>`"]
pub type ERASEPROTECT = crate::Reg<eraseprotect::ERASEPROTECT_SPEC>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "NRFFW (rw) register accessor: an alias for `Reg<NRFFW_SPEC>`"]
pub type NRFFW = crate::Reg<nrffw::NRFFW_SPEC>;
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "CUSTOMER (rw) register accessor: an alias for `Reg<CUSTOMER_SPEC>`"]
pub type CUSTOMER = crate::Reg<customer::CUSTOMER_SPEC>;
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
