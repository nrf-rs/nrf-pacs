#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_sample: TasksSample,
    tasks_stop: TasksStop,
    tasks_calibrateoffset: TasksCalibrateoffset,
    _reserved4: [u8; 0xf0],
    events_started: EventsStarted,
    events_end: EventsEnd,
    events_done: EventsDone,
    events_resultdone: EventsResultdone,
    events_calibratedone: EventsCalibratedone,
    events_stopped: EventsStopped,
    events_ch: [EventsCh; 8],
    _reserved11: [u8; 0x01a8],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved14: [u8; 0xf4],
    status: Status,
    _reserved15: [u8; 0xfc],
    enable: Enable,
    _reserved16: [u8; 0x0c],
    ch: [Ch; 8],
    _reserved17: [u8; 0x60],
    resolution: Resolution,
    oversample: Oversample,
    samplerate: Samplerate,
    _reserved20: [u8; 0x30],
    result: Result,
}
impl RegisterBlock {
    #[doc = "0x00 - Start the ADC and prepare the result buffer in RAM"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Take one ADC sample, if scan is enabled all channels are sampled"]
    #[inline(always)]
    pub const fn tasks_sample(&self) -> &TasksSample {
        &self.tasks_sample
    }
    #[doc = "0x08 - Stop the ADC and terminate any on-going conversion"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x0c - Starts offset auto-calibration"]
    #[inline(always)]
    pub const fn tasks_calibrateoffset(&self) -> &TasksCalibrateoffset {
        &self.tasks_calibrateoffset
    }
    #[doc = "0x100 - The ADC has started"]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x104 - The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x108 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    pub const fn events_done(&self) -> &EventsDone {
        &self.events_done
    }
    #[doc = "0x10c - A result is ready to get transferred to RAM."]
    #[inline(always)]
    pub const fn events_resultdone(&self) -> &EventsResultdone {
        &self.events_resultdone
    }
    #[doc = "0x110 - Calibration is complete"]
    #[inline(always)]
    pub const fn events_calibratedone(&self) -> &EventsCalibratedone {
        &self.events_calibratedone
    }
    #[doc = "0x114 - The ADC has stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x118..0x158 - Peripheral events."]
    #[inline(always)]
    pub const fn events_ch(&self, n: usize) -> &EventsCh {
        &self.events_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x158 - Peripheral events."]
    #[inline(always)]
    pub fn events_ch_iter(&self) -> impl Iterator<Item = &EventsCh> {
        self.events_ch.iter()
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
    #[doc = "0x400 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Enable or disable ADC"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x510..0x590 - Unspecified"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x590 - Unspecified"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x5f0 - Resolution configuration"]
    #[inline(always)]
    pub const fn resolution(&self) -> &Resolution {
        &self.resolution
    }
    #[doc = "0x5f4 - Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    #[inline(always)]
    pub const fn oversample(&self) -> &Oversample {
        &self.oversample
    }
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    #[inline(always)]
    pub const fn samplerate(&self) -> &Samplerate {
        &self.samplerate
    }
    #[doc = "0x62c..0x638 - RESULT EasyDMA channel"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
}
#[doc = "TASKS_START (w) register accessor: Start the ADC and prepare the result buffer in RAM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub mod tasks_start;
#[doc = "TASKS_SAMPLE (w) register accessor: Take one ADC sample, if scan is enabled all channels are sampled\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sample::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_sample`] module"]
#[doc(alias = "TASKS_SAMPLE")]
pub type TasksSample = crate::Reg<tasks_sample::TasksSampleSpec>;
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub mod tasks_sample;
#[doc = "TASKS_STOP (w) register accessor: Stop the ADC and terminate any on-going conversion\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub mod tasks_stop;
#[doc = "TASKS_CALIBRATEOFFSET (w) register accessor: Starts offset auto-calibration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_calibrateoffset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_calibrateoffset`] module"]
#[doc(alias = "TASKS_CALIBRATEOFFSET")]
pub type TasksCalibrateoffset = crate::Reg<tasks_calibrateoffset::TasksCalibrateoffsetSpec>;
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "EVENTS_STARTED (rw) register accessor: The ADC has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`] module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "The ADC has started"]
pub mod events_started;
#[doc = "EVENTS_END (rw) register accessor: The ADC has filled up the Result buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`] module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "The ADC has filled up the Result buffer"]
pub mod events_end;
#[doc = "EVENTS_DONE (rw) register accessor: A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_done`] module"]
#[doc(alias = "EVENTS_DONE")]
pub type EventsDone = crate::Reg<events_done::EventsDoneSpec>;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "EVENTS_RESULTDONE (rw) register accessor: A result is ready to get transferred to RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resultdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resultdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_resultdone`] module"]
#[doc(alias = "EVENTS_RESULTDONE")]
pub type EventsResultdone = crate::Reg<events_resultdone::EventsResultdoneSpec>;
#[doc = "A result is ready to get transferred to RAM."]
pub mod events_resultdone;
#[doc = "EVENTS_CALIBRATEDONE (rw) register accessor: Calibration is complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_calibratedone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_calibratedone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_calibratedone`] module"]
#[doc(alias = "EVENTS_CALIBRATEDONE")]
pub type EventsCalibratedone = crate::Reg<events_calibratedone::EventsCalibratedoneSpec>;
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "EVENTS_STOPPED (rw) register accessor: The ADC has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`] module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "The ADC has stopped"]
pub mod events_stopped;
#[doc = "Peripheral events."]
pub use self::events_ch::EventsCh;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_ch;
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
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: Enable or disable ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable or disable ADC"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = "RESOLUTION (rw) register accessor: Resolution configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`resolution::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resolution::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resolution`] module"]
#[doc(alias = "RESOLUTION")]
pub type Resolution = crate::Reg<resolution::ResolutionSpec>;
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "OVERSAMPLE (rw) register accessor: Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used.\n\nYou can [`read`](crate::Reg::read) this register and get [`oversample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oversample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oversample`] module"]
#[doc(alias = "OVERSAMPLE")]
pub type Oversample = crate::Reg<oversample::OversampleSpec>;
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "SAMPLERATE (rw) register accessor: Controls normal or continuous sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`samplerate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samplerate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samplerate`] module"]
#[doc(alias = "SAMPLERATE")]
pub type Samplerate = crate::Reg<samplerate::SamplerateSpec>;
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;
#[doc = "RESULT EasyDMA channel"]
pub use self::result::Result;
#[doc = r"Cluster"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
