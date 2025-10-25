#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    resetreas: Resetreas,
}
impl RegisterBlock {
    #[doc = "0x400 - Reset reason"]
    #[inline(always)]
    pub const fn resetreas(&self) -> &Resetreas {
        &self.resetreas
    }
}
#[doc = "RESETREAS (rw) register accessor: Reset reason\n\nYou can [`read`](crate::Reg::read) this register and get [`resetreas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetreas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetreas`] module"]
#[doc(alias = "RESETREAS")]
pub type Resetreas = crate::Reg<resetreas::ResetreasSpec>;
#[doc = "Reset reason"]
pub mod resetreas;
