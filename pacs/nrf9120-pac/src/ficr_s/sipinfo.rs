#[repr(C)]
#[doc = "SIP-specific device info"]
#[doc(alias = "SIPINFO")]
pub struct Sipinfo {
    partno: Partno,
    hwrevision: [Hwrevision; 4],
    variant: [Variant; 4],
}
impl Sipinfo {
    #[doc = "0x00 - SIP part number"]
    #[inline(always)]
    pub const fn partno(&self) -> &Partno {
        &self.partno
    }
    #[doc = "0x04 - Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
    #[inline(always)]
    pub const fn hwrevision(&self, n: usize) -> &Hwrevision {
        &self.hwrevision[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04 - Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
    #[inline(always)]
    pub fn hwrevision_iter(&self) -> impl Iterator<Item = &Hwrevision> {
        self.hwrevision.iter()
    }
    #[doc = "0x08 - Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
    #[inline(always)]
    pub const fn variant(&self, n: usize) -> &Variant {
        &self.variant[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08 - Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
    #[inline(always)]
    pub fn variant_iter(&self) -> impl Iterator<Item = &Variant> {
        self.variant.iter()
    }
}
#[doc = "PARTNO (r) register accessor: SIP part number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`partno::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@partno`]
module"]
#[doc(alias = "PARTNO")]
pub type Partno = crate::Reg<partno::PartnoSpec>;
#[doc = "SIP part number"]
pub mod partno;
#[doc = "HWREVISION (r) register accessor: Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwrevision::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwrevision`]
module"]
#[doc(alias = "HWREVISION")]
pub type Hwrevision = crate::Reg<hwrevision::HwrevisionSpec>;
#[doc = "Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A"]
pub mod hwrevision;
#[doc = "VARIANT (r) register accessor: Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`variant::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@variant`]
module"]
#[doc(alias = "VARIANT")]
pub type Variant = crate::Reg<variant::VariantSpec>;
#[doc = "Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA"]
pub mod variant;
