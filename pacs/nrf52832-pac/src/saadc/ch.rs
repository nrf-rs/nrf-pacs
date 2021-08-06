#[doc = "PSELP register accessor: an alias for `Reg<PSELP_SPEC>`"]
pub type PSELP = crate::Reg<pselp::PSELP_SPEC>;
#[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
pub mod pselp;
#[doc = "PSELN register accessor: an alias for `Reg<PSELN_SPEC>`"]
pub type PSELN = crate::Reg<pseln::PSELN_SPEC>;
#[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
pub mod pseln;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
pub mod config;
#[doc = "LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
pub mod limit;
