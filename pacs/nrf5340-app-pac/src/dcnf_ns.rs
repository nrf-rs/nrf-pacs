#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0420],
    #[doc = "0x420 - CPU ID of this subsystem"]
    pub cpuid: CPUID,
    _reserved1: [u8; 0x1c],
    #[doc = "0x440 - Unspecified"]
    pub extperi: [EXTPERI; 1],
    _reserved2: [u8; 0x1c],
    #[doc = "0x460 - Unspecified"]
    pub extram: [EXTRAM; 1],
    _reserved3: [u8; 0x1c],
    #[doc = "0x480 - Unspecified"]
    pub extcode: [EXTCODE; 1],
}
#[doc = "CPUID (r) register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
#[doc = "Unspecified"]
pub use extperi::EXTPERI;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extperi;
#[doc = "Unspecified"]
pub use extram::EXTRAM;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extram;
#[doc = "Unspecified"]
pub use extcode::EXTCODE;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extcode;
