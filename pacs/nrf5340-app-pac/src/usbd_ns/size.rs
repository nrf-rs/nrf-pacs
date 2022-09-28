#[doc = r"Register block"]
#[repr(C)]
pub struct SIZE {
    #[doc = "0x00..0x20 - Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
    pub epout: [EPOUT; 8],
    #[doc = "0x20 - Number of bytes received last on this ISO OUT data endpoint"]
    pub isoout: ISOOUT,
}
#[doc = "EPOUT (rw) register accessor: an alias for `Reg<EPOUT_SPEC>`"]
pub type EPOUT = crate::Reg<epout::EPOUT_SPEC>;
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
pub mod epout;
#[doc = "ISOOUT (r) register accessor: an alias for `Reg<ISOOUT_SPEC>`"]
pub type ISOOUT = crate::Reg<isoout::ISOOUT_SPEC>;
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub mod isoout;
