#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RAM")]
pub struct Ram {
    power: Power,
    powerset: Powerset,
    powerclr: Powerclr,
}
impl Ram {
    #[doc = "0x00 - Description cluster: RAMn power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x04 - Description cluster: RAMn power control set register"]
    #[inline(always)]
    pub const fn powerset(&self) -> &Powerset {
        &self.powerset
    }
    #[doc = "0x08 - Description cluster: RAMn power control clear register"]
    #[inline(always)]
    pub const fn powerclr(&self) -> &Powerclr {
        &self.powerclr
    }
}
#[doc = "POWER (rw) register accessor: Description cluster: RAMn power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Description cluster: RAMn power control register"]
pub mod power;
#[doc = "POWERSET (w) register accessor: Description cluster: RAMn power control set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerset`]
module"]
#[doc(alias = "POWERSET")]
pub type Powerset = crate::Reg<powerset::PowersetSpec>;
#[doc = "Description cluster: RAMn power control set register"]
pub mod powerset;
#[doc = "POWERCLR (w) register accessor: Description cluster: RAMn power control clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerclr`]
module"]
#[doc(alias = "POWERCLR")]
pub type Powerclr = crate::Reg<powerclr::PowerclrSpec>;
#[doc = "Description cluster: RAMn power control clear register"]
pub mod powerclr;
