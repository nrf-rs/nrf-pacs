#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_count: TasksCount,
    tasks_clear: TasksClear,
    tasks_shutdown: TasksShutdown,
    _reserved5: [u8; 0x2c],
    tasks_capture: [TasksCapture; 4],
    _reserved6: [u8; 0xf0],
    events_compare: [EventsCompare; 4],
    _reserved7: [u8; 0xb0],
    shorts: Shorts,
    _reserved8: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved10: [u8; 0x01f8],
    mode: Mode,
    bitmode: Bitmode,
    _reserved12: [u8; 0x04],
    prescaler: Prescaler,
    _reserved13: [u8; 0x2c],
    cc: [Cc; 4],
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
    #[doc = "0x40..0x50 - Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register"]
    #[inline(always)]
    pub const fn tasks_capture(&self, n: usize) -> &TasksCapture {
        &self.tasks_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register"]
    #[inline(always)]
    pub fn tasks_capture_iter(&self) -> impl Iterator<Item = &TasksCapture> {
        self.tasks_capture.iter()
    }
    #[doc = "0x140..0x150 - Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
    #[inline(always)]
    pub const fn events_compare(&self, n: usize) -> &EventsCompare {
        &self.events_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
    #[inline(always)]
    pub fn events_compare_iter(&self) -> impl Iterator<Item = &EventsCompare> {
        self.events_compare.iter()
    }
    #[doc = "0x200 - Shortcut register"]
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
    #[doc = "0x540..0x550 - Description collection\\[0\\]: Capture/Compare register 0"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x550 - Description collection\\[0\\]: Capture/Compare register 0"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
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
#[doc = "TASKS_CAPTURE (w) register accessor: Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_capture`] module"]
#[doc(alias = "TASKS_CAPTURE")]
pub type TasksCapture = crate::Reg<tasks_capture::TasksCaptureSpec>;
#[doc = "Description collection\\[0\\]: Capture Timer value to CC\\[0\\] register"]
pub mod tasks_capture;
#[doc = "EVENTS_COMPARE (rw) register accessor: Description collection\\[0\\]: Compare event on CC\\[0\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_compare`] module"]
#[doc(alias = "EVENTS_COMPARE")]
pub type EventsCompare = crate::Reg<events_compare::EventsCompareSpec>;
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
pub mod events_compare;
#[doc = "SHORTS (rw) register accessor: Shortcut register\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcut register"]
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
#[doc = "CC (rw) register accessor: Description collection\\[0\\]: Capture/Compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Description collection\\[0\\]: Capture/Compare register 0"]
pub mod cc;
