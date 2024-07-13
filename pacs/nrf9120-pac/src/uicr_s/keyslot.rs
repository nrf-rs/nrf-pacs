#[doc = r"Register block"]
#[repr(C)]
pub struct KEYSLOT {
    #[doc = "0x00..0x400 - Unspecified"]
    pub config: [CONFIG; 128],
    #[doc = "0x400..0xc00 - Unspecified"]
    pub key: [KEY; 128],
}
#[doc = "Unspecified"]
pub use config::CONFIG;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
#[doc = "Unspecified"]
pub use key::KEY;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod key;
