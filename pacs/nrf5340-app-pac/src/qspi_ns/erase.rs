#[doc = r"Register block"]
#[repr(C)]
pub struct ERASE {
    #[doc = "0x00 - Start address of flash block to be erased"]
    pub ptr: PTR,
    #[doc = "0x04 - Size of block to be erased."]
    pub len: LEN,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Start address of flash block to be erased"]
pub mod ptr;
#[doc = "LEN (rw) register accessor: an alias for `Reg<LEN_SPEC>`"]
pub type LEN = crate::Reg<len::LEN_SPEC>;
#[doc = "Size of block to be erased."]
pub mod len;
