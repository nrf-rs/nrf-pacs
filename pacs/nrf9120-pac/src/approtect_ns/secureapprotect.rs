#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SECUREAPPROTECT")]
pub struct Secureapprotect {
    _reserved_0_disable: [u8; 0x04],
}
impl Secureapprotect {
    #[doc = "0x00 - Software force SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub const fn forceprotect(&self) -> &Forceprotect {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "DISABLE (rw) register accessor: Software disable SECUREAPPROTECT mechanism\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`] module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "Software disable SECUREAPPROTECT mechanism"]
pub mod disable;
#[doc = "FORCEPROTECT (rw) register accessor: Software force SECUREAPPROTECT mechanism\n\nYou can [`read`](crate::Reg::read) this register and get [`forceprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forceprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forceprotect`] module"]
#[doc(alias = "FORCEPROTECT")]
pub type Forceprotect = crate::Reg<forceprotect::ForceprotectSpec>;
#[doc = "Software force SECUREAPPROTECT mechanism"]
pub mod forceprotect;
