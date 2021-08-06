#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x800 - Unspecified"]
    pub set: [SET; 256],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SET {
    #[doc = "0x00..0x08 - Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
    pub way: [crate::Reg<self::set::way::WAY_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod set;
