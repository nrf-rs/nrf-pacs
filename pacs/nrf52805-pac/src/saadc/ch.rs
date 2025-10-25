#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CH")]
pub struct Ch {
    pselp: Pselp,
    pseln: Pseln,
    config: Config,
    limit: Limit,
}
impl Ch {
    #[doc = "0x00 - Description cluster: Input positive pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub const fn pselp(&self) -> &Pselp {
        &self.pselp
    }
    #[doc = "0x04 - Description cluster: Input negative pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub const fn pseln(&self) -> &Pseln {
        &self.pseln
    }
    #[doc = "0x08 - Description cluster: Input configuration for CH\\[n\\]"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x0c - Description cluster: High/low limits for event monitoring a channel"]
    #[inline(always)]
    pub const fn limit(&self) -> &Limit {
        &self.limit
    }
}
#[doc = "PSELP (rw) register accessor: Description cluster: Input positive pin selection for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pselp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselp`] module"]
#[doc(alias = "PSELP")]
pub type Pselp = crate::Reg<pselp::PselpSpec>;
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
pub mod pselp;
#[doc = "PSELN (rw) register accessor: Description cluster: Input negative pin selection for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pseln::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseln::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pseln`] module"]
#[doc(alias = "PSELN")]
pub type Pseln = crate::Reg<pseln::PselnSpec>;
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
pub mod pseln;
#[doc = "CONFIG (rw) register accessor: Description cluster: Input configuration for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description cluster: Input configuration for CH\\[n\\]"]
pub mod config;
#[doc = "LIMIT (rw) register accessor: Description cluster: High/low limits for event monitoring a channel\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit`] module"]
#[doc(alias = "LIMIT")]
pub type Limit = crate::Reg<limit::LimitSpec>;
#[doc = "Description cluster: High/low limits for event monitoring a channel"]
pub mod limit;
