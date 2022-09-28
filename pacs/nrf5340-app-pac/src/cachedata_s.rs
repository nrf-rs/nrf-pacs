#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x2000 - Unspecified"]
    pub set: [SET; 256],
}
#[doc = "Unspecified"]
pub use set::SET;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod set;
