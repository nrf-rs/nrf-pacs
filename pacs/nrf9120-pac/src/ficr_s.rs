#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0140],
    sipinfo: Sipinfo,
    _reserved1: [u8; 0xb4],
    info: Info,
    _reserved2: [u8; 0xd4],
    trimcnf: [Trimcnf; 256],
    _reserved3: [u8; 0x0100],
    trng90b: Trng90b,
}
impl RegisterBlock {
    #[doc = "0x140..0x14c - SIP-specific device info"]
    #[inline(always)]
    pub const fn sipinfo(&self) -> &Sipinfo {
        &self.sipinfo
    }
    #[doc = "0x200..0x22c - Device info"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x300..0xb00 - Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(&self, n: usize) -> &Trimcnf {
        &self.trimcnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0xb00 - Unspecified"]
    #[inline(always)]
    pub fn trimcnf_iter(&self) -> impl Iterator<Item = &Trimcnf> {
        self.trimcnf.iter()
    }
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    #[inline(always)]
    pub const fn trng90b(&self) -> &Trng90b {
        &self.trng90b
    }
}
#[doc = "SIP-specific device info"]
pub use self::sipinfo::Sipinfo;
#[doc = r"Cluster"]
#[doc = "SIP-specific device info"]
pub mod sipinfo;
#[doc = "Device info"]
pub use self::info::Info;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "Unspecified"]
pub use self::trimcnf::Trimcnf;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = "NIST800-90B RNG calibration data"]
pub use self::trng90b::Trng90b;
#[doc = r"Cluster"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
