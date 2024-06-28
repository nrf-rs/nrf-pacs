#[doc = r"Register block"]
#[repr(C)]
pub struct PUBLISH_CH {
    #[doc = "0x00 - Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
    pub limith: LIMITH,
    #[doc = "0x04 - Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
    pub limitl: LIMITL,
}
#[doc = "LIMITH (rw) register accessor: an alias for `Reg<LIMITH_SPEC>`"]
pub type LIMITH = crate::Reg<limith::LIMITH_SPEC>;
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
pub mod limith;
#[doc = "LIMITL (rw) register accessor: an alias for `Reg<LIMITL_SPEC>`"]
pub type LIMITL = crate::Reg<limitl::LIMITL_SPEC>;
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
pub mod limitl;
