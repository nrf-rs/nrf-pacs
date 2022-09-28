#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    #[doc = "0x500..0x50c - Unspecified"]
    pub vregradio: VREGRADIO,
}
#[doc = "Unspecified"]
pub use vregradio::VREGRADIO;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregradio;
