#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    info: Info,
    _reserved1: [u8; 0xd4],
    trimcnf: [Trimcnf; 32],
    _reserved2: [u8; 0x50],
    nfc: Nfc,
    _reserved3: [u8; 0x07a0],
    trng90b: Trng90b,
    xosc32mtrim: Xosc32mtrim,
}
impl RegisterBlock {
    #[doc = "0x200..0x22c - Device info"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x300..0x400 - Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(&self, n: usize) -> &Trimcnf {
        &self.trimcnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x400 - Unspecified"]
    #[inline(always)]
    pub fn trimcnf_iter(&self) -> impl Iterator<Item = &Trimcnf> {
        self.trimcnf.iter()
    }
    #[doc = "0x450..0x460 - Unspecified"]
    #[inline(always)]
    pub const fn nfc(&self) -> &Nfc {
        &self.nfc
    }
    #[doc = "0xc00..0xc20 - NIST800-90B RNG calibration data"]
    #[inline(always)]
    pub const fn trng90b(&self) -> &Trng90b {
        &self.trng90b
    }
    #[doc = "0xc20 - XOSC32M capacitor selection trim values"]
    #[inline(always)]
    pub const fn xosc32mtrim(&self) -> &Xosc32mtrim {
        &self.xosc32mtrim
    }
}
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
#[doc = "Unspecified"]
pub use self::nfc::Nfc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = "NIST800-90B RNG calibration data"]
pub use self::trng90b::Trng90b;
#[doc = r"Cluster"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
#[doc = "XOSC32MTRIM (r) register accessor: XOSC32M capacitor selection trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32mtrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32mtrim`] module"]
#[doc(alias = "XOSC32MTRIM")]
pub type Xosc32mtrim = crate::Reg<xosc32mtrim::Xosc32mtrimSpec>;
#[doc = "XOSC32M capacitor selection trim values"]
pub mod xosc32mtrim;
