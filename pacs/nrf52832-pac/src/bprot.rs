#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    config0: Config0,
    config1: Config1,
    disableindebug: Disableindebug,
    unused0: Unused0,
    config2: Config2,
    config3: Config3,
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
    #[doc = "0x608 - Disable protection mechanism in debug interface mode"]
    #[inline(always)]
    pub const fn disableindebug(&self) -> &Disableindebug {
        &self.disableindebug
    }
    #[doc = "0x60c - Unspecified"]
    #[inline(always)]
    pub const fn unused0(&self) -> &Unused0 {
        &self.unused0
    }
    #[doc = "0x610 - Block protect configuration register 2"]
    #[inline(always)]
    pub const fn config2(&self) -> &Config2 {
        &self.config2
    }
    #[doc = "0x614 - Block protect configuration register 3"]
    #[inline(always)]
    pub const fn config3(&self) -> &Config3 {
        &self.config3
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
#[doc = "DISABLEINDEBUG (rw) register accessor: Disable protection mechanism in debug interface mode\n\nYou can [`read`](crate::Reg::read) this register and get [`disableindebug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disableindebug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disableindebug`] module"]
#[doc(alias = "DISABLEINDEBUG")]
pub type Disableindebug = crate::Reg<disableindebug::DisableindebugSpec>;
#[doc = "Disable protection mechanism in debug interface mode"]
pub mod disableindebug;
#[doc = "UNUSED0 (rw) register accessor: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused0`] module"]
#[doc(alias = "UNUSED0")]
pub type Unused0 = crate::Reg<unused0::Unused0Spec>;
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "CONFIG2 (rw) register accessor: Block protect configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config2`] module"]
#[doc(alias = "CONFIG2")]
pub type Config2 = crate::Reg<config2::Config2Spec>;
#[doc = "Block protect configuration register 2"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: Block protect configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`config3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config3`] module"]
#[doc(alias = "CONFIG3")]
pub type Config3 = crate::Reg<config3::Config3Spec>;
#[doc = "Block protect configuration register 3"]
pub mod config3;
