#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    config0: Config0,
    config1: Config1,
    disableindebug: Disableindebug,
}
impl RegisterBlock {
    #[doc = "0x600 - Block protect configuration register 0"]
    #[inline(always)]
    pub const fn config0(&self) -> &Config0 {
        &self.config0
    }
    #[doc = "0x604 - Block protect configuration register 1"]
    #[inline(always)]
    pub const fn config1(&self) -> &Config1 {
        &self.config1
    }
    #[doc = "0x608 - Disable protection mechanism in debug mode"]
    #[inline(always)]
    pub const fn disableindebug(&self) -> &Disableindebug {
        &self.disableindebug
    }
}
#[doc = "CONFIG0 (rw) register accessor: Block protect configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`] module"]
#[doc(alias = "CONFIG0")]
pub type Config0 = crate::Reg<config0::Config0Spec>;
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: Block protect configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`] module"]
#[doc(alias = "CONFIG1")]
pub type Config1 = crate::Reg<config1::Config1Spec>;
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "DISABLEINDEBUG (rw) register accessor: Disable protection mechanism in debug mode\n\nYou can [`read`](crate::Reg::read) this register and get [`disableindebug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disableindebug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disableindebug`] module"]
#[doc(alias = "DISABLEINDEBUG")]
pub type Disableindebug = crate::Reg<disableindebug::DisableindebugSpec>;
#[doc = "Disable protection mechanism in debug mode"]
pub mod disableindebug;
