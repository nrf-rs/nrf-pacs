#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Ready flag."]
    pub ready: crate::Reg<ready::READY_SPEC>,
    _reserved1: [u8; 0x0100],
    #[doc = "0x504 - Configuration register."]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved_2_erasepage: [u8; 0x04],
    #[doc = "0x50c - Register for erasing all non-volatile user memory."]
    pub eraseall: crate::Reg<eraseall::ERASEALL_SPEC>,
    #[doc = "0x510 - Register for erasing a protected non-volatile memory page."]
    pub erasepcr0: crate::Reg<erasepcr0::ERASEPCR0_SPEC>,
    #[doc = "0x514 - Register for start erasing User Information Congfiguration Registers."]
    pub eraseuicr: crate::Reg<eraseuicr::ERASEUICR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepcr1(&self) -> &crate::Reg<erasepcr1::ERASEPCR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<erasepcr1::ERASEPCR1_SPEC>)
        }
    }
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepage(&self) -> &crate::Reg<erasepage::ERASEPAGE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<erasepage::ERASEPAGE_SPEC>)
        }
    }
}
#[doc = "READY register accessor: an alias for `Reg<READY_SPEC>`"]
pub type READY = crate::Reg<ready::READY_SPEC>;
#[doc = "Ready flag."]
pub mod ready;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "ERASEPAGE register accessor: an alias for `Reg<ERASEPAGE_SPEC>`"]
pub type ERASEPAGE = crate::Reg<erasepage::ERASEPAGE_SPEC>;
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepage;
#[doc = "ERASEPCR1 register accessor: an alias for `Reg<ERASEPCR1_SPEC>`"]
pub type ERASEPCR1 = crate::Reg<erasepcr1::ERASEPCR1_SPEC>;
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepcr1;
#[doc = "ERASEALL register accessor: an alias for `Reg<ERASEALL_SPEC>`"]
pub type ERASEALL = crate::Reg<eraseall::ERASEALL_SPEC>;
#[doc = "Register for erasing all non-volatile user memory."]
pub mod eraseall;
#[doc = "ERASEPCR0 register accessor: an alias for `Reg<ERASEPCR0_SPEC>`"]
pub type ERASEPCR0 = crate::Reg<erasepcr0::ERASEPCR0_SPEC>;
#[doc = "Register for erasing a protected non-volatile memory page."]
pub mod erasepcr0;
#[doc = "ERASEUICR register accessor: an alias for `Reg<ERASEUICR_SPEC>`"]
pub type ERASEUICR = crate::Reg<eraseuicr::ERASEUICR_SPEC>;
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub mod eraseuicr;
