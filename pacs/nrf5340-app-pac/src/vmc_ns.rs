#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    #[doc = "0x600..0x60c - Unspecified"]
    pub ram0: RAM,
    _reserved1: [u8; 0x04],
    #[doc = "0x610..0x61c - Unspecified"]
    pub ram1: RAM,
    _reserved2: [u8; 0x04],
    #[doc = "0x620..0x62c - Unspecified"]
    pub ram2: RAM,
    _reserved3: [u8; 0x04],
    #[doc = "0x630..0x63c - Unspecified"]
    pub ram3: RAM,
    _reserved4: [u8; 0x04],
    #[doc = "0x640..0x64c - Unspecified"]
    pub ram4: RAM,
    _reserved5: [u8; 0x04],
    #[doc = "0x650..0x65c - Unspecified"]
    pub ram5: RAM,
    _reserved6: [u8; 0x04],
    #[doc = "0x660..0x66c - Unspecified"]
    pub ram6: RAM,
    _reserved7: [u8; 0x04],
    #[doc = "0x670..0x67c - Unspecified"]
    pub ram7: RAM,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster: RAM\\[n\\]
power control register"]
    pub power: crate::Reg<self::ram::power::POWER_SPEC>,
    #[doc = "0x04 - Description cluster: RAM\\[n\\]
power control set register"]
    pub powerset: crate::Reg<self::ram::powerset::POWERSET_SPEC>,
    #[doc = "0x08 - Description cluster: RAM\\[n\\]
power control clear register"]
    pub powerclr: crate::Reg<self::ram::powerclr::POWERCLR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ram;
