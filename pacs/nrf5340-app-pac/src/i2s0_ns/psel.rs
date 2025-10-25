#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    mck: Mck,
    sck: Sck,
    lrck: Lrck,
    sdin: Sdin,
    sdout: Sdout,
}
impl Psel {
    #[doc = "0x00 - Pin select for MCK signal"]
    #[inline(always)]
    pub const fn mck(&self) -> &Mck {
        &self.mck
    }
    #[doc = "0x04 - Pin select for SCK signal"]
    #[inline(always)]
    pub const fn sck(&self) -> &Sck {
        &self.sck
    }
    #[doc = "0x08 - Pin select for LRCK signal"]
    #[inline(always)]
    pub const fn lrck(&self) -> &Lrck {
        &self.lrck
    }
    #[doc = "0x0c - Pin select for SDIN signal"]
    #[inline(always)]
    pub const fn sdin(&self) -> &Sdin {
        &self.sdin
    }
    #[doc = "0x10 - Pin select for SDOUT signal"]
    #[inline(always)]
    pub const fn sdout(&self) -> &Sdout {
        &self.sdout
    }
}
#[doc = "MCK (rw) register accessor: Pin select for MCK signal\n\nYou can [`read`](crate::Reg::read) this register and get [`mck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mck`] module"]
#[doc(alias = "MCK")]
pub type Mck = crate::Reg<mck::MckSpec>;
#[doc = "Pin select for MCK signal"]
pub mod mck;
#[doc = "SCK (rw) register accessor: Pin select for SCK signal\n\nYou can [`read`](crate::Reg::read) this register and get [`sck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck`] module"]
#[doc(alias = "SCK")]
pub type Sck = crate::Reg<sck::SckSpec>;
#[doc = "Pin select for SCK signal"]
pub mod sck;
#[doc = "LRCK (rw) register accessor: Pin select for LRCK signal\n\nYou can [`read`](crate::Reg::read) this register and get [`lrck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrck`] module"]
#[doc(alias = "LRCK")]
pub type Lrck = crate::Reg<lrck::LrckSpec>;
#[doc = "Pin select for LRCK signal"]
pub mod lrck;
#[doc = "SDIN (rw) register accessor: Pin select for SDIN signal\n\nYou can [`read`](crate::Reg::read) this register and get [`sdin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdin`] module"]
#[doc(alias = "SDIN")]
pub type Sdin = crate::Reg<sdin::SdinSpec>;
#[doc = "Pin select for SDIN signal"]
pub mod sdin;
#[doc = "SDOUT (rw) register accessor: Pin select for SDOUT signal\n\nYou can [`read`](crate::Reg::read) this register and get [`sdout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdout`] module"]
#[doc(alias = "SDOUT")]
pub type Sdout = crate::Reg<sdout::SdoutSpec>;
#[doc = "Pin select for SDOUT signal"]
pub mod sdout;
