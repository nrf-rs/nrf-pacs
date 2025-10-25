#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_count: TasksCount,
    tasks_clear: TasksClear,
    tasks_shutdown: TasksShutdown,
    _reserved5: [u8; 0x2c],
    tasks_capture: [TasksCapture; 8],
    _reserved6: [u8; 0x20],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    subscribe_count: SubscribeCount,
    subscribe_clear: SubscribeClear,
    subscribe_shutdown: SubscribeShutdown,
    _reserved11: [u8; 0x2c],
    subscribe_capture: [SubscribeCapture; 8],
    _reserved12: [u8; 0x60],
    events_compare: [EventsCompare; 8],
    _reserved13: [u8; 0x60],
    publish_compare: [PublishCompare; 8],
    _reserved14: [u8; 0x20],
    shorts: Shorts,
    _reserved15: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved18: [u8; 0x01f8],
    mode: Mode,
    bitmode: Bitmode,
    _reserved20: [u8; 0x04],
    prescaler: Prescaler,
    _reserved21: [u8; 0x2c],
    cc: [Cc; 8],
    _reserved22: [u8; 0x20],
    oneshoten: [Oneshoten; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop Timer"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    #[inline(always)]
    pub const fn tasks_count(&self) -> &TasksCount {
        &self.tasks_count
    }
    #[doc = "0x0c - Clear time"]
    #[inline(always)]
    pub const fn tasks_clear(&self) -> &TasksClear {
        &self.tasks_clear
    }
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    #[inline(always)]
    pub const fn tasks_shutdown(&self) -> &TasksShutdown {
        &self.tasks_shutdown
    }
    #[doc = "0x40..0x60 - Description collection: Capture Timer value to CC\\[n\\] register"]
    #[inline(always)]
    pub const fn tasks_capture(&self, n: usize) -> &TasksCapture {
        &self.tasks_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Description collection: Capture Timer value to CC\\[n\\] register"]
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
    #[doc = "0x88 - Subscribe configuration for task COUNT"]
    #[inline(always)]
    pub const fn subscribe_count(&self) -> &SubscribeCount {
        &self.subscribe_count
    }
    #[doc = "0x8c - Subscribe configuration for task CLEAR"]
    #[inline(always)]
    pub const fn subscribe_clear(&self) -> &SubscribeClear {
        &self.subscribe_clear
    }
    #[doc = "0x90 - Deprecated register - Subscribe configuration for task SHUTDOWN"]
    #[inline(always)]
    pub const fn subscribe_shutdown(&self) -> &SubscribeShutdown {
        &self.subscribe_shutdown
    }
    #[doc = "0xc0..0xe0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_capture(&self, n: usize) -> &SubscribeCapture {
        &self.subscribe_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_capture_iter(&self) -> impl Iterator<Item = &SubscribeCapture> {
        self.subscribe_capture.iter()
    }
    #[doc = "0x140..0x160 - Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub const fn events_compare(&self, n: usize) -> &EventsCompare {
        &self.events_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x160 - Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare_iter(&self) -> impl Iterator<Item = &EventsCompare> {
        self.events_compare.iter()
    }
    #[doc = "0x1c0..0x1e0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_compare(&self, n: usize) -> &PublishCompare {
        &self.publish_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub fn publish_compare_iter(&self) -> impl Iterator<Item = &PublishCompare> {
        self.publish_compare.iter()
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
    #[doc = "0x504 - Timer mode selection"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    #[inline(always)]
    pub const fn bitmode(&self) -> &Bitmode {
        &self.bitmode
    }
    #[doc = "0x510 - Timer prescaler register"]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x540..0x560 - Description collection: Capture/Compare register n"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x560 - Description collection: Capture/Compare register n"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
    #[doc = "0x580..0x5a0 - Description collection: Enable one-shot operation for Capture/Compare channel n"]
    #[inline(always)]
    pub const fn oneshoten(&self, n: usize) -> &Oneshoten {
        &self.oneshoten[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x580..0x5a0 - Description collection: Enable one-shot operation for Capture/Compare channel n"]
    #[inline(always)]
    pub fn oneshoten_iter(&self) -> impl Iterator<Item = &Oneshoten> {
        self.oneshoten.iter()
    }
}
#[doc = "TASKS_START (w) register accessor: Start Timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop Timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "TASKS_COUNT (w) register accessor: Increment Timer (Counter mode only)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_count::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_count`] module"]
#[doc(alias = "TASKS_COUNT")]
pub type TasksCount = crate::Reg<tasks_count::TasksCountSpec>;
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "TASKS_CLEAR (w) register accessor: Clear time\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clear`] module"]
#[doc(alias = "TASKS_CLEAR")]
pub type TasksClear = crate::Reg<tasks_clear::TasksClearSpec>;
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "TASKS_SHUTDOWN (w) register accessor: Deprecated register - Shut down timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_shutdown::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_shutdown`] module"]
#[doc(alias = "TASKS_SHUTDOWN")]
pub type TasksShutdown = crate::Reg<tasks_shutdown::TasksShutdownSpec>;
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "TASKS_CAPTURE (w) register accessor: Description collection: Capture Timer value to CC\\[n\\] register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_capture`] module"]
#[doc(alias = "TASKS_CAPTURE")]
pub type TasksCapture = crate::Reg<tasks_capture::TasksCaptureSpec>;
#[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
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
#[doc = "SUBSCRIBE_COUNT (rw) register accessor: Subscribe configuration for task COUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_count`] module"]
#[doc(alias = "SUBSCRIBE_COUNT")]
pub type SubscribeCount = crate::Reg<subscribe_count::SubscribeCountSpec>;
#[doc = "Subscribe configuration for task COUNT"]
pub mod subscribe_count;
#[doc = "SUBSCRIBE_CLEAR (rw) register accessor: Subscribe configuration for task CLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_clear`] module"]
#[doc(alias = "SUBSCRIBE_CLEAR")]
pub type SubscribeClear = crate::Reg<subscribe_clear::SubscribeClearSpec>;
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "SUBSCRIBE_SHUTDOWN (rw) register accessor: Deprecated register - Subscribe configuration for task SHUTDOWN\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_shutdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_shutdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_shutdown`] module"]
#[doc(alias = "SUBSCRIBE_SHUTDOWN")]
pub type SubscribeShutdown = crate::Reg<subscribe_shutdown::SubscribeShutdownSpec>;
#[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
pub mod subscribe_shutdown;
#[doc = "SUBSCRIBE_CAPTURE (rw) register accessor: Description collection: Subscribe configuration for task CAPTURE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_capture`] module"]
#[doc(alias = "SUBSCRIBE_CAPTURE")]
pub type SubscribeCapture = crate::Reg<subscribe_capture::SubscribeCaptureSpec>;
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
pub mod subscribe_capture;
#[doc = "EVENTS_COMPARE (rw) register accessor: Description collection: Compare event on CC\\[n\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_compare`] module"]
#[doc(alias = "EVENTS_COMPARE")]
pub type EventsCompare = crate::Reg<events_compare::EventsCompareSpec>;
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub mod events_compare;
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
#[doc = "MODE (rw) register accessor: Timer mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "BITMODE (rw) register accessor: Configure the number of bits used by the TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`bitmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitmode`] module"]
#[doc(alias = "BITMODE")]
pub type Bitmode = crate::Reg<bitmode::BitmodeSpec>;
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "PRESCALER (rw) register accessor: Timer prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`] module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "CC (rw) register accessor: Description collection: Capture/Compare register n\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Description collection: Capture/Compare register n"]
pub mod cc;
#[doc = "ONESHOTEN (rw) register accessor: Description collection: Enable one-shot operation for Capture/Compare channel n\n\nYou can [`read`](crate::Reg::read) this register and get [`oneshoten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oneshoten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oneshoten`] module"]
#[doc(alias = "ONESHOTEN")]
pub type Oneshoten = crate::Reg<oneshoten::OneshotenSpec>;
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
pub mod oneshoten;
