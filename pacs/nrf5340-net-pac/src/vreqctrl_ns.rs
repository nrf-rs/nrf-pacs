#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    vregradio: Vregradio,
}
impl RegisterBlock {
    #[doc = "0x500..0x50c - Unspecified"]
    #[inline(always)]
    pub const fn vregradio(&self) -> &Vregradio {
        &self.vregradio
    }
}
#[doc = "Unspecified"]
pub use self::vregradio::Vregradio;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregradio;
