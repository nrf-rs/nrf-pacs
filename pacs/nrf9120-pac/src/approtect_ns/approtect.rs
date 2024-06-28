#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "APPROTECT")]
pub struct Approtect {
    _reserved_0_disable: [u8; 0x04],
}
impl Approtect {
    #[doc = "0x00 - Software force APPROTECT mechanism"]
    #[inline(always)]
    pub const fn forceprotect(&self) -> &Forceprotect {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
}
#[doc = "DISABLE (rw) register accessor: Software disable APPROTECT mechanism\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`]
module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "Software disable APPROTECT mechanism"]
pub mod disable;
#[doc = "FORCEPROTECT (rw) register accessor: Software force APPROTECT mechanism\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`forceprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`forceprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forceprotect`]
module"]
#[doc(alias = "FORCEPROTECT")]
pub type Forceprotect = crate::Reg<forceprotect::ForceprotectSpec>;
#[doc = "Software force APPROTECT mechanism"]
pub mod forceprotect;
