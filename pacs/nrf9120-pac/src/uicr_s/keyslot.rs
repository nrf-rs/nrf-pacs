#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "KEYSLOT")]
pub struct Keyslot {
    config: [Config; 128],
    key: [Key; 128],
}
impl Keyslot {
    #[doc = "0x00..0x400 - Unspecified"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x400 - Unspecified"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
    #[doc = "0x400..0xc00 - Unspecified"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0xc00 - Unspecified"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
}
#[doc = "Unspecified"]
pub use self::config::Config;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
#[doc = "Unspecified"]
pub use self::key::Key;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod key;
