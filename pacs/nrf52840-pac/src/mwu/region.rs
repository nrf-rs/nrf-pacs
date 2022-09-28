#[doc = r"Register block"]
#[repr(C)]
pub struct REGION {
    #[doc = "0x00 - Description cluster\\[n\\]: Start address for region n"]
    pub start: START,
    #[doc = "0x04 - Description cluster\\[n\\]: End address of region n"]
    pub end: END,
}
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Description cluster\\[n\\]: Start address for region n"]
pub mod start;
#[doc = "END (rw) register accessor: an alias for `Reg<END_SPEC>`"]
pub type END = crate::Reg<end::END_SPEC>;
#[doc = "Description cluster\\[n\\]: End address of region n"]
pub mod end;
