#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    sck: Sck,
    csn: Csn,
    _reserved2: [u8; 0x04],
    io0: Io0,
    io1: Io1,
    io2: Io2,
    io3: Io3,
}
impl Psel {
    #[doc = "0x00 - Pin select for serial clock SCK"]
    #[inline(always)]
    pub const fn sck(&self) -> &Sck {
        &self.sck
    }
    #[doc = "0x04 - Pin select for chip select signal CSN."]
    #[inline(always)]
    pub const fn csn(&self) -> &Csn {
        &self.csn
    }
    #[doc = "0x0c - Pin select for serial data MOSI/IO0."]
    #[inline(always)]
    pub const fn io0(&self) -> &Io0 {
        &self.io0
    }
    #[doc = "0x10 - Pin select for serial data MISO/IO1."]
    #[inline(always)]
    pub const fn io1(&self) -> &Io1 {
        &self.io1
    }
    #[doc = "0x14 - Pin select for serial data WP/IO2."]
    #[inline(always)]
    pub const fn io2(&self) -> &Io2 {
        &self.io2
    }
    #[doc = "0x18 - Pin select for serial data HOLD/IO3."]
    #[inline(always)]
    pub const fn io3(&self) -> &Io3 {
        &self.io3
    }
}
#[doc = "SCK (rw) register accessor: Pin select for serial clock SCK\n\nYou can [`read`](crate::Reg::read) this register and get [`sck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sck`] module"]
#[doc(alias = "SCK")]
pub type Sck = crate::Reg<sck::SckSpec>;
#[doc = "Pin select for serial clock SCK"]
pub mod sck;
#[doc = "CSN (rw) register accessor: Pin select for chip select signal CSN.\n\nYou can [`read`](crate::Reg::read) this register and get [`csn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csn`] module"]
#[doc(alias = "CSN")]
pub type Csn = crate::Reg<csn::CsnSpec>;
#[doc = "Pin select for chip select signal CSN."]
pub mod csn;
#[doc = "IO0 (rw) register accessor: Pin select for serial data MOSI/IO0.\n\nYou can [`read`](crate::Reg::read) this register and get [`io0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io0`] module"]
#[doc(alias = "IO0")]
pub type Io0 = crate::Reg<io0::Io0Spec>;
#[doc = "Pin select for serial data MOSI/IO0."]
pub mod io0;
#[doc = "IO1 (rw) register accessor: Pin select for serial data MISO/IO1.\n\nYou can [`read`](crate::Reg::read) this register and get [`io1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io1`] module"]
#[doc(alias = "IO1")]
pub type Io1 = crate::Reg<io1::Io1Spec>;
#[doc = "Pin select for serial data MISO/IO1."]
pub mod io1;
#[doc = "IO2 (rw) register accessor: Pin select for serial data WP/IO2.\n\nYou can [`read`](crate::Reg::read) this register and get [`io2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io2`] module"]
#[doc(alias = "IO2")]
pub type Io2 = crate::Reg<io2::Io2Spec>;
#[doc = "Pin select for serial data WP/IO2."]
pub mod io2;
#[doc = "IO3 (rw) register accessor: Pin select for serial data HOLD/IO3.\n\nYou can [`read`](crate::Reg::read) this register and get [`io3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io3`] module"]
#[doc(alias = "IO3")]
pub type Io3 = crate::Reg<io3::Io3Spec>;
#[doc = "Pin select for serial data HOLD/IO3."]
pub mod io3;
