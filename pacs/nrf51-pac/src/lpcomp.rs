#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_sample: TasksSample,
    _reserved3: [u8; 0xf4],
    events_ready: EventsReady,
    events_down: EventsDown,
    events_up: EventsUp,
    events_cross: EventsCross,
    _reserved7: [u8; 0xf0],
    shorts: Shorts,
    _reserved8: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved10: [u8; 0xf4],
    result: Result,
    _reserved11: [u8; 0xfc],
    enable: Enable,
    psel: Psel,
    refsel: Refsel,
    extrefsel: Extrefsel,
    _reserved15: [u8; 0x10],
    anadetect: Anadetect,
    _reserved16: [u8; 0x0ad8],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Start the comparator."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop the comparator."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Sample comparator value."]
    #[inline(always)]
    pub const fn tasks_sample(&self) -> &TasksSample {
        &self.tasks_sample
    }
    #[doc = "0x100 - LPCOMP is ready and output is valid."]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x104 - Input voltage crossed the threshold going down."]
    #[inline(always)]
    pub const fn events_down(&self) -> &EventsDown {
        &self.events_down
    }
    #[doc = "0x108 - Input voltage crossed the threshold going up."]
    #[inline(always)]
    pub const fn events_up(&self) -> &EventsUp {
        &self.events_up
    }
    #[doc = "0x10c - Input voltage crossed the threshold in any direction."]
    #[inline(always)]
    pub const fn events_cross(&self) -> &EventsCross {
        &self.events_cross
    }
    #[doc = "0x200 - Shortcuts for the LPCOMP."]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x304 - Interrupt enable set register."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Interrupt enable clear register."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x400 - Result of last compare."]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x500 - Enable the LPCOMP."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Input pin select."]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x508 - Reference select."]
    #[inline(always)]
    pub const fn refsel(&self) -> &Refsel {
        &self.refsel
    }
    #[doc = "0x50c - External reference select."]
    #[inline(always)]
    pub const fn extrefsel(&self) -> &Extrefsel {
        &self.extrefsel
    }
    #[doc = "0x520 - Analog detect configuration."]
    #[inline(always)]
    pub const fn anadetect(&self) -> &Anadetect {
        &self.anadetect
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_START (w) register accessor: Start the comparator.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start the comparator."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop the comparator.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop the comparator."]
pub mod tasks_stop;
#[doc = "TASKS_SAMPLE (w) register accessor: Sample comparator value.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sample::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_sample`] module"]
#[doc(alias = "TASKS_SAMPLE")]
pub type TasksSample = crate::Reg<tasks_sample::TasksSampleSpec>;
#[doc = "Sample comparator value."]
pub mod tasks_sample;
#[doc = "EVENTS_READY (rw) register accessor: LPCOMP is ready and output is valid.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`] module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "LPCOMP is ready and output is valid."]
pub mod events_ready;
#[doc = "EVENTS_DOWN (rw) register accessor: Input voltage crossed the threshold going down.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_down::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_down::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_down`] module"]
#[doc(alias = "EVENTS_DOWN")]
pub type EventsDown = crate::Reg<events_down::EventsDownSpec>;
#[doc = "Input voltage crossed the threshold going down."]
pub mod events_down;
#[doc = "EVENTS_UP (rw) register accessor: Input voltage crossed the threshold going up.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_up::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_up::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_up`] module"]
#[doc(alias = "EVENTS_UP")]
pub type EventsUp = crate::Reg<events_up::EventsUpSpec>;
#[doc = "Input voltage crossed the threshold going up."]
pub mod events_up;
#[doc = "EVENTS_CROSS (rw) register accessor: Input voltage crossed the threshold in any direction.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cross::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cross::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_cross`] module"]
#[doc(alias = "EVENTS_CROSS")]
pub type EventsCross = crate::Reg<events_cross::EventsCrossSpec>;
#[doc = "Input voltage crossed the threshold in any direction."]
pub mod events_cross;
#[doc = "SHORTS (rw) register accessor: Shortcuts for the LPCOMP.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts for the LPCOMP."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "RESULT (r) register accessor: Result of last compare.\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result of last compare."]
pub mod result;
#[doc = "ENABLE (rw) register accessor: Enable the LPCOMP.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable the LPCOMP."]
pub mod enable;
#[doc = "PSEL (rw) register accessor: Input pin select.\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psel`] module"]
#[doc(alias = "PSEL")]
pub type Psel = crate::Reg<psel::PselSpec>;
#[doc = "Input pin select."]
pub mod psel;
#[doc = "REFSEL (rw) register accessor: Reference select.\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refsel`] module"]
#[doc(alias = "REFSEL")]
pub type Refsel = crate::Reg<refsel::RefselSpec>;
#[doc = "Reference select."]
pub mod refsel;
#[doc = "EXTREFSEL (rw) register accessor: External reference select.\n\nYou can [`read`](crate::Reg::read) this register and get [`extrefsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extrefsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extrefsel`] module"]
#[doc(alias = "EXTREFSEL")]
pub type Extrefsel = crate::Reg<extrefsel::ExtrefselSpec>;
#[doc = "External reference select."]
pub mod extrefsel;
#[doc = "ANADETECT (rw) register accessor: Analog detect configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`anadetect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anadetect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anadetect`] module"]
#[doc(alias = "ANADETECT")]
pub type Anadetect = crate::Reg<anadetect::AnadetectSpec>;
#[doc = "Analog detect configuration."]
pub mod anadetect;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
