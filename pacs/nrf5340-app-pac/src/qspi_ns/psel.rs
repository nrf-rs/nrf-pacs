#[doc = "SCK register accessor: an alias for `Reg<SCK_SPEC>`"]
pub type SCK = crate::Reg<sck::SCK_SPEC>;
#[doc = "Pin select for serial clock SCK"]
pub mod sck;
#[doc = "CSN register accessor: an alias for `Reg<CSN_SPEC>`"]
pub type CSN = crate::Reg<csn::CSN_SPEC>;
#[doc = "Pin select for chip select signal CSN."]
pub mod csn;
#[doc = "IO0 register accessor: an alias for `Reg<IO0_SPEC>`"]
pub type IO0 = crate::Reg<io0::IO0_SPEC>;
#[doc = "Pin select for serial data MOSI/IO0."]
pub mod io0;
#[doc = "IO1 register accessor: an alias for `Reg<IO1_SPEC>`"]
pub type IO1 = crate::Reg<io1::IO1_SPEC>;
#[doc = "Pin select for serial data MISO/IO1."]
pub mod io1;
#[doc = "IO2 register accessor: an alias for `Reg<IO2_SPEC>`"]
pub type IO2 = crate::Reg<io2::IO2_SPEC>;
#[doc = "Pin select for serial data WP/IO2."]
pub mod io2;
#[doc = "IO3 register accessor: an alias for `Reg<IO3_SPEC>`"]
pub type IO3 = crate::Reg<io3::IO3_SPEC>;
#[doc = "Pin select for serial data HOLD/IO3."]
pub mod io3;
