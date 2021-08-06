#[doc = "RXDELAY register accessor: an alias for `Reg<RXDELAY_SPEC>`"]
pub type RXDELAY = crate::Reg<rxdelay::RXDELAY_SPEC>;
#[doc = "Sample delay for input serial data on MISO"]
pub mod rxdelay;
#[doc = "CSNDUR register accessor: an alias for `Reg<CSNDUR_SPEC>`"]
pub type CSNDUR = crate::Reg<csndur::CSNDUR_SPEC>;
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
pub mod csndur;
