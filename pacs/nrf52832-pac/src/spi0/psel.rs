#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: SCK,
    #[doc = "0x04 - Pin select for MOSI"]
    pub mosi: MOSI,
    #[doc = "0x08 - Pin select for MISO"]
    pub miso: MISO,
}
#[doc = "SCK (rw) register accessor: an alias for `Reg<SCK_SPEC>`"]
pub type SCK = crate::Reg<sck::SCK_SPEC>;
#[doc = "Pin select for SCK"]
pub mod sck;
#[doc = "MOSI (rw) register accessor: an alias for `Reg<MOSI_SPEC>`"]
pub type MOSI = crate::Reg<mosi::MOSI_SPEC>;
#[doc = "Pin select for MOSI"]
pub mod mosi;
#[doc = "MISO (rw) register accessor: an alias for `Reg<MISO_SPEC>`"]
pub type MISO = crate::Reg<miso::MISO_SPEC>;
#[doc = "Pin select for MISO"]
pub mod miso;
