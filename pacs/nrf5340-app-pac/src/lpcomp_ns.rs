#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_sample: TasksSample,
    _reserved3: [u8; 0x74],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    subscribe_sample: SubscribeSample,
    _reserved6: [u8; 0x74],
    events_ready: EventsReady,
    events_down: EventsDown,
    events_up: EventsUp,
    events_cross: EventsCross,
    _reserved10: [u8; 0x70],
    publish_ready: PublishReady,
    publish_down: PublishDown,
    publish_up: PublishUp,
    publish_cross: PublishCross,
    _reserved14: [u8; 0x70],
    shorts: Shorts,
    _reserved15: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved17: [u8; 0xf4],
    result: Result,
    _reserved18: [u8; 0xfc],
    enable: Enable,
    psel: Psel,
    refsel: Refsel,
    extrefsel: Extrefsel,
    _reserved22: [u8; 0x10],
    anadetect: Anadetect,
    _reserved23: [u8; 0x14],
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
    #[doc = "0x80 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(&self) -> &SubscribeStart {
        &self.subscribe_start
    }
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x88 - Subscribe configuration for task SAMPLE"]
    #[inline(always)]
    pub const fn subscribe_sample(&self) -> &SubscribeSample {
        &self.subscribe_sample
    }
    #[doc = "0x100 - LPCOMP is ready and output is valid"]
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
    #[doc = "0x180 - Publish configuration for event READY"]
    #[inline(always)]
    pub const fn publish_ready(&self) -> &PublishReady {
        &self.publish_ready
    }
    #[doc = "0x184 - Publish configuration for event DOWN"]
    #[inline(always)]
    pub const fn publish_down(&self) -> &PublishDown {
        &self.publish_down
    }
    #[doc = "0x188 - Publish configuration for event UP"]
    #[inline(always)]
    pub const fn publish_up(&self) -> &PublishUp {
        &self.publish_up
    }
    #[doc = "0x18c - Publish configuration for event CROSS"]
    #[inline(always)]
    pub const fn publish_cross(&self) -> &PublishCross {
        &self.publish_cross
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x500 - Enable LPCOMP"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Input pin select"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x508 - Reference select"]
    #[inline(always)]
    pub const fn refsel(&self) -> &Refsel {
        &self.refsel
    }
    #[doc = "0x50c - External reference select"]
    #[inline(always)]
    pub const fn extrefsel(&self) -> &Extrefsel {
        &self.extrefsel
    }
    #[doc = "0x520 - Analog detect configuration"]
    #[inline(always)]
    pub const fn anadetect(&self) -> &Anadetect {
        &self.anadetect
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
#[doc = "SUBSCRIBE_START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_start`] module"]
#[doc(alias = "SUBSCRIBE_START")]
pub type SubscribeStart = crate::Reg<subscribe_start::SubscribeStartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stop`] module"]
#[doc(alias = "SUBSCRIBE_STOP")]
pub type SubscribeStop = crate::Reg<subscribe_stop::SubscribeStopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SAMPLE (rw) register accessor: Subscribe configuration for task SAMPLE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_sample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_sample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_sample`] module"]
#[doc(alias = "SUBSCRIBE_SAMPLE")]
pub type SubscribeSample = crate::Reg<subscribe_sample::SubscribeSampleSpec>;
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
#[doc = "EVENTS_READY (rw) register accessor: LPCOMP is ready and output is valid\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`] module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "LPCOMP is ready and output is valid"]
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
#[doc = "PUBLISH_READY (rw) register accessor: Publish configuration for event READY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ready`] module"]
#[doc(alias = "PUBLISH_READY")]
pub type PublishReady = crate::Reg<publish_ready::PublishReadySpec>;
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "PUBLISH_DOWN (rw) register accessor: Publish configuration for event DOWN\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_down::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_down::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_down`] module"]
#[doc(alias = "PUBLISH_DOWN")]
pub type PublishDown = crate::Reg<publish_down::PublishDownSpec>;
#[doc = "Publish configuration for event DOWN"]
pub mod publish_down;
#[doc = "PUBLISH_UP (rw) register accessor: Publish configuration for event UP\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_up::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_up::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_up`] module"]
#[doc(alias = "PUBLISH_UP")]
pub type PublishUp = crate::Reg<publish_up::PublishUpSpec>;
#[doc = "Publish configuration for event UP"]
pub mod publish_up;
#[doc = "PUBLISH_CROSS (rw) register accessor: Publish configuration for event CROSS\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_cross::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_cross::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_cross`] module"]
#[doc(alias = "PUBLISH_CROSS")]
pub type PublishCross = crate::Reg<publish_cross::PublishCrossSpec>;
#[doc = "Publish configuration for event CROSS"]
pub mod publish_cross;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "ENABLE (rw) register accessor: Enable LPCOMP\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable LPCOMP"]
pub mod enable;
#[doc = "PSEL (rw) register accessor: Input pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psel`] module"]
#[doc(alias = "PSEL")]
pub type Psel = crate::Reg<psel::PselSpec>;
#[doc = "Input pin select"]
pub mod psel;
#[doc = "REFSEL (rw) register accessor: Reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refsel`] module"]
#[doc(alias = "REFSEL")]
pub type Refsel = crate::Reg<refsel::RefselSpec>;
#[doc = "Reference select"]
pub mod refsel;
#[doc = "EXTREFSEL (rw) register accessor: External reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`extrefsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extrefsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extrefsel`] module"]
#[doc(alias = "EXTREFSEL")]
pub type Extrefsel = crate::Reg<extrefsel::ExtrefselSpec>;
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "ANADETECT (rw) register accessor: Analog detect configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`anadetect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anadetect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anadetect`] module"]
#[doc(alias = "ANADETECT")]
pub type Anadetect = crate::Reg<anadetect::AnadetectSpec>;
#[doc = "Analog detect configuration"]
pub mod anadetect;
#[doc = "HYST (rw) register accessor: Comparator hysteresis enable\n\nYou can [`read`](crate::Reg::read) this register and get [`hyst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hyst`] module"]
#[doc(alias = "HYST")]
pub type Hyst = crate::Reg<hyst::HystSpec>;
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
