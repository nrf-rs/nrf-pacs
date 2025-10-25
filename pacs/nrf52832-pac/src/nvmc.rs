#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    ready: Ready,
    _reserved1: [u8; 0x0100],
    config: Config,
    _reserved_2_erasepage: [u8; 0x04],
    eraseall: Eraseall,
    erasepcr0: Erasepcr0,
    eraseuicr: Eraseuicr,
    _reserved6: [u8; 0x28],
    icachecnf: Icachecnf,
    _reserved7: [u8; 0x04],
    ihit: Ihit,
    imiss: Imiss,
}
impl RegisterBlock {
    #[doc = "0x400 - Ready flag"]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x504 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x508 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub const fn erasepcr1(&self) -> &Erasepcr1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x508 - Register for erasing a page in Code area"]
    #[inline(always)]
    pub const fn erasepage(&self) -> &Erasepage {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub const fn eraseall(&self) -> &Eraseall {
        &self.eraseall
    }
    #[doc = "0x510 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub const fn erasepcr0(&self) -> &Erasepcr0 {
        &self.erasepcr0
    }
    #[doc = "0x514 - Register for erasing User Information Configuration Registers"]
    #[inline(always)]
    pub const fn eraseuicr(&self) -> &Eraseuicr {
        &self.eraseuicr
    }
    #[doc = "0x540 - I-Code cache configuration register."]
    #[inline(always)]
    pub const fn icachecnf(&self) -> &Icachecnf {
        &self.icachecnf
    }
    #[doc = "0x548 - I-Code cache hit counter."]
    #[inline(always)]
    pub const fn ihit(&self) -> &Ihit {
        &self.ihit
    }
    #[doc = "0x54c - I-Code cache miss counter."]
    #[inline(always)]
    pub const fn imiss(&self) -> &Imiss {
        &self.imiss
    }
}
#[doc = "READY (r) register accessor: Ready flag\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`] module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEPAGE (rw) register accessor: Register for erasing a page in Code area\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepage::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepage::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepage`] module"]
#[doc(alias = "ERASEPAGE")]
pub type Erasepage = crate::Reg<erasepage::ErasepageSpec>;
#[doc = "Register for erasing a page in Code area"]
pub mod erasepage;
#[doc = "ERASEPCR1 (rw) register accessor: Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepcr1`] module"]
#[doc(alias = "ERASEPCR1")]
pub type Erasepcr1 = crate::Reg<erasepcr1::Erasepcr1Spec>;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "ERASEALL (rw) register accessor: Register for erasing all non-volatile user memory\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseall`] module"]
#[doc(alias = "ERASEALL")]
pub type Eraseall = crate::Reg<eraseall::EraseallSpec>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPCR0 (rw) register accessor: Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepcr0`] module"]
#[doc(alias = "ERASEPCR0")]
pub type Erasepcr0 = crate::Reg<erasepcr0::Erasepcr0Spec>;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "ERASEUICR (rw) register accessor: Register for erasing User Information Configuration Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseuicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseuicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseuicr`] module"]
#[doc(alias = "ERASEUICR")]
pub type Eraseuicr = crate::Reg<eraseuicr::EraseuicrSpec>;
#[doc = "Register for erasing User Information Configuration Registers"]
pub mod eraseuicr;
#[doc = "ICACHECNF (rw) register accessor: I-Code cache configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`icachecnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icachecnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icachecnf`] module"]
#[doc(alias = "ICACHECNF")]
pub type Icachecnf = crate::Reg<icachecnf::IcachecnfSpec>;
#[doc = "I-Code cache configuration register."]
pub mod icachecnf;
#[doc = "IHIT (rw) register accessor: I-Code cache hit counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`ihit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihit`] module"]
#[doc(alias = "IHIT")]
pub type Ihit = crate::Reg<ihit::IhitSpec>;
#[doc = "I-Code cache hit counter."]
pub mod ihit;
#[doc = "IMISS (rw) register accessor: I-Code cache miss counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`imiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imiss`] module"]
#[doc(alias = "IMISS")]
pub type Imiss = crate::Reg<imiss::ImissSpec>;
#[doc = "I-Code cache miss counter."]
pub mod imiss;
