#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "VREGH")]
pub struct Vregh {
    dcdcen: Dcdcen,
}
impl Vregh {
    #[doc = "0x00 - DC/DC enable register for VREGH"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
}
#[doc = "DCDCEN (rw) register accessor: DC/DC enable register for VREGH\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`] module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "DC/DC enable register for VREGH"]
pub mod dcdcen;
