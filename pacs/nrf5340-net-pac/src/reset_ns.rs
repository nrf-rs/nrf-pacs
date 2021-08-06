#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: crate::Reg<resetreas::RESETREAS_SPEC>,
}
#[doc = "RESETREAS register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
