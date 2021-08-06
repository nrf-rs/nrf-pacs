#[doc = "VREQH register accessor: an alias for `Reg<VREQH_SPEC>`"]
pub type VREQH = crate::Reg<vreqh::VREQH_SPEC>;
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
pub mod vreqh;
#[doc = "VREQHREADY register accessor: an alias for `Reg<VREQHREADY_SPEC>`"]
pub type VREQHREADY = crate::Reg<vreqhready::VREQHREADY_SPEC>;
#[doc = "High voltage on RADIO is ready"]
pub mod vreqhready;
