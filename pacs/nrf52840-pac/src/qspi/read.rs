#[doc = r"Register block"]
#[repr(C)]
pub struct READ {
    #[doc = "0x00 - Flash memory source address"]
    pub src: SRC,
    #[doc = "0x04 - RAM destination address"]
    pub dst: DST,
    #[doc = "0x08 - Read transfer length"]
    pub cnt: CNT,
}
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Flash memory source address"]
pub mod src;
#[doc = "DST (rw) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "RAM destination address"]
pub mod dst;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Read transfer length"]
pub mod cnt;
