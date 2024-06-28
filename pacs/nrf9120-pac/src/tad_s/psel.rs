#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    traceclk: Traceclk,
    tracedata0: Tracedata0,
    tracedata1: Tracedata1,
    tracedata2: Tracedata2,
    tracedata3: Tracedata3,
}
impl Psel {
    #[doc = "0x00 - Pin configuration for TRACECLK"]
    #[inline(always)]
    pub const fn traceclk(&self) -> &Traceclk {
        &self.traceclk
    }
    #[doc = "0x04 - Pin configuration for TRACEDATA\\[0\\]"]
    #[inline(always)]
    pub const fn tracedata0(&self) -> &Tracedata0 {
        &self.tracedata0
    }
    #[doc = "0x08 - Pin configuration for TRACEDATA\\[1\\]"]
    #[inline(always)]
    pub const fn tracedata1(&self) -> &Tracedata1 {
        &self.tracedata1
    }
    #[doc = "0x0c - Pin configuration for TRACEDATA\\[2\\]"]
    #[inline(always)]
    pub const fn tracedata2(&self) -> &Tracedata2 {
        &self.tracedata2
    }
    #[doc = "0x10 - Pin configuration for TRACEDATA\\[3\\]"]
    #[inline(always)]
    pub const fn tracedata3(&self) -> &Tracedata3 {
        &self.tracedata3
    }
}
#[doc = "TRACECLK (rw) register accessor: Pin configuration for TRACECLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclk`]
module"]
#[doc(alias = "TRACECLK")]
pub type Traceclk = crate::Reg<traceclk::TraceclkSpec>;
#[doc = "Pin configuration for TRACECLK"]
pub mod traceclk;
#[doc = "TRACEDATA0 (rw) register accessor: Pin configuration for TRACEDATA\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracedata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracedata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracedata0`]
module"]
#[doc(alias = "TRACEDATA0")]
pub type Tracedata0 = crate::Reg<tracedata0::Tracedata0Spec>;
#[doc = "Pin configuration for TRACEDATA\\[0\\]"]
pub mod tracedata0;
#[doc = "TRACEDATA1 (rw) register accessor: Pin configuration for TRACEDATA\\[1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracedata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracedata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracedata1`]
module"]
#[doc(alias = "TRACEDATA1")]
pub type Tracedata1 = crate::Reg<tracedata1::Tracedata1Spec>;
#[doc = "Pin configuration for TRACEDATA\\[1\\]"]
pub mod tracedata1;
#[doc = "TRACEDATA2 (rw) register accessor: Pin configuration for TRACEDATA\\[2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracedata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracedata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracedata2`]
module"]
#[doc(alias = "TRACEDATA2")]
pub type Tracedata2 = crate::Reg<tracedata2::Tracedata2Spec>;
#[doc = "Pin configuration for TRACEDATA\\[2\\]"]
pub mod tracedata2;
#[doc = "TRACEDATA3 (rw) register accessor: Pin configuration for TRACEDATA\\[3\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracedata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracedata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracedata3`]
module"]
#[doc(alias = "TRACEDATA3")]
pub type Tracedata3 = crate::Reg<tracedata3::Tracedata3Spec>;
#[doc = "Pin configuration for TRACEDATA\\[3\\]"]
pub mod tracedata3;
