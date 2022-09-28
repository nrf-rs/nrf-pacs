#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    pub events_invalidoperation: EVENTS_INVALIDOPERATION,
    #[doc = "0x104 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    pub events_dividebyzero: EVENTS_DIVIDEBYZERO,
    #[doc = "0x108 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    pub events_overflow: EVENTS_OVERFLOW,
    #[doc = "0x10c - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    pub events_underflow: EVENTS_UNDERFLOW,
    #[doc = "0x110 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    pub events_inexact: EVENTS_INEXACT,
    #[doc = "0x114 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    pub events_denormalinput: EVENTS_DENORMALINPUT,
    _reserved6: [u8; 0x01e8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
}
#[doc = "EVENTS_INVALIDOPERATION (rw) register accessor: an alias for `Reg<EVENTS_INVALIDOPERATION_SPEC>`"]
pub type EVENTS_INVALIDOPERATION =
    crate::Reg<events_invalidoperation::EVENTS_INVALIDOPERATION_SPEC>;
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub mod events_invalidoperation;
#[doc = "EVENTS_DIVIDEBYZERO (rw) register accessor: an alias for `Reg<EVENTS_DIVIDEBYZERO_SPEC>`"]
pub type EVENTS_DIVIDEBYZERO = crate::Reg<events_dividebyzero::EVENTS_DIVIDEBYZERO_SPEC>;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub mod events_dividebyzero;
#[doc = "EVENTS_OVERFLOW (rw) register accessor: an alias for `Reg<EVENTS_OVERFLOW_SPEC>`"]
pub type EVENTS_OVERFLOW = crate::Reg<events_overflow::EVENTS_OVERFLOW_SPEC>;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub mod events_overflow;
#[doc = "EVENTS_UNDERFLOW (rw) register accessor: an alias for `Reg<EVENTS_UNDERFLOW_SPEC>`"]
pub type EVENTS_UNDERFLOW = crate::Reg<events_underflow::EVENTS_UNDERFLOW_SPEC>;
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub mod events_underflow;
#[doc = "EVENTS_INEXACT (rw) register accessor: an alias for `Reg<EVENTS_INEXACT_SPEC>`"]
pub type EVENTS_INEXACT = crate::Reg<events_inexact::EVENTS_INEXACT_SPEC>;
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub mod events_inexact;
#[doc = "EVENTS_DENORMALINPUT (rw) register accessor: an alias for `Reg<EVENTS_DENORMALINPUT_SPEC>`"]
pub type EVENTS_DENORMALINPUT = crate::Reg<events_denormalinput::EVENTS_DENORMALINPUT_SPEC>;
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub mod events_denormalinput;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
