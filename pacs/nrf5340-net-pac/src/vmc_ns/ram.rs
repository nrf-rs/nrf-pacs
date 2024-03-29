#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster: RAM\\[n\\]
power control register"]
    pub power: POWER,
    #[doc = "0x04 - Description cluster: RAM\\[n\\]
power control set register"]
    pub powerset: POWERSET,
    #[doc = "0x08 - Description cluster: RAM\\[n\\]
power control clear register"]
    pub powerclr: POWERCLR,
}
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Description cluster: RAM\\[n\\]
power control register"]
pub mod power;
#[doc = "POWERSET (rw) register accessor: an alias for `Reg<POWERSET_SPEC>`"]
pub type POWERSET = crate::Reg<powerset::POWERSET_SPEC>;
#[doc = "Description cluster: RAM\\[n\\]
power control set register"]
pub mod powerset;
#[doc = "POWERCLR (rw) register accessor: an alias for `Reg<POWERCLR_SPEC>`"]
pub type POWERCLR = crate::Reg<powerclr::POWERCLR_SPEC>;
#[doc = "Description cluster: RAM\\[n\\]
power control clear register"]
pub mod powerclr;
