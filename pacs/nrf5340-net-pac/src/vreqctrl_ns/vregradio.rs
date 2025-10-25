#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "VREGRADIO")]
pub struct Vregradio {
    vreqh: Vreqh,
    _reserved1: [u8; 0x04],
    vreqhready: Vreqhready,
}
impl Vregradio {
    #[doc = "0x00 - Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
    #[inline(always)]
    pub const fn vreqh(&self) -> &Vreqh {
        &self.vreqh
    }
    #[doc = "0x08 - High voltage on RADIO is ready"]
    #[inline(always)]
    pub const fn vreqhready(&self) -> &Vreqhready {
        &self.vreqhready
    }
}
#[doc = "VREQH (rw) register accessor: Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready\n\nYou can [`read`](crate::Reg::read) this register and get [`vreqh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreqh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreqh`] module"]
#[doc(alias = "VREQH")]
pub type Vreqh = crate::Reg<vreqh::VreqhSpec>;
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
pub mod vreqh;
#[doc = "VREQHREADY (r) register accessor: High voltage on RADIO is ready\n\nYou can [`read`](crate::Reg::read) this register and get [`vreqhready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreqhready`] module"]
#[doc(alias = "VREQHREADY")]
pub type Vreqhready = crate::Reg<vreqhready::VreqhreadySpec>;
#[doc = "High voltage on RADIO is ready"]
pub mod vreqhready;
