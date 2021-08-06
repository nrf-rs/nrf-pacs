#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800..0x80c - Unspecified"]
    pub acl0: ACL,
    _reserved1: [u8; 0x04],
    #[doc = "0x810..0x81c - Unspecified"]
    pub acl1: ACL,
    _reserved2: [u8; 0x04],
    #[doc = "0x820..0x82c - Unspecified"]
    pub acl2: ACL,
    _reserved3: [u8; 0x04],
    #[doc = "0x830..0x83c - Unspecified"]
    pub acl3: ACL,
    _reserved4: [u8; 0x04],
    #[doc = "0x840..0x84c - Unspecified"]
    pub acl4: ACL,
    _reserved5: [u8; 0x04],
    #[doc = "0x850..0x85c - Unspecified"]
    pub acl5: ACL,
    _reserved6: [u8; 0x04],
    #[doc = "0x860..0x86c - Unspecified"]
    pub acl6: ACL,
    _reserved7: [u8; 0x04],
    #[doc = "0x870..0x87c - Unspecified"]
    pub acl7: ACL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ACL {
    #[doc = "0x00 - Description cluster: Configure the word-aligned start address of region n to protect"]
    pub addr: crate::Reg<self::acl::addr::ADDR_SPEC>,
    #[doc = "0x04 - Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
    pub size: crate::Reg<self::acl::size::SIZE_SPEC>,
    #[doc = "0x08 - Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    pub perm: crate::Reg<self::acl::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod acl;
