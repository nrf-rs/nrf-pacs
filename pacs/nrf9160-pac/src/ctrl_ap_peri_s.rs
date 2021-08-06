#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x488 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 0x78],
    #[doc = "0x500..0x508 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MAILBOX {
    #[doc = "0x00 - Data sent from the debugger to the CPU"]
    pub rxdata: crate::Reg<self::mailbox::rxdata::RXDATA_SPEC>,
    #[doc = "0x04 - Status to indicate if data sent from the debugger to the CPU has been read"]
    pub rxstatus: crate::Reg<self::mailbox::rxstatus::RXSTATUS_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Data sent from the CPU to the debugger"]
    pub txdata: crate::Reg<self::mailbox::txdata::TXDATA_SPEC>,
    #[doc = "0x84 - Status to indicate if data sent from the CPU to the debugger has been read"]
    pub txstatus: crate::Reg<self::mailbox::txstatus::TXSTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = r"Register block"]
#[repr(C)]
pub struct ERASEPROTECT {
    #[doc = "0x00 - Lock register ERASEPROTECT.DISABLE from being written until next reset"]
    pub lock: crate::Reg<self::eraseprotect::lock::LOCK_SPEC>,
    #[doc = "0x04 - Disable ERASEPROTECT and perform ERASEALL"]
    pub disable: crate::Reg<self::eraseprotect::disable::DISABLE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod eraseprotect;
