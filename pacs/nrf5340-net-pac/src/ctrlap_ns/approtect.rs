#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "APPROTECT")]
pub struct Approtect {
    lock: Lock,
    disable: Disable,
}
impl Approtect {
    #[doc = "0x00 - This register locks the APPROTECT.DISABLE register from being written to until next reset."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x04 - This register disables the APPROTECT register and enables debug access to non-secure mode."]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        &self.disable
    }
}
#[doc = "LOCK (rw) register accessor: This register locks the APPROTECT.DISABLE register from being written to until next reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "This register locks the APPROTECT.DISABLE register from being written to until next reset."]
pub mod lock;
#[doc = "DISABLE (rw) register accessor: This register disables the APPROTECT register and enables debug access to non-secure mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`] module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "This register disables the APPROTECT register and enables debug access to non-secure mode."]
pub mod disable;
