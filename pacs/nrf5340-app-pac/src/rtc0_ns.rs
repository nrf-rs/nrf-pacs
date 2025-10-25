#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_clear: TasksClear,
    tasks_trigovrflw: TasksTrigovrflw,
    _reserved4: [u8; 0x30],
    tasks_capture: [TasksCapture; 4],
    _reserved5: [u8; 0x30],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    subscribe_clear: SubscribeClear,
    subscribe_trigovrflw: SubscribeTrigovrflw,
    _reserved9: [u8; 0x30],
    subscribe_capture: [SubscribeCapture; 4],
    _reserved10: [u8; 0x30],
    events_tick: EventsTick,
    events_ovrflw: EventsOvrflw,
    _reserved12: [u8; 0x38],
    events_compare: [EventsCompare; 4],
    _reserved13: [u8; 0x30],
    publish_tick: PublishTick,
    publish_ovrflw: PublishOvrflw,
    _reserved15: [u8; 0x38],
    publish_compare: [PublishCompare; 4],
    _reserved16: [u8; 0x30],
    shorts: Shorts,
    _reserved17: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved19: [u8; 0x34],
    evten: Evten,
    evtenset: Evtenset,
    evtenclr: Evtenclr,
    _reserved22: [u8; 0x01b8],
    counter: Counter,
    prescaler: Prescaler,
    _reserved24: [u8; 0x34],
    cc: [Cc; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Start RTC counter"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop RTC counter"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Clear RTC counter"]
    #[inline(always)]
    pub const fn tasks_clear(&self) -> &TasksClear {
        &self.tasks_clear
    }
    #[doc = "0x0c - Set counter to 0xFFFFF0"]
    #[inline(always)]
    pub const fn tasks_trigovrflw(&self) -> &TasksTrigovrflw {
        &self.tasks_trigovrflw
    }
    #[doc = "0x40..0x50 - Description collection: Capture RTC counter to CC\\[n\\] register"]
    #[inline(always)]
    pub const fn tasks_capture(&self, n: usize) -> &TasksCapture {
        &self.tasks_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Description collection: Capture RTC counter to CC\\[n\\] register"]
    #[inline(always)]
    pub fn tasks_capture_iter(&self) -> impl Iterator<Item = &TasksCapture> {
        self.tasks_capture.iter()
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
    #[doc = "0x88 - Subscribe configuration for task CLEAR"]
    #[inline(always)]
    pub const fn subscribe_clear(&self) -> &SubscribeClear {
        &self.subscribe_clear
    }
    #[doc = "0x8c - Subscribe configuration for task TRIGOVRFLW"]
    #[inline(always)]
    pub const fn subscribe_trigovrflw(&self) -> &SubscribeTrigovrflw {
        &self.subscribe_trigovrflw
    }
    #[doc = "0xc0..0xd0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_capture(&self, n: usize) -> &SubscribeCapture {
        &self.subscribe_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xd0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_capture_iter(&self) -> impl Iterator<Item = &SubscribeCapture> {
        self.subscribe_capture.iter()
    }
    #[doc = "0x100 - Event on counter increment"]
    #[inline(always)]
    pub const fn events_tick(&self) -> &EventsTick {
        &self.events_tick
    }
    #[doc = "0x104 - Event on counter overflow"]
    #[inline(always)]
    pub const fn events_ovrflw(&self) -> &EventsOvrflw {
        &self.events_ovrflw
    }
    #[doc = "0x140..0x150 - Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub const fn events_compare(&self, n: usize) -> &EventsCompare {
        &self.events_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare_iter(&self) -> impl Iterator<Item = &EventsCompare> {
        self.events_compare.iter()
    }
    #[doc = "0x180 - Publish configuration for event TICK"]
    #[inline(always)]
    pub const fn publish_tick(&self) -> &PublishTick {
        &self.publish_tick
    }
    #[doc = "0x184 - Publish configuration for event OVRFLW"]
    #[inline(always)]
    pub const fn publish_ovrflw(&self) -> &PublishOvrflw {
        &self.publish_ovrflw
    }
    #[doc = "0x1c0..0x1d0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_compare(&self, n: usize) -> &PublishCompare {
        &self.publish_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1d0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub fn publish_compare_iter(&self) -> impl Iterator<Item = &PublishCompare> {
        self.publish_compare.iter()
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
    #[doc = "0x340 - Enable or disable event routing"]
    #[inline(always)]
    pub const fn evten(&self) -> &Evten {
        &self.evten
    }
    #[doc = "0x344 - Enable event routing"]
    #[inline(always)]
    pub const fn evtenset(&self) -> &Evtenset {
        &self.evtenset
    }
    #[doc = "0x348 - Disable event routing"]
    #[inline(always)]
    pub const fn evtenclr(&self) -> &Evtenclr {
        &self.evtenclr
    }
    #[doc = "0x504 - Current counter value"]
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
    #[doc = "0x508 - 12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x540..0x550 - Description collection: Compare register n"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x550 - Description collection: Compare register n"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
}
#[doc = "TASKS_START (w) register accessor: Start RTC counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start RTC counter"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop RTC counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop RTC counter"]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR (w) register accessor: Clear RTC counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clear`] module"]
#[doc(alias = "TASKS_CLEAR")]
pub type TasksClear = crate::Reg<tasks_clear::TasksClearSpec>;
#[doc = "Clear RTC counter"]
pub mod tasks_clear;
#[doc = "TASKS_TRIGOVRFLW (w) register accessor: Set counter to 0xFFFFF0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigovrflw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_trigovrflw`] module"]
#[doc(alias = "TASKS_TRIGOVRFLW")]
pub type TasksTrigovrflw = crate::Reg<tasks_trigovrflw::TasksTrigovrflwSpec>;
#[doc = "Set counter to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "TASKS_CAPTURE (w) register accessor: Description collection: Capture RTC counter to CC\\[n\\] register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_capture`] module"]
#[doc(alias = "TASKS_CAPTURE")]
pub type TasksCapture = crate::Reg<tasks_capture::TasksCaptureSpec>;
#[doc = "Description collection: Capture RTC counter to CC\\[n\\] register"]
pub mod tasks_capture;
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
#[doc = "SUBSCRIBE_CLEAR (rw) register accessor: Subscribe configuration for task CLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_clear`] module"]
#[doc(alias = "SUBSCRIBE_CLEAR")]
pub type SubscribeClear = crate::Reg<subscribe_clear::SubscribeClearSpec>;
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "SUBSCRIBE_TRIGOVRFLW (rw) register accessor: Subscribe configuration for task TRIGOVRFLW\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_trigovrflw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_trigovrflw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_trigovrflw`] module"]
#[doc(alias = "SUBSCRIBE_TRIGOVRFLW")]
pub type SubscribeTrigovrflw = crate::Reg<subscribe_trigovrflw::SubscribeTrigovrflwSpec>;
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
pub mod subscribe_trigovrflw;
#[doc = "SUBSCRIBE_CAPTURE (rw) register accessor: Description collection: Subscribe configuration for task CAPTURE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_capture`] module"]
#[doc(alias = "SUBSCRIBE_CAPTURE")]
pub type SubscribeCapture = crate::Reg<subscribe_capture::SubscribeCaptureSpec>;
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
pub mod subscribe_capture;
#[doc = "EVENTS_TICK (rw) register accessor: Event on counter increment\n\nYou can [`read`](crate::Reg::read) this register and get [`events_tick::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_tick::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_tick`] module"]
#[doc(alias = "EVENTS_TICK")]
pub type EventsTick = crate::Reg<events_tick::EventsTickSpec>;
#[doc = "Event on counter increment"]
pub mod events_tick;
#[doc = "EVENTS_OVRFLW (rw) register accessor: Event on counter overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ovrflw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ovrflw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ovrflw`] module"]
#[doc(alias = "EVENTS_OVRFLW")]
pub type EventsOvrflw = crate::Reg<events_ovrflw::EventsOvrflwSpec>;
#[doc = "Event on counter overflow"]
pub mod events_ovrflw;
#[doc = "EVENTS_COMPARE (rw) register accessor: Description collection: Compare event on CC\\[n\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_compare`] module"]
#[doc(alias = "EVENTS_COMPARE")]
pub type EventsCompare = crate::Reg<events_compare::EventsCompareSpec>;
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub mod events_compare;
#[doc = "PUBLISH_TICK (rw) register accessor: Publish configuration for event TICK\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_tick::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_tick::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_tick`] module"]
#[doc(alias = "PUBLISH_TICK")]
pub type PublishTick = crate::Reg<publish_tick::PublishTickSpec>;
#[doc = "Publish configuration for event TICK"]
pub mod publish_tick;
#[doc = "PUBLISH_OVRFLW (rw) register accessor: Publish configuration for event OVRFLW\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ovrflw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ovrflw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ovrflw`] module"]
#[doc(alias = "PUBLISH_OVRFLW")]
pub type PublishOvrflw = crate::Reg<publish_ovrflw::PublishOvrflwSpec>;
#[doc = "Publish configuration for event OVRFLW"]
pub mod publish_ovrflw;
#[doc = "PUBLISH_COMPARE (rw) register accessor: Description collection: Publish configuration for event COMPARE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_compare`] module"]
#[doc(alias = "PUBLISH_COMPARE")]
pub type PublishCompare = crate::Reg<publish_compare::PublishCompareSpec>;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
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
#[doc = "EVTEN (rw) register accessor: Enable or disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evten`] module"]
#[doc(alias = "EVTEN")]
pub type Evten = crate::Reg<evten::EvtenSpec>;
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "EVTENSET (rw) register accessor: Enable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evtenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtenset`] module"]
#[doc(alias = "EVTENSET")]
pub type Evtenset = crate::Reg<evtenset::EvtensetSpec>;
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "EVTENCLR (rw) register accessor: Disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evtenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtenclr`] module"]
#[doc(alias = "EVTENCLR")]
pub type Evtenclr = crate::Reg<evtenclr::EvtenclrSpec>;
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "COUNTER (r) register accessor: Current counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`] module"]
#[doc(alias = "COUNTER")]
pub type Counter = crate::Reg<counter::CounterSpec>;
#[doc = "Current counter value"]
pub mod counter;
#[doc = "PRESCALER (rw) register accessor: 12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped.\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`] module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
pub mod prescaler;
#[doc = "CC (rw) register accessor: Description collection: Compare register n\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Description collection: Compare register n"]
pub mod cc;
