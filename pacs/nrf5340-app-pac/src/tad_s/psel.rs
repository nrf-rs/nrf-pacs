#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin configuration for TRACECLK"]
    pub traceclk: TRACECLK,
    #[doc = "0x04 - Pin configuration for TRACEDATA\\[0\\]"]
    pub tracedata0: TRACEDATA0,
    #[doc = "0x08 - Pin configuration for TRACEDATA\\[1\\]"]
    pub tracedata1: TRACEDATA1,
    #[doc = "0x0c - Pin configuration for TRACEDATA\\[2\\]"]
    pub tracedata2: TRACEDATA2,
    #[doc = "0x10 - Pin configuration for TRACEDATA\\[3\\]"]
    pub tracedata3: TRACEDATA3,
}
#[doc = "TRACECLK (rw) register accessor: an alias for `Reg<TRACECLK_SPEC>`"]
pub type TRACECLK = crate::Reg<traceclk::TRACECLK_SPEC>;
#[doc = "Pin configuration for TRACECLK"]
pub mod traceclk;
#[doc = "TRACEDATA0 (rw) register accessor: an alias for `Reg<TRACEDATA0_SPEC>`"]
pub type TRACEDATA0 = crate::Reg<tracedata0::TRACEDATA0_SPEC>;
#[doc = "Pin configuration for TRACEDATA\\[0\\]"]
pub mod tracedata0;
#[doc = "TRACEDATA1 (rw) register accessor: an alias for `Reg<TRACEDATA1_SPEC>`"]
pub type TRACEDATA1 = crate::Reg<tracedata1::TRACEDATA1_SPEC>;
#[doc = "Pin configuration for TRACEDATA\\[1\\]"]
pub mod tracedata1;
#[doc = "TRACEDATA2 (rw) register accessor: an alias for `Reg<TRACEDATA2_SPEC>`"]
pub type TRACEDATA2 = crate::Reg<tracedata2::TRACEDATA2_SPEC>;
#[doc = "Pin configuration for TRACEDATA\\[2\\]"]
pub mod tracedata2;
#[doc = "TRACEDATA3 (rw) register accessor: an alias for `Reg<TRACEDATA3_SPEC>`"]
pub type TRACEDATA3 = crate::Reg<tracedata3::TRACEDATA3_SPEC>;
#[doc = "Pin configuration for TRACEDATA\\[3\\]"]
pub mod tracedata3;
