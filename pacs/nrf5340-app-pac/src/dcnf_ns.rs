#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0420],
    #[doc = "0x420 - CPU ID of this subsystem"]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
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
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTPERI {
    #[doc = "0x00 - Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
    pub protect: crate::Reg<self::extperi::protect::PROTECT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extperi;
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTRAM {
    #[doc = "0x00 - Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
    pub protect: crate::Reg<self::extram::protect::PROTECT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extram;
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTCODE {
    #[doc = "0x00 - Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]"]
    pub protect: crate::Reg<self::extcode::protect::PROTECT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extcode;
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
