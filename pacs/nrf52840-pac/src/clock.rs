#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_hfclkstart: TasksHfclkstart,
    tasks_hfclkstop: TasksHfclkstop,
    tasks_lfclkstart: TasksLfclkstart,
    tasks_lfclkstop: TasksLfclkstop,
    tasks_cal: TasksCal,
    tasks_ctstart: TasksCtstart,
    tasks_ctstop: TasksCtstop,
    _reserved7: [u8; 0xe4],
    events_hfclkstarted: EventsHfclkstarted,
    events_lfclkstarted: EventsLfclkstarted,
    _reserved9: [u8; 0x04],
    events_done: EventsDone,
    events_ctto: EventsCtto,
    _reserved11: [u8; 0x14],
    events_ctstarted: EventsCtstarted,
    events_ctstopped: EventsCtstopped,
    _reserved13: [u8; 0x01d4],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved15: [u8; 0xfc],
    hfclkrun: Hfclkrun,
    hfclkstat: Hfclkstat,
    _reserved17: [u8; 0x04],
    lfclkrun: Lfclkrun,
    lfclkstat: Lfclkstat,
    lfclksrccopy: Lfclksrccopy,
    _reserved20: [u8; 0xf8],
    lfclksrc: Lfclksrc,
    _reserved21: [u8; 0x0c],
    hfxodebounce: Hfxodebounce,
    _reserved22: [u8; 0x0c],
    ctiv: Ctiv,
    _reserved23: [u8; 0x20],
    traceconfig: Traceconfig,
    _reserved24: [u8; 0x54],
    lfrcmode: Lfrcmode,
}
impl RegisterBlock {
    #[doc = "0x00 - Start HFXO crystal oscillator"]
    #[inline(always)]
    pub const fn tasks_hfclkstart(&self) -> &TasksHfclkstart {
        &self.tasks_hfclkstart
    }
    #[doc = "0x04 - Stop HFXO crystal oscillator"]
    #[inline(always)]
    pub const fn tasks_hfclkstop(&self) -> &TasksHfclkstop {
        &self.tasks_hfclkstop
    }
    #[doc = "0x08 - Start LFCLK"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(&self) -> &TasksLfclkstart {
        &self.tasks_lfclkstart
    }
    #[doc = "0x0c - Stop LFCLK"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(&self) -> &TasksLfclkstop {
        &self.tasks_lfclkstop
    }
    #[doc = "0x10 - Start calibration of LFRC"]
    #[inline(always)]
    pub const fn tasks_cal(&self) -> &TasksCal {
        &self.tasks_cal
    }
    #[doc = "0x14 - Start calibration timer"]
    #[inline(always)]
    pub const fn tasks_ctstart(&self) -> &TasksCtstart {
        &self.tasks_ctstart
    }
    #[doc = "0x18 - Stop calibration timer"]
    #[inline(always)]
    pub const fn tasks_ctstop(&self) -> &TasksCtstop {
        &self.tasks_ctstop
    }
    #[doc = "0x100 - HFXO crystal oscillator started"]
    #[inline(always)]
    pub const fn events_hfclkstarted(&self) -> &EventsHfclkstarted {
        &self.events_hfclkstarted
    }
    #[doc = "0x104 - LFCLK started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(&self) -> &EventsLfclkstarted {
        &self.events_lfclkstarted
    }
    #[doc = "0x10c - Calibration of LFRC completed"]
    #[inline(always)]
    pub const fn events_done(&self) -> &EventsDone {
        &self.events_done
    }
    #[doc = "0x110 - Calibration timer timeout"]
    #[inline(always)]
    pub const fn events_ctto(&self) -> &EventsCtto {
        &self.events_ctto
    }
    #[doc = "0x128 - Calibration timer has been started and is ready to process new tasks"]
    #[inline(always)]
    pub const fn events_ctstarted(&self) -> &EventsCtstarted {
        &self.events_ctstarted
    }
    #[doc = "0x12c - Calibration timer has been stopped and is ready to process new tasks"]
    #[inline(always)]
    pub const fn events_ctstopped(&self) -> &EventsCtstopped {
        &self.events_ctstopped
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
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkrun(&self) -> &Hfclkrun {
        &self.hfclkrun
    }
    #[doc = "0x40c - HFCLK status"]
    #[inline(always)]
    pub const fn hfclkstat(&self) -> &Hfclkstat {
        &self.hfclkstat
    }
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclkrun(&self) -> &Lfclkrun {
        &self.lfclkrun
    }
    #[doc = "0x418 - LFCLK status"]
    #[inline(always)]
    pub const fn lfclkstat(&self) -> &Lfclkstat {
        &self.lfclkstat
    }
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub const fn lfclksrccopy(&self) -> &Lfclksrccopy {
        &self.lfclksrccopy
    }
    #[doc = "0x518 - Clock source for the LFCLK"]
    #[inline(always)]
    pub const fn lfclksrc(&self) -> &Lfclksrc {
        &self.lfclksrc
    }
    #[doc = "0x528 - HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
    #[inline(always)]
    pub const fn hfxodebounce(&self) -> &Hfxodebounce {
        &self.hfxodebounce
    }
    #[doc = "0x538 - Calibration timer interval"]
    #[inline(always)]
    pub const fn ctiv(&self) -> &Ctiv {
        &self.ctiv
    }
    #[doc = "0x55c - Clocking options for the trace port debug interface"]
    #[inline(always)]
    pub const fn traceconfig(&self) -> &Traceconfig {
        &self.traceconfig
    }
    #[doc = "0x5b4 - LFRC mode configuration"]
    #[inline(always)]
    pub const fn lfrcmode(&self) -> &Lfrcmode {
        &self.lfrcmode
    }
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: Start HFXO crystal oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstart`] module"]
#[doc(alias = "TASKS_HFCLKSTART")]
pub type TasksHfclkstart = crate::Reg<tasks_hfclkstart::TasksHfclkstartSpec>;
#[doc = "Start HFXO crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: Stop HFXO crystal oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstop`] module"]
#[doc(alias = "TASKS_HFCLKSTOP")]
pub type TasksHfclkstop = crate::Reg<tasks_hfclkstop::TasksHfclkstopSpec>;
#[doc = "Stop HFXO crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: Start LFCLK\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstart`] module"]
#[doc(alias = "TASKS_LFCLKSTART")]
pub type TasksLfclkstart = crate::Reg<tasks_lfclkstart::TasksLfclkstartSpec>;
#[doc = "Start LFCLK"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: Stop LFCLK\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstop`] module"]
#[doc(alias = "TASKS_LFCLKSTOP")]
pub type TasksLfclkstop = crate::Reg<tasks_lfclkstop::TasksLfclkstopSpec>;
#[doc = "Stop LFCLK"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: Start calibration of LFRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_cal`] module"]
#[doc(alias = "TASKS_CAL")]
pub type TasksCal = crate::Reg<tasks_cal::TasksCalSpec>;
#[doc = "Start calibration of LFRC"]
pub mod tasks_cal;
#[doc = "TASKS_CTSTART (w) register accessor: Start calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ctstart`] module"]
#[doc(alias = "TASKS_CTSTART")]
pub type TasksCtstart = crate::Reg<tasks_ctstart::TasksCtstartSpec>;
#[doc = "Start calibration timer"]
pub mod tasks_ctstart;
#[doc = "TASKS_CTSTOP (w) register accessor: Stop calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ctstop`] module"]
#[doc(alias = "TASKS_CTSTOP")]
pub type TasksCtstop = crate::Reg<tasks_ctstop::TasksCtstopSpec>;
#[doc = "Stop calibration timer"]
pub mod tasks_ctstop;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: HFXO crystal oscillator started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclkstarted`] module"]
#[doc(alias = "EVENTS_HFCLKSTARTED")]
pub type EventsHfclkstarted = crate::Reg<events_hfclkstarted::EventsHfclkstartedSpec>;
#[doc = "HFXO crystal oscillator started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: LFCLK started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lfclkstarted`] module"]
#[doc(alias = "EVENTS_LFCLKSTARTED")]
pub type EventsLfclkstarted = crate::Reg<events_lfclkstarted::EventsLfclkstartedSpec>;
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: Calibration of LFRC completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_done`] module"]
#[doc(alias = "EVENTS_DONE")]
pub type EventsDone = crate::Reg<events_done::EventsDoneSpec>;
#[doc = "Calibration of LFRC completed"]
pub mod events_done;
#[doc = "EVENTS_CTTO (rw) register accessor: Calibration timer timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctto`] module"]
#[doc(alias = "EVENTS_CTTO")]
pub type EventsCtto = crate::Reg<events_ctto::EventsCttoSpec>;
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
#[doc = "EVENTS_CTSTARTED (rw) register accessor: Calibration timer has been started and is ready to process new tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctstarted`] module"]
#[doc(alias = "EVENTS_CTSTARTED")]
pub type EventsCtstarted = crate::Reg<events_ctstarted::EventsCtstartedSpec>;
#[doc = "Calibration timer has been started and is ready to process new tasks"]
pub mod events_ctstarted;
#[doc = "EVENTS_CTSTOPPED (rw) register accessor: Calibration timer has been stopped and is ready to process new tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctstopped`] module"]
#[doc(alias = "EVENTS_CTSTOPPED")]
pub type EventsCtstopped = crate::Reg<events_ctstopped::EventsCtstoppedSpec>;
#[doc = "Calibration timer has been stopped and is ready to process new tasks"]
pub mod events_ctstopped;
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
#[doc = "HFCLKRUN (r) register accessor: Status indicating that HFCLKSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkrun`] module"]
#[doc(alias = "HFCLKRUN")]
pub type Hfclkrun = crate::Reg<hfclkrun::HfclkrunSpec>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: HFCLK status\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstat`] module"]
#[doc(alias = "HFCLKSTAT")]
pub type Hfclkstat = crate::Reg<hfclkstat::HfclkstatSpec>;
#[doc = "HFCLK status"]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: Status indicating that LFCLKSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkrun`] module"]
#[doc(alias = "LFCLKRUN")]
pub type Lfclkrun = crate::Reg<lfclkrun::LfclkrunSpec>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: LFCLK status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkstat`] module"]
#[doc(alias = "LFCLKSTAT")]
pub type Lfclkstat = crate::Reg<lfclkstat::LfclkstatSpec>;
#[doc = "LFCLK status"]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrccopy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrccopy`] module"]
#[doc(alias = "LFCLKSRCCOPY")]
pub type Lfclksrccopy = crate::Reg<lfclksrccopy::LfclksrccopySpec>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC (rw) register accessor: Clock source for the LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrc`] module"]
#[doc(alias = "LFCLKSRC")]
pub type Lfclksrc = crate::Reg<lfclksrc::LfclksrcSpec>;
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "HFXODEBOUNCE (rw) register accessor: HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxodebounce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxodebounce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxodebounce`] module"]
#[doc(alias = "HFXODEBOUNCE")]
pub type Hfxodebounce = crate::Reg<hfxodebounce::HfxodebounceSpec>;
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
pub mod hfxodebounce;
#[doc = "CTIV (rw) register accessor: Calibration timer interval\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiv`] module"]
#[doc(alias = "CTIV")]
pub type Ctiv = crate::Reg<ctiv::CtivSpec>;
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "TRACECONFIG (rw) register accessor: Clocking options for the trace port debug interface\n\nYou can [`read`](crate::Reg::read) this register and get [`traceconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceconfig`] module"]
#[doc(alias = "TRACECONFIG")]
pub type Traceconfig = crate::Reg<traceconfig::TraceconfigSpec>;
#[doc = "Clocking options for the trace port debug interface"]
pub mod traceconfig;
#[doc = "LFRCMODE (rw) register accessor: LFRC mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrcmode`] module"]
#[doc(alias = "LFRCMODE")]
pub type Lfrcmode = crate::Reg<lfrcmode::LfrcmodeSpec>;
#[doc = "LFRC mode configuration"]
pub mod lfrcmode;
