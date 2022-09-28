#[doc = r"Register block"]
#[repr(C)]
pub struct IFTIMING {
    #[doc = "0x00 - Sample delay for input serial data on MISO"]
    pub rxdelay: RXDELAY,
    #[doc = "0x04 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
    pub csndur: CSNDUR,
}
#[doc = "RXDELAY (rw) register accessor: an alias for `Reg<RXDELAY_SPEC>`"]
pub type RXDELAY = crate::Reg<rxdelay::RXDELAY_SPEC>;
#[doc = "Sample delay for input serial data on MISO"]
pub mod rxdelay;
#[doc = "CSNDUR (rw) register accessor: an alias for `Reg<CSNDUR_SPEC>`"]
pub type CSNDUR = crate::Reg<csndur::CSNDUR_SPEC>;
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
pub mod csndur;
