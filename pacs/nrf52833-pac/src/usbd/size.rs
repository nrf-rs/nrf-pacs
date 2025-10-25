#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SIZE")]
pub struct Size {
    epout: [Epout; 8],
    isoout: Isoout,
}
impl Size {
    #[doc = "0x00..0x20 - Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub const fn epout(&self, n: usize) -> &Epout {
        &self.epout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn epout_iter(&self) -> impl Iterator<Item = &Epout> {
        self.epout.iter()
    }
    #[doc = "0x20 - Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub const fn isoout(&self) -> &Isoout {
        &self.isoout
    }
}
#[doc = "EPOUT (rw) register accessor: Description collection: Number of bytes received last in the data stage of this OUT endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`epout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epout`] module"]
#[doc(alias = "EPOUT")]
pub type Epout = crate::Reg<epout::EpoutSpec>;
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
pub mod epout;
#[doc = "ISOOUT (r) register accessor: Number of bytes received last on this ISO OUT data endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`isoout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoout`] module"]
#[doc(alias = "ISOOUT")]
pub type Isoout = crate::Reg<isoout::IsooutSpec>;
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub mod isoout;
