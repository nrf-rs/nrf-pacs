#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e00],
    #[doc = "0xe00 - Unspecified"]
    pub secureapprotect: SECUREAPPROTECT,
    _reserved1: [u8; 0x0c],
    #[doc = "0xe10 - Unspecified"]
    pub approtect: APPROTECT,
}
#[doc = "Unspecified"]
pub use secureapprotect::SECUREAPPROTECT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod secureapprotect;
#[doc = "Unspecified"]
pub use approtect::APPROTECT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod approtect;
