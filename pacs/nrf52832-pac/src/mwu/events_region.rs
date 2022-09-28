#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_REGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Write access to region 0 detected"]
    pub wa: WA,
    #[doc = "0x04 - Description cluster\\[0\\]: Read access to region 0 detected"]
    pub ra: RA,
}
#[doc = "WA (rw) register accessor: an alias for `Reg<WA_SPEC>`"]
pub type WA = crate::Reg<wa::WA_SPEC>;
#[doc = "Description cluster\\[0\\]: Write access to region 0 detected"]
pub mod wa;
#[doc = "RA (rw) register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Description cluster\\[0\\]: Read access to region 0 detected"]
pub mod ra;
