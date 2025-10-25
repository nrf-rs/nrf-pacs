#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x05c4],
    xosc32mcaps: Xosc32mcaps,
    _reserved1: [u8; 0xf8],
    xosc32ki: Xosc32ki,
}
impl RegisterBlock {
    #[doc = "0x5c4 - Programmable capacitance of XC1 and XC2"]
    #[inline(always)]
    pub const fn xosc32mcaps(&self) -> &Xosc32mcaps {
        &self.xosc32mcaps
    }
    #[doc = "0x6c0..0x6d4 - Unspecified"]
    #[inline(always)]
    pub const fn xosc32ki(&self) -> &Xosc32ki {
        &self.xosc32ki
    }
}
#[doc = "XOSC32MCAPS (rw) register accessor: Programmable capacitance of XC1 and XC2\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32mcaps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32mcaps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32mcaps`] module"]
#[doc(alias = "XOSC32MCAPS")]
pub type Xosc32mcaps = crate::Reg<xosc32mcaps::Xosc32mcapsSpec>;
#[doc = "Programmable capacitance of XC1 and XC2"]
pub mod xosc32mcaps;
#[doc = "Unspecified"]
pub use self::xosc32ki::Xosc32ki;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod xosc32ki;
