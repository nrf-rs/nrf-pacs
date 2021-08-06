#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x440 - Description collection: Mutex register"]
    pub mutex: [crate::Reg<mutex::MUTEX_SPEC>; 16],
}
#[doc = "MUTEX register accessor: an alias for `Reg<MUTEX_SPEC>`"]
pub type MUTEX = crate::Reg<mutex::MUTEX_SPEC>;
#[doc = "Description collection: Mutex register"]
pub mod mutex;
