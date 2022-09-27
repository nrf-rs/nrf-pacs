#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x800 - Unspecified"]
    pub set: [SET; 256],
}
#[doc = "Unspecified"]
pub use set::SET;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod set;
