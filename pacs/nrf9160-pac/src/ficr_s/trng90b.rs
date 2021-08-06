#[doc = "BYTES register accessor: an alias for `Reg<BYTES_SPEC>`"]
pub type BYTES = crate::Reg<bytes::BYTES_SPEC>;
#[doc = "Amount of bytes for the required entropy bits"]
pub mod bytes;
#[doc = "RCCUTOFF register accessor: an alias for `Reg<RCCUTOFF_SPEC>`"]
pub type RCCUTOFF = crate::Reg<rccutoff::RCCUTOFF_SPEC>;
#[doc = "Repetition counter cutoff"]
pub mod rccutoff;
#[doc = "APCUTOFF register accessor: an alias for `Reg<APCUTOFF_SPEC>`"]
pub type APCUTOFF = crate::Reg<apcutoff::APCUTOFF_SPEC>;
#[doc = "Adaptive proportion cutoff"]
pub mod apcutoff;
#[doc = "STARTUP register accessor: an alias for `Reg<STARTUP_SPEC>`"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "Amount of bytes for the startup tests"]
pub mod startup;
#[doc = "ROSC1 register accessor: an alias for `Reg<ROSC1_SPEC>`"]
pub type ROSC1 = crate::Reg<rosc1::ROSC1_SPEC>;
#[doc = "Sample count for ring oscillator 1"]
pub mod rosc1;
#[doc = "ROSC2 register accessor: an alias for `Reg<ROSC2_SPEC>`"]
pub type ROSC2 = crate::Reg<rosc2::ROSC2_SPEC>;
#[doc = "Sample count for ring oscillator 2"]
pub mod rosc2;
#[doc = "ROSC3 register accessor: an alias for `Reg<ROSC3_SPEC>`"]
pub type ROSC3 = crate::Reg<rosc3::ROSC3_SPEC>;
#[doc = "Sample count for ring oscillator 3"]
pub mod rosc3;
#[doc = "ROSC4 register accessor: an alias for `Reg<ROSC4_SPEC>`"]
pub type ROSC4 = crate::Reg<rosc4::ROSC4_SPEC>;
#[doc = "Sample count for ring oscillator 4"]
pub mod rosc4;
