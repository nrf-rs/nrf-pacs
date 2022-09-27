#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for serial clock SCK"]
    pub sck: SCK,
    #[doc = "0x04 - Pin select for chip select signal CSN."]
    pub csn: CSN,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Pin select for serial data MOSI/IO0."]
    pub io0: IO0,
    #[doc = "0x10 - Pin select for serial data MISO/IO1."]
    pub io1: IO1,
    #[doc = "0x14 - Pin select for serial data IO2."]
    pub io2: IO2,
    #[doc = "0x18 - Pin select for serial data IO3."]
    pub io3: IO3,
}
#[doc = "SCK (rw) register accessor: an alias for `Reg<SCK_SPEC>`"]
pub type SCK = crate::Reg<sck::SCK_SPEC>;
#[doc = "Pin select for serial clock SCK"]
pub mod sck;
#[doc = "CSN (rw) register accessor: an alias for `Reg<CSN_SPEC>`"]
pub type CSN = crate::Reg<csn::CSN_SPEC>;
#[doc = "Pin select for chip select signal CSN."]
pub mod csn;
#[doc = "IO0 (rw) register accessor: an alias for `Reg<IO0_SPEC>`"]
pub type IO0 = crate::Reg<io0::IO0_SPEC>;
#[doc = "Pin select for serial data MOSI/IO0."]
pub mod io0;
#[doc = "IO1 (rw) register accessor: an alias for `Reg<IO1_SPEC>`"]
pub type IO1 = crate::Reg<io1::IO1_SPEC>;
#[doc = "Pin select for serial data MISO/IO1."]
pub mod io1;
#[doc = "IO2 (rw) register accessor: an alias for `Reg<IO2_SPEC>`"]
pub type IO2 = crate::Reg<io2::IO2_SPEC>;
#[doc = "Pin select for serial data IO2."]
pub mod io2;
#[doc = "IO3 (rw) register accessor: an alias for `Reg<IO3_SPEC>`"]
pub type IO3 = crate::Reg<io3::IO3_SPEC>;
#[doc = "Pin select for serial data IO3."]
pub mod io3;
