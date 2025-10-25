#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    ready: Ready,
    _reserved1: [u8; 0x04],
    readynext: Readynext,
    _reserved2: [u8; 0xf8],
    config: Config,
    _reserved_3_erasepage: [u8; 0x04],
    eraseall: Eraseall,
    erasepcr0: Erasepcr0,
    eraseuicr: Eraseuicr,
    erasepagepartial: Erasepagepartial,
    erasepagepartialcfg: Erasepagepartialcfg,
    _reserved9: [u8; 0x20],
    icachecnf: Icachecnf,
    _reserved10: [u8; 0x04],
    ihit: Ihit,
    imiss: Imiss,
}
impl RegisterBlock {
    #[doc = "0x400 - Ready flag"]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x408 - Ready flag"]
    #[inline(always)]
    pub const fn readynext(&self) -> &Readynext {
        &self.readynext
    }
    #[doc = "0x504 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x508 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub const fn erasepcr1(&self) -> &Erasepcr1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x508 - Register for erasing a page in code area"]
    #[inline(always)]
    pub const fn erasepage(&self) -> &Erasepage {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub const fn eraseall(&self) -> &Eraseall {
        &self.eraseall
    }
    #[doc = "0x510 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub const fn erasepcr0(&self) -> &Erasepcr0 {
        &self.erasepcr0
    }
    #[doc = "0x514 - Register for erasing user information configuration registers"]
    #[inline(always)]
    pub const fn eraseuicr(&self) -> &Eraseuicr {
        &self.eraseuicr
    }
    #[doc = "0x518 - Register for partial erase of a page in code area"]
    #[inline(always)]
    pub const fn erasepagepartial(&self) -> &Erasepagepartial {
        &self.erasepagepartial
    }
    #[doc = "0x51c - Register for partial erase configuration"]
    #[inline(always)]
    pub const fn erasepagepartialcfg(&self) -> &Erasepagepartialcfg {
        &self.erasepagepartialcfg
    }
    #[doc = "0x540 - I-code cache configuration register"]
    #[inline(always)]
    pub const fn icachecnf(&self) -> &Icachecnf {
        &self.icachecnf
    }
    #[doc = "0x548 - I-code cache hit counter"]
    #[inline(always)]
    pub const fn ihit(&self) -> &Ihit {
        &self.ihit
    }
    #[doc = "0x54c - I-code cache miss counter"]
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
#[doc = "READYNEXT (r) register accessor: Ready flag\n\nYou can [`read`](crate::Reg::read) this register and get [`readynext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readynext`] module"]
#[doc(alias = "READYNEXT")]
pub type Readynext = crate::Reg<readynext::ReadynextSpec>;
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEPAGE (w) register accessor: Register for erasing a page in code area\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepage::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepage`] module"]
#[doc(alias = "ERASEPAGE")]
pub type Erasepage = crate::Reg<erasepage::ErasepageSpec>;
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "ERASEPCR1 (w) register accessor: Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepcr1`] module"]
#[doc(alias = "ERASEPCR1")]
pub type Erasepcr1 = crate::Reg<erasepcr1::Erasepcr1Spec>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr1;
#[doc = "ERASEALL (w) register accessor: Register for erasing all non-volatile user memory\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseall`] module"]
#[doc(alias = "ERASEALL")]
pub type Eraseall = crate::Reg<eraseall::EraseallSpec>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPCR0 (w) register accessor: Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepcr0`] module"]
#[doc(alias = "ERASEPCR0")]
pub type Erasepcr0 = crate::Reg<erasepcr0::Erasepcr0Spec>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr0;
#[doc = "ERASEUICR (w) register accessor: Register for erasing user information configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseuicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseuicr`] module"]
#[doc(alias = "ERASEUICR")]
pub type Eraseuicr = crate::Reg<eraseuicr::EraseuicrSpec>;
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "ERASEPAGEPARTIAL (w) register accessor: Register for partial erase of a page in code area\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepagepartial::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepagepartial`] module"]
#[doc(alias = "ERASEPAGEPARTIAL")]
pub type Erasepagepartial = crate::Reg<erasepagepartial::ErasepagepartialSpec>;
#[doc = "Register for partial erase of a page in code area"]
pub mod erasepagepartial;
#[doc = "ERASEPAGEPARTIALCFG (rw) register accessor: Register for partial erase configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepagepartialcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepagepartialcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepagepartialcfg`] module"]
#[doc(alias = "ERASEPAGEPARTIALCFG")]
pub type Erasepagepartialcfg = crate::Reg<erasepagepartialcfg::ErasepagepartialcfgSpec>;
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "ICACHECNF (rw) register accessor: I-code cache configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icachecnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icachecnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icachecnf`] module"]
#[doc(alias = "ICACHECNF")]
pub type Icachecnf = crate::Reg<icachecnf::IcachecnfSpec>;
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "IHIT (rw) register accessor: I-code cache hit counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ihit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihit`] module"]
#[doc(alias = "IHIT")]
pub type Ihit = crate::Reg<ihit::IhitSpec>;
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "IMISS (rw) register accessor: I-code cache miss counter\n\nYou can [`read`](crate::Reg::read) this register and get [`imiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imiss`] module"]
#[doc(alias = "IMISS")]
pub type Imiss = crate::Reg<imiss::ImissSpec>;
#[doc = "I-code cache miss counter"]
pub mod imiss;
