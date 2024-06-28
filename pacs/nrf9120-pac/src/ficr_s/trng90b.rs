#[repr(C)]
#[doc = "NIST800-90B RNG calibration data"]
#[doc(alias = "TRNG90B")]
pub struct Trng90b {
    bytes: Bytes,
    rccutoff: Rccutoff,
    apcutoff: Apcutoff,
    startup: Startup,
    rosc1: Rosc1,
    rosc2: Rosc2,
    rosc3: Rosc3,
    rosc4: Rosc4,
}
impl Trng90b {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
    }
    #[doc = "0x04 - Repetition counter cutoff"]
    #[inline(always)]
    pub const fn rccutoff(&self) -> &Rccutoff {
        &self.rccutoff
    }
    #[doc = "0x08 - Adaptive proportion cutoff"]
    #[inline(always)]
    pub const fn apcutoff(&self) -> &Apcutoff {
        &self.apcutoff
    }
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    #[inline(always)]
    pub const fn startup(&self) -> &Startup {
        &self.startup
    }
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    #[inline(always)]
    pub const fn rosc1(&self) -> &Rosc1 {
        &self.rosc1
    }
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    #[inline(always)]
    pub const fn rosc2(&self) -> &Rosc2 {
        &self.rosc2
    }
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    #[inline(always)]
    pub const fn rosc3(&self) -> &Rosc3 {
        &self.rosc3
    }
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    #[inline(always)]
    pub const fn rosc4(&self) -> &Rosc4 {
        &self.rosc4
    }
}
#[doc = "BYTES (r) register accessor: Amount of bytes for the required entropy bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytes`]
module"]
#[doc(alias = "BYTES")]
pub type Bytes = crate::Reg<bytes::BytesSpec>;
#[doc = "Amount of bytes for the required entropy bits"]
pub mod bytes;
#[doc = "RCCUTOFF (r) register accessor: Repetition counter cutoff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rccutoff::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rccutoff`]
module"]
#[doc(alias = "RCCUTOFF")]
pub type Rccutoff = crate::Reg<rccutoff::RccutoffSpec>;
#[doc = "Repetition counter cutoff"]
pub mod rccutoff;
#[doc = "APCUTOFF (r) register accessor: Adaptive proportion cutoff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apcutoff::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apcutoff`]
module"]
#[doc(alias = "APCUTOFF")]
pub type Apcutoff = crate::Reg<apcutoff::ApcutoffSpec>;
#[doc = "Adaptive proportion cutoff"]
pub mod apcutoff;
#[doc = "STARTUP (r) register accessor: Amount of bytes for the startup tests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startup::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startup`]
module"]
#[doc(alias = "STARTUP")]
pub type Startup = crate::Reg<startup::StartupSpec>;
#[doc = "Amount of bytes for the startup tests"]
pub mod startup;
#[doc = "ROSC1 (r) register accessor: Sample count for ring oscillator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosc1`]
module"]
#[doc(alias = "ROSC1")]
pub type Rosc1 = crate::Reg<rosc1::Rosc1Spec>;
#[doc = "Sample count for ring oscillator 1"]
pub mod rosc1;
#[doc = "ROSC2 (r) register accessor: Sample count for ring oscillator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosc2`]
module"]
#[doc(alias = "ROSC2")]
pub type Rosc2 = crate::Reg<rosc2::Rosc2Spec>;
#[doc = "Sample count for ring oscillator 2"]
pub mod rosc2;
#[doc = "ROSC3 (r) register accessor: Sample count for ring oscillator 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosc3`]
module"]
#[doc(alias = "ROSC3")]
pub type Rosc3 = crate::Reg<rosc3::Rosc3Spec>;
#[doc = "Sample count for ring oscillator 3"]
pub mod rosc3;
#[doc = "ROSC4 (r) register accessor: Sample count for ring oscillator 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rosc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosc4`]
module"]
#[doc(alias = "ROSC4")]
pub type Rosc4 = crate::Reg<rosc4::Rosc4Spec>;
#[doc = "Sample count for ring oscillator 4"]
pub mod rosc4;
