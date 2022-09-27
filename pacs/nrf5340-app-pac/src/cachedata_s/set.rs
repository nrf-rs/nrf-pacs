#[doc = r"Register block"]
#[repr(C)]
pub struct SET {
    #[doc = "0x00..0x20 - Unspecified"]
    pub way: [WAY; 2],
}
#[doc = "Unspecified"]
pub use way::WAY;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod way;
