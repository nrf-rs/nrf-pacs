#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_clear: TasksClear,
    tasks_trigovrflw: TasksTrigovrflw,
    _reserved4: [u8; 0xf0],
    events_tick: EventsTick,
    events_ovrflw: EventsOvrflw,
    _reserved6: [u8; 0x38],
    events_compare: [EventsCompare; 4],
    _reserved7: [u8; 0x01b4],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved9: [u8; 0x34],
    evten: Evten,
    evtenset: Evtenset,
    evtenclr: Evtenclr,
    _reserved12: [u8; 0x01b8],
    counter: Counter,
    prescaler: Prescaler,
    _reserved14: [u8; 0x34],
    cc: [Cc; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Start RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Clear RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_clear(&self) -> &TasksClear {
        &self.tasks_clear
    }
    #[doc = "0x0c - Set COUNTER to 0xFFFFF0"]
    #[inline(always)]
    pub const fn tasks_trigovrflw(&self) -> &TasksTrigovrflw {
        &self.tasks_trigovrflw
    }
    #[doc = "0x100 - Event on COUNTER increment"]
    #[inline(always)]
    pub const fn events_tick(&self) -> &EventsTick {
        &self.events_tick
    }
    #[doc = "0x104 - Event on COUNTER overflow"]
    #[inline(always)]
    pub const fn events_ovrflw(&self) -> &EventsOvrflw {
        &self.events_ovrflw
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
    #[doc = "0x504 - Current COUNTER value"]
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
    #[doc = "0x508 - 12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x540..0x550 - Description collection\\[0\\]: Compare register 0"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x550 - Description collection\\[0\\]: Compare register 0"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
}
#[doc = "TASKS_START (w) register accessor: Start RTC COUNTER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start RTC COUNTER"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop RTC COUNTER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop RTC COUNTER"]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR (w) register accessor: Clear RTC COUNTER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clear`] module"]
#[doc(alias = "TASKS_CLEAR")]
pub type TasksClear = crate::Reg<tasks_clear::TasksClearSpec>;
#[doc = "Clear RTC COUNTER"]
pub mod tasks_clear;
#[doc = "TASKS_TRIGOVRFLW (w) register accessor: Set COUNTER to 0xFFFFF0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigovrflw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_trigovrflw`] module"]
#[doc(alias = "TASKS_TRIGOVRFLW")]
pub type TasksTrigovrflw = crate::Reg<tasks_trigovrflw::TasksTrigovrflwSpec>;
#[doc = "Set COUNTER to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "EVENTS_TICK (rw) register accessor: Event on COUNTER increment\n\nYou can [`read`](crate::Reg::read) this register and get [`events_tick::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_tick::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_tick`] module"]
#[doc(alias = "EVENTS_TICK")]
pub type EventsTick = crate::Reg<events_tick::EventsTickSpec>;
#[doc = "Event on COUNTER increment"]
pub mod events_tick;
#[doc = "EVENTS_OVRFLW (rw) register accessor: Event on COUNTER overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ovrflw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ovrflw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ovrflw`] module"]
#[doc(alias = "EVENTS_OVRFLW")]
pub type EventsOvrflw = crate::Reg<events_ovrflw::EventsOvrflwSpec>;
#[doc = "Event on COUNTER overflow"]
pub mod events_ovrflw;
#[doc = "EVENTS_COMPARE (rw) register accessor: Description collection\\[0\\]: Compare event on CC\\[0\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_compare`] module"]
#[doc(alias = "EVENTS_COMPARE")]
pub type EventsCompare = crate::Reg<events_compare::EventsCompareSpec>;
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match"]
pub mod events_compare;
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
#[doc = "COUNTER (r) register accessor: Current COUNTER value\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`] module"]
#[doc(alias = "COUNTER")]
pub type Counter = crate::Reg<counter::CounterSpec>;
#[doc = "Current COUNTER value"]
pub mod counter;
#[doc = "PRESCALER (rw) register accessor: 12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`] module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
pub mod prescaler;
#[doc = "CC (rw) register accessor: Description collection\\[0\\]: Compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Description collection\\[0\\]: Compare register 0"]
pub mod cc;
