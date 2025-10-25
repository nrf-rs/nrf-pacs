#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SET")]
pub struct Set {
    way: [Way; 2],
}
impl Set {
    #[doc = "0x00..0x08 - Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn way(&self, n: usize) -> &Way {
        &self.way[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub fn way_iter(&self) -> impl Iterator<Item = &Way> {
        self.way.iter()
    }
}
#[doc = "WAY (rw) register accessor: Description collection: Cache information for SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`way::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`way::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@way`] module"]
#[doc(alias = "WAY")]
pub type Way = crate::Reg<way::WaySpec>;
#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
pub mod way;
