#[repr(C)]
#[doc = "Channel group tasks"]
#[doc(alias = "TASKS_CHG")]
pub struct TasksChg {
    en: En,
    dis: Dis,
}
impl TasksChg {
    #[doc = "0x00 - Description cluster\\[0\\]: Enable channel group 0"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x04 - Description cluster\\[0\\]: Disable channel group 0"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
}
#[doc = "EN (w) register accessor: Description cluster\\[0\\]: Enable channel group 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Description cluster\\[0\\]: Enable channel group 0"]
pub mod en;
#[doc = "DIS (w) register accessor: Description cluster\\[0\\]: Disable channel group 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`] module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "Description cluster\\[0\\]: Disable channel group 0"]
pub mod dis;
