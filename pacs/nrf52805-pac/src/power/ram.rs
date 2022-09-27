#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
    pub power: POWER,
    #[doc = "0x04 - Description cluster: RAMn power control set register"]
    pub powerset: POWERSET,
    #[doc = "0x08 - Description cluster: RAMn power control clear register"]
    pub powerclr: POWERCLR,
}
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
pub mod power;
#[doc = "POWERSET (w) register accessor: an alias for `Reg<POWERSET_SPEC>`"]
pub type POWERSET = crate::Reg<powerset::POWERSET_SPEC>;
#[doc = "Description cluster: RAMn power control set register"]
pub mod powerset;
#[doc = "POWERCLR (w) register accessor: an alias for `Reg<POWERCLR_SPEC>`"]
pub type POWERCLR = crate::Reg<powerclr::POWERCLR_SPEC>;
#[doc = "Description cluster: RAMn power control clear register"]
pub mod powerclr;
