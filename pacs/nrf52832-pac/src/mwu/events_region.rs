#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "EVENTS_REGION")]
pub struct EventsRegion {
    wa: Wa,
    ra: Ra,
}
impl EventsRegion {
    #[doc = "0x00 - Description cluster\\[0\\]: Write access to region 0 detected"]
    #[inline(always)]
    pub const fn wa(&self) -> &Wa {
        &self.wa
    }
    #[doc = "0x04 - Description cluster\\[0\\]: Read access to region 0 detected"]
    #[inline(always)]
    pub const fn ra(&self) -> &Ra {
        &self.ra
    }
}
#[doc = "WA (rw) register accessor: Description cluster\\[0\\]: Write access to region 0 detected\n\nYou can [`read`](crate::Reg::read) this register and get [`wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wa`] module"]
#[doc(alias = "WA")]
pub type Wa = crate::Reg<wa::WaSpec>;
#[doc = "Description cluster\\[0\\]: Write access to region 0 detected"]
pub mod wa;
#[doc = "RA (rw) register accessor: Description cluster\\[0\\]: Read access to region 0 detected\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra`] module"]
#[doc(alias = "RA")]
pub type Ra = crate::Reg<ra::RaSpec>;
#[doc = "Description cluster\\[0\\]: Read access to region 0 detected"]
pub mod ra;
