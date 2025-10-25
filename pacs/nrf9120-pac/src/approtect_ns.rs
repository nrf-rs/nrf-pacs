#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e00],
    secureapprotect: Secureapprotect,
    _reserved1: [u8; 0x0c],
    approtect: Approtect,
}
impl RegisterBlock {
    #[doc = "0xe00 - Unspecified"]
    #[inline(always)]
    pub const fn secureapprotect(&self) -> &Secureapprotect {
        &self.secureapprotect
    }
    #[doc = "0xe10 - Unspecified"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
}
#[doc = "Unspecified"]
pub use self::secureapprotect::Secureapprotect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod secureapprotect;
#[doc = "Unspecified"]
pub use self::approtect::Approtect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod approtect;
