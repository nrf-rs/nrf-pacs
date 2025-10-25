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
    _reserved11: [u8; 0x01f0],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved13: [u8; 0xfc],
    hfclkrun: Hfclkrun,
    hfclkstat: Hfclkstat,
    _reserved15: [u8; 0x04],
    lfclkrun: Lfclkrun,
    lfclkstat: Lfclkstat,
    lfclksrccopy: Lfclksrccopy,
    _reserved18: [u8; 0xf8],
    lfclksrc: Lfclksrc,
    _reserved19: [u8; 0x1c],
    ctiv: Ctiv,
    _reserved20: [u8; 0x14],
    xtalfreq: Xtalfreq,
}
impl RegisterBlock {
    #[doc = "0x00 - Start HFCLK clock source."]
    #[inline(always)]
    pub const fn tasks_hfclkstart(&self) -> &TasksHfclkstart {
        &self.tasks_hfclkstart
    }
    #[doc = "0x04 - Stop HFCLK clock source."]
    #[inline(always)]
    pub const fn tasks_hfclkstop(&self) -> &TasksHfclkstop {
        &self.tasks_hfclkstop
    }
    #[doc = "0x08 - Start LFCLK clock source."]
    #[inline(always)]
    pub const fn tasks_lfclkstart(&self) -> &TasksLfclkstart {
        &self.tasks_lfclkstart
    }
    #[doc = "0x0c - Stop LFCLK clock source."]
    #[inline(always)]
    pub const fn tasks_lfclkstop(&self) -> &TasksLfclkstop {
        &self.tasks_lfclkstop
    }
    #[doc = "0x10 - Start calibration of LFCLK RC oscillator."]
    #[inline(always)]
    pub const fn tasks_cal(&self) -> &TasksCal {
        &self.tasks_cal
    }
    #[doc = "0x14 - Start calibration timer."]
    #[inline(always)]
    pub const fn tasks_ctstart(&self) -> &TasksCtstart {
        &self.tasks_ctstart
    }
    #[doc = "0x18 - Stop calibration timer."]
    #[inline(always)]
    pub const fn tasks_ctstop(&self) -> &TasksCtstop {
        &self.tasks_ctstop
    }
    #[doc = "0x100 - HFCLK oscillator started."]
    #[inline(always)]
    pub const fn events_hfclkstarted(&self) -> &EventsHfclkstarted {
        &self.events_hfclkstarted
    }
    #[doc = "0x104 - LFCLK oscillator started."]
    #[inline(always)]
    pub const fn events_lfclkstarted(&self) -> &EventsLfclkstarted {
        &self.events_lfclkstarted
    }
    #[doc = "0x10c - Calibration of LFCLK RC oscillator completed."]
    #[inline(always)]
    pub const fn events_done(&self) -> &EventsDone {
        &self.events_done
    }
    #[doc = "0x110 - Calibration timer timeout."]
    #[inline(always)]
    pub const fn events_ctto(&self) -> &EventsCtto {
        &self.events_ctto
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
    #[doc = "0x408 - Task HFCLKSTART trigger status."]
    #[inline(always)]
    pub const fn hfclkrun(&self) -> &Hfclkrun {
        &self.hfclkrun
    }
    #[doc = "0x40c - High frequency clock status."]
    #[inline(always)]
    pub const fn hfclkstat(&self) -> &Hfclkstat {
        &self.hfclkstat
    }
    #[doc = "0x414 - Task LFCLKSTART triggered status."]
    #[inline(always)]
    pub const fn lfclkrun(&self) -> &Lfclkrun {
        &self.lfclkrun
    }
    #[doc = "0x418 - Low frequency clock status."]
    #[inline(always)]
    pub const fn lfclkstat(&self) -> &Lfclkstat {
        &self.lfclkstat
    }
    #[doc = "0x41c - Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
    #[inline(always)]
    pub const fn lfclksrccopy(&self) -> &Lfclksrccopy {
        &self.lfclksrccopy
    }
    #[doc = "0x518 - Clock source for the LFCLK clock."]
    #[inline(always)]
    pub const fn lfclksrc(&self) -> &Lfclksrc {
        &self.lfclksrc
    }
    #[doc = "0x538 - Calibration timer interval."]
    #[inline(always)]
    pub const fn ctiv(&self) -> &Ctiv {
        &self.ctiv
    }
    #[doc = "0x550 - Crystal frequency."]
    #[inline(always)]
    pub const fn xtalfreq(&self) -> &Xtalfreq {
        &self.xtalfreq
    }
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: Start HFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstart`] module"]
#[doc(alias = "TASKS_HFCLKSTART")]
pub type TasksHfclkstart = crate::Reg<tasks_hfclkstart::TasksHfclkstartSpec>;
#[doc = "Start HFCLK clock source."]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: Stop HFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstop`] module"]
#[doc(alias = "TASKS_HFCLKSTOP")]
pub type TasksHfclkstop = crate::Reg<tasks_hfclkstop::TasksHfclkstopSpec>;
#[doc = "Stop HFCLK clock source."]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: Start LFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstart`] module"]
#[doc(alias = "TASKS_LFCLKSTART")]
pub type TasksLfclkstart = crate::Reg<tasks_lfclkstart::TasksLfclkstartSpec>;
#[doc = "Start LFCLK clock source."]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: Stop LFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstop`] module"]
#[doc(alias = "TASKS_LFCLKSTOP")]
pub type TasksLfclkstop = crate::Reg<tasks_lfclkstop::TasksLfclkstopSpec>;
#[doc = "Stop LFCLK clock source."]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: Start calibration of LFCLK RC oscillator.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_cal`] module"]
#[doc(alias = "TASKS_CAL")]
pub type TasksCal = crate::Reg<tasks_cal::TasksCalSpec>;
#[doc = "Start calibration of LFCLK RC oscillator."]
pub mod tasks_cal;
#[doc = "TASKS_CTSTART (w) register accessor: Start calibration timer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ctstart`] module"]
#[doc(alias = "TASKS_CTSTART")]
pub type TasksCtstart = crate::Reg<tasks_ctstart::TasksCtstartSpec>;
#[doc = "Start calibration timer."]
pub mod tasks_ctstart;
#[doc = "TASKS_CTSTOP (w) register accessor: Stop calibration timer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ctstop`] module"]
#[doc(alias = "TASKS_CTSTOP")]
pub type TasksCtstop = crate::Reg<tasks_ctstop::TasksCtstopSpec>;
#[doc = "Stop calibration timer."]
pub mod tasks_ctstop;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: HFCLK oscillator started.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclkstarted`] module"]
#[doc(alias = "EVENTS_HFCLKSTARTED")]
pub type EventsHfclkstarted = crate::Reg<events_hfclkstarted::EventsHfclkstartedSpec>;
#[doc = "HFCLK oscillator started."]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: LFCLK oscillator started.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lfclkstarted`] module"]
#[doc(alias = "EVENTS_LFCLKSTARTED")]
pub type EventsLfclkstarted = crate::Reg<events_lfclkstarted::EventsLfclkstartedSpec>;
#[doc = "LFCLK oscillator started."]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: Calibration of LFCLK RC oscillator completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_done`] module"]
#[doc(alias = "EVENTS_DONE")]
pub type EventsDone = crate::Reg<events_done::EventsDoneSpec>;
#[doc = "Calibration of LFCLK RC oscillator completed."]
pub mod events_done;
#[doc = "EVENTS_CTTO (rw) register accessor: Calibration timer timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctto`] module"]
#[doc(alias = "EVENTS_CTTO")]
pub type EventsCtto = crate::Reg<events_ctto::EventsCttoSpec>;
#[doc = "Calibration timer timeout."]
pub mod events_ctto;
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
#[doc = "HFCLKRUN (r) register accessor: Task HFCLKSTART trigger status.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkrun`] module"]
#[doc(alias = "HFCLKRUN")]
pub type Hfclkrun = crate::Reg<hfclkrun::HfclkrunSpec>;
#[doc = "Task HFCLKSTART trigger status."]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: High frequency clock status.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstat`] module"]
#[doc(alias = "HFCLKSTAT")]
pub type Hfclkstat = crate::Reg<hfclkstat::HfclkstatSpec>;
#[doc = "High frequency clock status."]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: Task LFCLKSTART triggered status.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkrun`] module"]
#[doc(alias = "LFCLKRUN")]
pub type Lfclkrun = crate::Reg<lfclkrun::LfclkrunSpec>;
#[doc = "Task LFCLKSTART triggered status."]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: Low frequency clock status.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkstat`] module"]
#[doc(alias = "LFCLKSTAT")]
pub type Lfclkstat = crate::Reg<lfclkstat::LfclkstatSpec>;
#[doc = "Low frequency clock status."]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: Clock source for the LFCLK clock, set when task LKCLKSTART is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrccopy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrccopy`] module"]
#[doc(alias = "LFCLKSRCCOPY")]
pub type Lfclksrccopy = crate::Reg<lfclksrccopy::LfclksrccopySpec>;
#[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC (rw) register accessor: Clock source for the LFCLK clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrc`] module"]
#[doc(alias = "LFCLKSRC")]
pub type Lfclksrc = crate::Reg<lfclksrc::LfclksrcSpec>;
#[doc = "Clock source for the LFCLK clock."]
pub mod lfclksrc;
#[doc = "CTIV (rw) register accessor: Calibration timer interval.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiv`] module"]
#[doc(alias = "CTIV")]
pub type Ctiv = crate::Reg<ctiv::CtivSpec>;
#[doc = "Calibration timer interval."]
pub mod ctiv;
#[doc = "XTALFREQ (rw) register accessor: Crystal frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalfreq`] module"]
#[doc(alias = "XTALFREQ")]
pub type Xtalfreq = crate::Reg<xtalfreq::XtalfreqSpec>;
#[doc = "Crystal frequency."]
pub mod xtalfreq;
