#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    pub events_invalidoperation: crate::Reg<events_invalidoperation::EVENTS_INVALIDOPERATION_SPEC>,
    #[doc = "0x104 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    pub events_dividebyzero: crate::Reg<events_dividebyzero::EVENTS_DIVIDEBYZERO_SPEC>,
    #[doc = "0x108 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    pub events_overflow: crate::Reg<events_overflow::EVENTS_OVERFLOW_SPEC>,
    #[doc = "0x10c - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    pub events_underflow: crate::Reg<events_underflow::EVENTS_UNDERFLOW_SPEC>,
    #[doc = "0x110 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    pub events_inexact: crate::Reg<events_inexact::EVENTS_INEXACT_SPEC>,
    #[doc = "0x114 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    pub events_denormalinput: crate::Reg<events_denormalinput::EVENTS_DENORMALINPUT_SPEC>,
    _reserved6: [u8; 0x01e8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
}
#[doc = "EVENTS_INVALIDOPERATION register accessor: an alias for `Reg<EVENTS_INVALIDOPERATION_SPEC>`"]
pub type EVENTS_INVALIDOPERATION =
    crate::Reg<events_invalidoperation::EVENTS_INVALIDOPERATION_SPEC>;
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub mod events_invalidoperation;
#[doc = "EVENTS_DIVIDEBYZERO register accessor: an alias for `Reg<EVENTS_DIVIDEBYZERO_SPEC>`"]
pub type EVENTS_DIVIDEBYZERO = crate::Reg<events_dividebyzero::EVENTS_DIVIDEBYZERO_SPEC>;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub mod events_dividebyzero;
#[doc = "EVENTS_OVERFLOW register accessor: an alias for `Reg<EVENTS_OVERFLOW_SPEC>`"]
pub type EVENTS_OVERFLOW = crate::Reg<events_overflow::EVENTS_OVERFLOW_SPEC>;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub mod events_overflow;
#[doc = "EVENTS_UNDERFLOW register accessor: an alias for `Reg<EVENTS_UNDERFLOW_SPEC>`"]
pub type EVENTS_UNDERFLOW = crate::Reg<events_underflow::EVENTS_UNDERFLOW_SPEC>;
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub mod events_underflow;
#[doc = "EVENTS_INEXACT register accessor: an alias for `Reg<EVENTS_INEXACT_SPEC>`"]
pub type EVENTS_INEXACT = crate::Reg<events_inexact::EVENTS_INEXACT_SPEC>;
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub mod events_inexact;
#[doc = "EVENTS_DENORMALINPUT register accessor: an alias for `Reg<EVENTS_DENORMALINPUT_SPEC>`"]
pub type EVENTS_DENORMALINPUT = crate::Reg<events_denormalinput::EVENTS_DENORMALINPUT_SPEC>;
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub mod events_denormalinput;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
