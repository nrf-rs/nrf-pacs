#[doc = "PSELP register accessor: an alias for `Reg<PSELP_SPEC>`"]
pub type PSELP = crate::Reg<pselp::PSELP_SPEC>;
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
pub mod pselp;
#[doc = "PSELN register accessor: an alias for `Reg<PSELN_SPEC>`"]
pub type PSELN = crate::Reg<pseln::PSELN_SPEC>;
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
pub mod pseln;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Description cluster: Input configuration for CH\\[n\\]"]
pub mod config;
#[doc = "LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "Description cluster: High/low limits for event monitoring of a channel"]
pub mod limit;
