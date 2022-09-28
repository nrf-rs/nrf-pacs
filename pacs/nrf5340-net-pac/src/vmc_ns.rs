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
}
#[doc = "Unspecified"]
pub use ram::RAM;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ram;
