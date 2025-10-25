#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "SUBSCRIBE_CHG")]
pub struct SubscribeChg {
    en: En,
    dis: Dis,
}
impl SubscribeChg {
    #[doc = "0x00 - Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x04 - Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
}
#[doc = "EN (rw) register accessor: Description cluster: Subscribe configuration for task CHG\\[n\\].EN\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
pub mod en;
#[doc = "DIS (rw) register accessor: Description cluster: Subscribe configuration for task CHG\\[n\\].DIS\n\nYou can [`read`](crate::Reg::read) this register and get [`dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`] module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
pub mod dis;
