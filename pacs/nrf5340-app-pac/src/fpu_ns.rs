#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_invalidoperation: EventsInvalidoperation,
    events_dividebyzero: EventsDividebyzero,
    events_overflow: EventsOverflow,
    events_underflow: EventsUnderflow,
    events_inexact: EventsInexact,
    events_denormalinput: EventsDenormalinput,
    _reserved6: [u8; 0x01e8],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
}
impl RegisterBlock {
    #[doc = "0x100 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_invalidoperation(&self) -> &EventsInvalidoperation {
        &self.events_invalidoperation
    }
    #[doc = "0x104 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_dividebyzero(&self) -> &EventsDividebyzero {
        &self.events_dividebyzero
    }
    #[doc = "0x108 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_overflow(&self) -> &EventsOverflow {
        &self.events_overflow
    }
    #[doc = "0x10c - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_underflow(&self) -> &EventsUnderflow {
        &self.events_underflow
    }
    #[doc = "0x110 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_inexact(&self) -> &EventsInexact {
        &self.events_inexact
    }
    #[doc = "0x114 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_denormalinput(&self) -> &EventsDenormalinput {
        &self.events_denormalinput
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
}
#[doc = "EVENTS_INVALIDOPERATION (rw) register accessor: An FPUIOC exception triggered by an invalid operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_invalidoperation::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_invalidoperation::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_invalidoperation`] module"]
#[doc(alias = "EVENTS_INVALIDOPERATION")]
pub type EventsInvalidoperation = crate::Reg<events_invalidoperation::EventsInvalidoperationSpec>;
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub mod events_invalidoperation;
#[doc = "EVENTS_DIVIDEBYZERO (rw) register accessor: An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_dividebyzero::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_dividebyzero::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_dividebyzero`] module"]
#[doc(alias = "EVENTS_DIVIDEBYZERO")]
pub type EventsDividebyzero = crate::Reg<events_dividebyzero::EventsDividebyzeroSpec>;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub mod events_dividebyzero;
#[doc = "EVENTS_OVERFLOW (rw) register accessor: An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_overflow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_overflow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_overflow`] module"]
#[doc(alias = "EVENTS_OVERFLOW")]
pub type EventsOverflow = crate::Reg<events_overflow::EventsOverflowSpec>;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub mod events_overflow;
#[doc = "EVENTS_UNDERFLOW (rw) register accessor: An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_underflow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_underflow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_underflow`] module"]
#[doc(alias = "EVENTS_UNDERFLOW")]
pub type EventsUnderflow = crate::Reg<events_underflow::EventsUnderflowSpec>;
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub mod events_underflow;
#[doc = "EVENTS_INEXACT (rw) register accessor: An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_inexact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_inexact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_inexact`] module"]
#[doc(alias = "EVENTS_INEXACT")]
pub type EventsInexact = crate::Reg<events_inexact::EventsInexactSpec>;
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub mod events_inexact;
#[doc = "EVENTS_DENORMALINPUT (rw) register accessor: An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_denormalinput::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_denormalinput::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_denormalinput`] module"]
#[doc(alias = "EVENTS_DENORMALINPUT")]
pub type EventsDenormalinput = crate::Reg<events_denormalinput::EventsDenormalinputSpec>;
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub mod events_denormalinput;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
