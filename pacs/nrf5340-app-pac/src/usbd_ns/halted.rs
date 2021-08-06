#[doc = "EPIN register accessor: an alias for `Reg<EPIN_SPEC>`"]
pub type EPIN = crate::Reg<epin::EPIN_SPEC>;
#[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epin;
#[doc = "EPOUT register accessor: an alias for `Reg<EPOUT_SPEC>`"]
pub type EPOUT = crate::Reg<epout::EPOUT_SPEC>;
#[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epout;
