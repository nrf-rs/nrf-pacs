#[doc = r"Register block"]
#[repr(C)]
pub struct REGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Start address for region 0"]
    pub start: START,
    #[doc = "0x04 - Description cluster\\[0\\]: End address of region 0"]
    pub end: END,
}
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Description cluster\\[0\\]: Start address for region 0"]
pub mod start;
#[doc = "END (rw) register accessor: an alias for `Reg<END_SPEC>`"]
pub type END = crate::Reg<end::END_SPEC>;
#[doc = "Description cluster\\[0\\]: End address of region 0"]
pub mod end;
