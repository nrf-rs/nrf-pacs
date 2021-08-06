#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x488 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 0x78],
    #[doc = "0x500..0x508 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
    _reserved2: [u8; 0x38],
    #[doc = "0x540..0x548 - Unspecified"]
    pub approtect: APPROTECT,
    #[doc = "0x548..0x550 - Unspecified"]
    pub secureapprotect: SECUREAPPROTECT,
    _reserved4: [u8; 0xb0],
    #[doc = "0x600 - Status bits for CTRL-AP peripheral."]
    pub status: crate::Reg<status::STATUS_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MAILBOX {
    #[doc = "0x00 - Data sent from the debugger to the CPU."]
    pub rxdata: crate::Reg<self::mailbox::rxdata::RXDATA_SPEC>,
    #[doc = "0x04 - This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
    pub rxstatus: crate::Reg<self::mailbox::rxstatus::RXSTATUS_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Data sent from the CPU to the debugger."]
    pub txdata: crate::Reg<self::mailbox::txdata::TXDATA_SPEC>,
    #[doc = "0x84 - This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
    pub txstatus: crate::Reg<self::mailbox::txstatus::TXSTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = r"Register block"]
#[repr(C)]
pub struct ERASEPROTECT {
    #[doc = "0x00 - This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
    pub lock: crate::Reg<self::eraseprotect::lock::LOCK_SPEC>,
    #[doc = "0x04 - This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
    pub disable: crate::Reg<self::eraseprotect::disable::DISABLE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod eraseprotect;
#[doc = r"Register block"]
#[repr(C)]
pub struct APPROTECT {
    #[doc = "0x00 - This register locks the APPROTECT.DISABLE register from being written to until next reset."]
    pub lock: crate::Reg<self::approtect::lock::LOCK_SPEC>,
    #[doc = "0x04 - This register disables the APPROTECT register and enables debug access to non-secure mode."]
    pub disable: crate::Reg<self::approtect::disable::DISABLE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod approtect;
#[doc = r"Register block"]
#[repr(C)]
pub struct SECUREAPPROTECT {
    #[doc = "0x00 - This register locks the SECUREAPPROTECT.DISABLE register from being written until next reset."]
    pub lock: crate::Reg<self::secureapprotect::lock::LOCK_SPEC>,
    #[doc = "0x04 - This register disables the SECUREAPPROTECT register and enables debug access to secure mode."]
    pub disable: crate::Reg<self::secureapprotect::disable::DISABLE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod secureapprotect;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status bits for CTRL-AP peripheral."]
pub mod status;
