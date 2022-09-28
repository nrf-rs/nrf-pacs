#[doc = r"Register block"]
#[repr(C)]
pub struct WRITE {
    #[doc = "0x00 - Flash destination address"]
    pub dst: DST,
    #[doc = "0x04 - RAM source address"]
    pub src: SRC,
    #[doc = "0x08 - Write transfer length"]
    pub cnt: CNT,
}
#[doc = "DST (rw) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Flash destination address"]
pub mod dst;
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "RAM source address"]
pub mod src;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Write transfer length"]
pub mod cnt;
