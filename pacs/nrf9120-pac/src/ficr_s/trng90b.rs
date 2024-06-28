#[doc = r"Register block"]
#[repr(C)]
pub struct TRNG90B {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    pub bytes: BYTES,
    #[doc = "0x04 - Repetition counter cutoff"]
    pub rccutoff: RCCUTOFF,
    #[doc = "0x08 - Adaptive proportion cutoff"]
    pub apcutoff: APCUTOFF,
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    pub startup: STARTUP,
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    pub rosc1: ROSC1,
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    pub rosc2: ROSC2,
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    pub rosc3: ROSC3,
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    pub rosc4: ROSC4,
}
#[doc = "BYTES (r) register accessor: an alias for `Reg<BYTES_SPEC>`"]
pub type BYTES = crate::Reg<bytes::BYTES_SPEC>;
#[doc = "Amount of bytes for the required entropy bits"]
pub mod bytes;
#[doc = "RCCUTOFF (r) register accessor: an alias for `Reg<RCCUTOFF_SPEC>`"]
pub type RCCUTOFF = crate::Reg<rccutoff::RCCUTOFF_SPEC>;
#[doc = "Repetition counter cutoff"]
pub mod rccutoff;
#[doc = "APCUTOFF (r) register accessor: an alias for `Reg<APCUTOFF_SPEC>`"]
pub type APCUTOFF = crate::Reg<apcutoff::APCUTOFF_SPEC>;
#[doc = "Adaptive proportion cutoff"]
pub mod apcutoff;
#[doc = "STARTUP (r) register accessor: an alias for `Reg<STARTUP_SPEC>`"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "Amount of bytes for the startup tests"]
pub mod startup;
#[doc = "ROSC1 (r) register accessor: an alias for `Reg<ROSC1_SPEC>`"]
pub type ROSC1 = crate::Reg<rosc1::ROSC1_SPEC>;
#[doc = "Sample count for ring oscillator 1"]
pub mod rosc1;
#[doc = "ROSC2 (r) register accessor: an alias for `Reg<ROSC2_SPEC>`"]
pub type ROSC2 = crate::Reg<rosc2::ROSC2_SPEC>;
#[doc = "Sample count for ring oscillator 2"]
pub mod rosc2;
#[doc = "ROSC3 (r) register accessor: an alias for `Reg<ROSC3_SPEC>`"]
pub type ROSC3 = crate::Reg<rosc3::ROSC3_SPEC>;
#[doc = "Sample count for ring oscillator 3"]
pub mod rosc3;
#[doc = "ROSC4 (r) register accessor: an alias for `Reg<ROSC4_SPEC>`"]
pub type ROSC4 = crate::Reg<rosc4::ROSC4_SPEC>;
#[doc = "Sample count for ring oscillator 4"]
pub mod rosc4;
