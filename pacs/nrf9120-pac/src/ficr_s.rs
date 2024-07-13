#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0140],
    #[doc = "0x140..0x14c - SIP-specific device info"]
    pub sipinfo: SIPINFO,
    _reserved1: [u8; 0xb4],
    #[doc = "0x200..0x22c - Device info"]
    pub info: INFO,
    _reserved2: [u8; 0xd4],
    #[doc = "0x300..0xb00 - Unspecified"]
    pub trimcnf: [TRIMCNF; 256],
    _reserved3: [u8; 0x0100],
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
}
#[doc = "SIP-specific device info"]
pub use sipinfo::SIPINFO;
#[doc = r"Cluster"]
#[doc = "SIP-specific device info"]
pub mod sipinfo;
#[doc = "Device info"]
pub use info::INFO;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "Unspecified"]
pub use trimcnf::TRIMCNF;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = "NIST800-90B RNG calibration data"]
pub use trng90b::TRNG90B;
#[doc = r"Cluster"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
