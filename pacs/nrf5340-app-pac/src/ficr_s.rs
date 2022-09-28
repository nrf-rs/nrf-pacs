#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200..0x22c - Device info"]
    pub info: INFO,
    _reserved1: [u8; 0xd4],
    #[doc = "0x300..0x400 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
    _reserved2: [u8; 0x50],
    #[doc = "0x450..0x460 - Unspecified"]
    pub nfc: NFC,
    _reserved3: [u8; 0x07a0],
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
    #[doc = "0xc20 - XOSC32M capacitor selection trim values"]
    pub xosc32mtrim: XOSC32MTRIM,
}
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
#[doc = "Unspecified"]
pub use nfc::NFC;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = "NIST800-90B RNG calibration data"]
pub use trng90b::TRNG90B;
#[doc = r"Cluster"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
#[doc = "XOSC32MTRIM (r) register accessor: an alias for `Reg<XOSC32MTRIM_SPEC>`"]
pub type XOSC32MTRIM = crate::Reg<xosc32mtrim::XOSC32MTRIM_SPEC>;
#[doc = "XOSC32M capacitor selection trim values"]
pub mod xosc32mtrim;
