#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0550],
    forceprotect: Forceprotect,
    _reserved1: [u8; 0x04],
    disable: Disable,
}
impl RegisterBlock {
    #[doc = "0x550 - Software force enable APPROTECT mechanism until next reset."]
    #[inline(always)]
    pub const fn forceprotect(&self) -> &Forceprotect {
        &self.forceprotect
    }
    #[doc = "0x558 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        &self.disable
    }
}
#[doc = "FORCEPROTECT (rw) register accessor: Software force enable APPROTECT mechanism until next reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`forceprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forceprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forceprotect`] module"]
#[doc(alias = "FORCEPROTECT")]
pub type Forceprotect = crate::Reg<forceprotect::ForceprotectSpec>;
#[doc = "Software force enable APPROTECT mechanism until next reset."]
pub mod forceprotect;
#[doc = "DISABLE (rw) register accessor: Software disable APPROTECT mechanism\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`] module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "Software disable APPROTECT mechanism"]
pub mod disable;
