#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
    pub pselp: PSELP,
    #[doc = "0x04 - Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
    pub pseln: PSELN,
    #[doc = "0x08 - Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
    pub config: CONFIG,
    #[doc = "0x0c - Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
    pub limit: LIMIT,
}
#[doc = "PSELP (rw) register accessor: an alias for `Reg<PSELP_SPEC>`"]
pub type PSELP = crate::Reg<pselp::PSELP_SPEC>;
#[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
pub mod pselp;
#[doc = "PSELN (rw) register accessor: an alias for `Reg<PSELN_SPEC>`"]
pub type PSELN = crate::Reg<pseln::PSELN_SPEC>;
#[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
pub mod pseln;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
pub mod config;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
pub mod limit;
