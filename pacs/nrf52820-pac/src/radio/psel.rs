#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00..0x20 - Description collection: Pin select for DFE pin n"]
    pub dfegpio: [DFEGPIO; 8],
}
#[doc = "DFEGPIO (rw) register accessor: an alias for `Reg<DFEGPIO_SPEC>`"]
pub type DFEGPIO = crate::Reg<dfegpio::DFEGPIO_SPEC>;
#[doc = "Description collection: Pin select for DFE pin n"]
pub mod dfegpio;
