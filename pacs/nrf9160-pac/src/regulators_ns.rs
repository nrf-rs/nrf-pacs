#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    systemoff: Systemoff,
    _reserved1: [u8; 0x10],
    extpofcon: Extpofcon,
    _reserved2: [u8; 0x60],
    dcdcen: Dcdcen,
}
impl RegisterBlock {
    #[doc = "0x500 - System OFF register"]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x514 - External power failure warning configuration"]
    #[inline(always)]
    pub const fn extpofcon(&self) -> &Extpofcon {
        &self.extpofcon
    }
    #[doc = "0x578 - Enable a step-down DC/DC voltage regulator."]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
}
#[doc = "SYSTEMOFF (w) register accessor: System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`] module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "EXTPOFCON (rw) register accessor: External power failure warning configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`extpofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extpofcon`] module"]
#[doc(alias = "EXTPOFCON")]
pub type Extpofcon = crate::Reg<extpofcon::ExtpofconSpec>;
#[doc = "External power failure warning configuration"]
pub mod extpofcon;
#[doc = "DCDCEN (rw) register accessor: Enable a step-down DC/DC voltage regulator.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`] module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "Enable a step-down DC/DC voltage regulator."]
pub mod dcdcen;
