#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    sck: Sck,
    mosi: Mosi,
    miso: Miso,
}
impl Psel {
    #[doc = "0x00 - Pin select for SCK"]
    #[inline(always)]
    pub const fn sck(&self) -> &Sck {
        &self.sck
    }
    #[doc = "0x04 - Pin select for MOSI"]
    #[inline(always)]
    pub const fn mosi(&self) -> &Mosi {
        &self.mosi
    }
    #[doc = "0x08 - Pin select for MISO"]
    #[inline(always)]
    pub const fn miso(&self) -> &Miso {
        &self.miso
    }
}
#[doc = "SCK (rw) register accessor: Pin select for SCK\n\nYou can [`read`](crate::Reg::read) this register and get [`sck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck`] module"]
#[doc(alias = "SCK")]
pub type Sck = crate::Reg<sck::SckSpec>;
#[doc = "Pin select for SCK"]
pub mod sck;
#[doc = "MOSI (rw) register accessor: Pin select for MOSI\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi`] module"]
#[doc(alias = "MOSI")]
pub type Mosi = crate::Reg<mosi::MosiSpec>;
#[doc = "Pin select for MOSI"]
pub mod mosi;
#[doc = "MISO (rw) register accessor: Pin select for MISO\n\nYou can [`read`](crate::Reg::read) this register and get [`miso::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso`] module"]
#[doc(alias = "MISO")]
pub type Miso = crate::Reg<miso::MisoSpec>;
#[doc = "Pin select for MISO"]
pub mod miso;
