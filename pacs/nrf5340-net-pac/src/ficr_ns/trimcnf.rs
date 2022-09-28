#[doc = r"Register block"]
#[repr(C)]
pub struct TRIMCNF {
    #[doc = "0x00 - Description cluster: Address"]
    pub addr: ADDR,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: DATA,
}
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Description cluster: Address"]
pub mod addr;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Description cluster: Data"]
pub mod data;
