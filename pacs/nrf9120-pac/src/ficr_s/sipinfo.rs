#[doc = r"Register block"]
#[repr(C)]
pub struct SIPINFO {
    #[doc = "0x00 - SIP part number"]
    pub partno: PARTNO,
    #[doc = "0x04 - Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
    pub hwrevision: [HWREVISION; 4],
    #[doc = "0x08 - Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
    pub variant: [VARIANT; 4],
}
#[doc = "PARTNO (r) register accessor: an alias for `Reg<PARTNO_SPEC>`"]
pub type PARTNO = crate::Reg<partno::PARTNO_SPEC>;
#[doc = "SIP part number"]
pub mod partno;
#[doc = "HWREVISION (r) register accessor: an alias for `Reg<HWREVISION_SPEC>`"]
pub type HWREVISION = crate::Reg<hwrevision::HWREVISION_SPEC>;
#[doc = "Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
pub mod hwrevision;
#[doc = "VARIANT (r) register accessor: an alias for `Reg<VARIANT_SPEC>`"]
pub type VARIANT = crate::Reg<variant::VARIANT_SPEC>;
#[doc = "Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
pub mod variant;
