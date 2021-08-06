#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x2000 - Unspecified"]
    pub set: [SET; 256],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SET {
    #[doc = "0x00..0x20 - Unspecified"]
    pub way: [self::set::WAY; 2],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod set;
