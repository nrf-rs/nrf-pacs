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
    _reserved8: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved11: [u8; 0xf4],
    result: Result,
    _reserved12: [u8; 0xfc],
    enable: Enable,
    psel: Psel,
    refsel: Refsel,
    extrefsel: Extrefsel,
    _reserved16: [u8; 0x20],
    th: Th,
    mode: Mode,
    hyst: Hyst,
}
impl RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop comparator"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Sample comparator value"]
    #[inline(always)]
    pub const fn tasks_sample(&self) -> &TasksSample {
        &self.tasks_sample
    }
    #[doc = "0x100 - COMP is ready and output is valid"]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x104 - Downward crossing"]
    #[inline(always)]
    pub const fn events_down(&self) -> &EventsDown {
        &self.events_down
    }
    #[doc = "0x108 - Upward crossing"]
    #[inline(always)]
    pub const fn events_up(&self) -> &EventsUp {
        &self.events_up
    }
    #[doc = "0x10c - Downward or upward crossing"]
    #[inline(always)]
    pub const fn events_cross(&self) -> &EventsCross {
        &self.events_cross
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x400 - Compare result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x500 - COMP enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Pin select"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x508 - Reference source select for single-ended mode"]
    #[inline(always)]
    pub const fn refsel(&self) -> &Refsel {
        &self.refsel
    }
    #[doc = "0x50c - External reference select"]
    #[inline(always)]
    pub const fn extrefsel(&self) -> &Extrefsel {
        &self.extrefsel
    }
    #[doc = "0x530 - Threshold configuration for hysteresis unit"]
    #[inline(always)]
    pub const fn th(&self) -> &Th {
        &self.th
    }
    #[doc = "0x534 - Mode configuration"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x538 - Comparator hysteresis enable"]
    #[inline(always)]
    pub const fn hyst(&self) -> &Hyst {
        &self.hyst
    }
}
#[doc = "TASKS_START (w) register accessor: Start comparator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop comparator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "TASKS_SAMPLE (w) register accessor: Sample comparator value\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sample::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_sample`] module"]
#[doc(alias = "TASKS_SAMPLE")]
pub type TasksSample = crate::Reg<tasks_sample::TasksSampleSpec>;
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "EVENTS_READY (rw) register accessor: COMP is ready and output is valid\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`] module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "COMP is ready and output is valid"]
pub mod events_ready;
#[doc = "EVENTS_DOWN (rw) register accessor: Downward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_down::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_down::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_down`] module"]
#[doc(alias = "EVENTS_DOWN")]
pub type EventsDown = crate::Reg<events_down::EventsDownSpec>;
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "EVENTS_UP (rw) register accessor: Upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_up::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_up::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_up`] module"]
#[doc(alias = "EVENTS_UP")]
pub type EventsUp = crate::Reg<events_up::EventsUpSpec>;
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "EVENTS_CROSS (rw) register accessor: Downward or upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cross::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cross::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_cross`] module"]
#[doc(alias = "EVENTS_CROSS")]
pub type EventsCross = crate::Reg<events_cross::EventsCrossSpec>;
#[doc = "Downward or upward crossing"]
pub mod events_cross;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "RESULT (r) register accessor: Compare result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Compare result"]
pub mod result;
#[doc = "ENABLE (rw) register accessor: COMP enable\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "COMP enable"]
pub mod enable;
#[doc = "PSEL (rw) register accessor: Pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psel`] module"]
#[doc(alias = "PSEL")]
pub type Psel = crate::Reg<psel::PselSpec>;
#[doc = "Pin select"]
pub mod psel;
#[doc = "REFSEL (rw) register accessor: Reference source select for single-ended mode\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refsel`] module"]
#[doc(alias = "REFSEL")]
pub type Refsel = crate::Reg<refsel::RefselSpec>;
#[doc = "Reference source select for single-ended mode"]
pub mod refsel;
#[doc = "EXTREFSEL (rw) register accessor: External reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`extrefsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extrefsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extrefsel`] module"]
#[doc(alias = "EXTREFSEL")]
pub type Extrefsel = crate::Reg<extrefsel::ExtrefselSpec>;
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "TH (rw) register accessor: Threshold configuration for hysteresis unit\n\nYou can [`read`](crate::Reg::read) this register and get [`th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@th`] module"]
#[doc(alias = "TH")]
pub type Th = crate::Reg<th::ThSpec>;
#[doc = "Threshold configuration for hysteresis unit"]
pub mod th;
#[doc = "MODE (rw) register accessor: Mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Mode configuration"]
pub mod mode;
#[doc = "HYST (rw) register accessor: Comparator hysteresis enable\n\nYou can [`read`](crate::Reg::read) this register and get [`hyst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyst`] module"]
#[doc(alias = "HYST")]
pub type Hyst = crate::Reg<hyst::HystSpec>;
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
