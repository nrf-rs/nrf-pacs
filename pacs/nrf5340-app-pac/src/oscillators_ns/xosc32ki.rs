#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "XOSC32KI")]
pub struct Xosc32ki {
    bypass: Bypass,
    _reserved1: [u8; 0x0c],
    intcap: Intcap,
}
impl Xosc32ki {
    #[doc = "0x00 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub const fn bypass(&self) -> &Bypass {
        &self.bypass
    }
    #[doc = "0x10 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub const fn intcap(&self) -> &Intcap {
        &self.intcap
    }
}
#[doc = "BYPASS (rw) register accessor: Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nYou can [`read`](crate::Reg::read) this register and get [`bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bypass`] module"]
#[doc(alias = "BYPASS")]
pub type Bypass = crate::Reg<bypass::BypassSpec>;
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub mod bypass;
#[doc = "INTCAP (rw) register accessor: Control usage of internal load capacitors\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcap`] module"]
#[doc(alias = "INTCAP")]
pub type Intcap = crate::Reg<intcap::IntcapSpec>;
#[doc = "Control usage of internal load capacitors"]
pub mod intcap;
