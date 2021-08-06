#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800..0x880 - Unspecified"]
    pub acl: [ACL; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ACL {
    #[doc = "0x00 - Description cluster\\[n\\]: Configure the word-aligned start address of region n to protect"]
    pub addr: crate::Reg<self::acl::addr::ADDR_SPEC>,
    #[doc = "0x04 - Description cluster\\[n\\]: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
    pub size: crate::Reg<self::acl::size::SIZE_SPEC>,
    #[doc = "0x08 - Description cluster\\[n\\]: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    pub perm: crate::Reg<self::acl::perm::PERM_SPEC>,
    #[doc = "0x0c - Unspecified"]
    pub unused0: crate::Reg<self::acl::unused0::UNUSED0_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod acl;
