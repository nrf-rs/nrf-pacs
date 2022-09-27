#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800..0x880 - Unspecified"]
    pub acl: [ACL; 8],
}
#[doc = "Unspecified"]
pub use acl::ACL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod acl;
