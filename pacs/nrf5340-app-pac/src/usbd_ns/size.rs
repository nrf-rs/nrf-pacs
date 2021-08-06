#[doc = "EPOUT register accessor: an alias for `Reg<EPOUT_SPEC>`"]
pub type EPOUT = crate::Reg<epout::EPOUT_SPEC>;
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
pub mod epout;
#[doc = "ISOOUT register accessor: an alias for `Reg<ISOOUT_SPEC>`"]
pub type ISOOUT = crate::Reg<isoout::ISOOUT_SPEC>;
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub mod isoout;
