#[doc = r"Register block"]
#[repr(C)]
pub struct PERREGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
    pub substatwa: SUBSTATWA,
    #[doc = "0x04 - Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
    pub substatra: SUBSTATRA,
}
#[doc = "SUBSTATWA (rw) register accessor: an alias for `Reg<SUBSTATWA_SPEC>`"]
pub type SUBSTATWA = crate::Reg<substatwa::SUBSTATWA_SPEC>;
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
pub mod substatwa;
#[doc = "SUBSTATRA (rw) register accessor: an alias for `Reg<SUBSTATRA_SPEC>`"]
pub type SUBSTATRA = crate::Reg<substatra::SUBSTATRA_SPEC>;
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
pub mod substatra;
