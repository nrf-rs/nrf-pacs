#[doc = r"Register block"]
#[repr(C)]
pub struct APPROTECT {
    _reserved_0_disable: [u8; 0x04],
}
impl APPROTECT {
    #[doc = "0x00 - Software force APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> &FORCEPROTECT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const FORCEPROTECT) }
    }
    #[doc = "0x00 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> &DISABLE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DISABLE) }
    }
}
#[doc = "DISABLE (rw) register accessor: an alias for `Reg<DISABLE_SPEC>`"]
pub type DISABLE = crate::Reg<disable::DISABLE_SPEC>;
#[doc = "Software disable APPROTECT mechanism"]
pub mod disable;
#[doc = "FORCEPROTECT (rw) register accessor: an alias for `Reg<FORCEPROTECT_SPEC>`"]
pub type FORCEPROTECT = crate::Reg<forceprotect::FORCEPROTECT_SPEC>;
#[doc = "Software force APPROTECT mechanism"]
pub mod forceprotect;
