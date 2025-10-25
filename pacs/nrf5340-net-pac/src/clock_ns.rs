#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_hfclkstart: TasksHfclkstart,
    tasks_hfclkstop: TasksHfclkstop,
    tasks_lfclkstart: TasksLfclkstart,
    tasks_lfclkstop: TasksLfclkstop,
    tasks_cal: TasksCal,
    _reserved5: [u8; 0x6c],
    subscribe_hfclkstart: SubscribeHfclkstart,
    subscribe_hfclkstop: SubscribeHfclkstop,
    subscribe_lfclkstart: SubscribeLfclkstart,
    subscribe_lfclkstop: SubscribeLfclkstop,
    subscribe_cal: SubscribeCal,
    _reserved10: [u8; 0x6c],
    events_hfclkstarted: EventsHfclkstarted,
    events_lfclkstarted: EventsLfclkstarted,
    _reserved12: [u8; 0x14],
    events_done: EventsDone,
    _reserved13: [u8; 0x60],
    publish_hfclkstarted: PublishHfclkstarted,
    publish_lfclkstarted: PublishLfclkstarted,
    _reserved15: [u8; 0x14],
    publish_done: PublishDone,
    _reserved16: [u8; 0x0160],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved20: [u8; 0xf8],
    hfclkrun: Hfclkrun,
    hfclkstat: Hfclkstat,
    _reserved22: [u8; 0x04],
    lfclkrun: Lfclkrun,
    lfclkstat: Lfclkstat,
    lfclksrccopy: Lfclksrccopy,
    _reserved25: [u8; 0xf4],
    hfclksrc: Hfclksrc,
    lfclksrc: Lfclksrc,
    _reserved27: [u8; 0x3c],
    hfclkctrl: Hfclkctrl,
    _reserved28: [u8; 0x14],
    hfclkalwaysrun: Hfclkalwaysrun,
    lfclkalwaysrun: Lfclkalwaysrun,
}
impl RegisterBlock {
    #[doc = "0x00 - Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
    #[inline(always)]
    pub const fn tasks_hfclkstart(&self) -> &TasksHfclkstart {
        &self.tasks_hfclkstart
    }
    #[doc = "0x04 - Stop HFCLK128M/HFCLK64M source"]
    #[inline(always)]
    pub const fn tasks_hfclkstop(&self) -> &TasksHfclkstop {
        &self.tasks_hfclkstop
    }
    #[doc = "0x08 - Start LFCLK source as selected in LFCLKSRC"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(&self) -> &TasksLfclkstart {
        &self.tasks_lfclkstart
    }
    #[doc = "0x0c - Stop LFCLK source"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(&self) -> &TasksLfclkstop {
        &self.tasks_lfclkstop
    }
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    #[inline(always)]
    pub const fn tasks_cal(&self) -> &TasksCal {
        &self.tasks_cal
    }
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclkstart(&self) -> &SubscribeHfclkstart {
        &self.subscribe_hfclkstart
    }
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclkstop(&self) -> &SubscribeHfclkstop {
        &self.subscribe_hfclkstop
    }
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_lfclkstart(&self) -> &SubscribeLfclkstart {
        &self.subscribe_lfclkstart
    }
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_lfclkstop(&self) -> &SubscribeLfclkstop {
        &self.subscribe_lfclkstop
    }
    #[doc = "0x90 - Subscribe configuration for task CAL"]
    #[inline(always)]
    pub const fn subscribe_cal(&self) -> &SubscribeCal {
        &self.subscribe_cal
    }
    #[doc = "0x100 - HFCLK128M/HFCLK64M source started"]
    #[inline(always)]
    pub const fn events_hfclkstarted(&self) -> &EventsHfclkstarted {
        &self.events_hfclkstarted
    }
    #[doc = "0x104 - LFCLK source started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(&self) -> &EventsLfclkstarted {
        &self.events_lfclkstarted
    }
    #[doc = "0x11c - Calibration of LFRC oscillator complete event"]
    #[inline(always)]
    pub const fn events_done(&self) -> &EventsDone {
        &self.events_done
    }
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclkstarted(&self) -> &PublishHfclkstarted {
        &self.publish_hfclkstarted
    }
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_lfclkstarted(&self) -> &PublishLfclkstarted {
        &self.publish_lfclkstarted
    }
    #[doc = "0x19c - Publish configuration for event DONE"]
    #[inline(always)]
    pub const fn publish_done(&self) -> &PublishDone {
        &self.publish_done
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
    #[doc = "0x30c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkrun(&self) -> &Hfclkrun {
        &self.hfclkrun
    }
    #[doc = "0x40c - Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    #[inline(always)]
    pub const fn hfclkstat(&self) -> &Hfclkstat {
        &self.hfclkstat
    }
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclkrun(&self) -> &Lfclkrun {
        &self.lfclkrun
    }
    #[doc = "0x418 - Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    #[inline(always)]
    pub const fn lfclkstat(&self) -> &Lfclkstat {
        &self.lfclkstat
    }
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub const fn lfclksrccopy(&self) -> &Lfclksrccopy {
        &self.lfclksrccopy
    }
    #[doc = "0x514 - Clock source for HFCLK128M/HFCLK64M"]
    #[inline(always)]
    pub const fn hfclksrc(&self) -> &Hfclksrc {
        &self.hfclksrc
    }
    #[doc = "0x518 - Clock source for LFCLK"]
    #[inline(always)]
    pub const fn lfclksrc(&self) -> &Lfclksrc {
        &self.lfclksrc
    }
    #[doc = "0x558 - HFCLK128M frequency configuration"]
    #[inline(always)]
    pub const fn hfclkctrl(&self) -> &Hfclkctrl {
        &self.hfclkctrl
    }
    #[doc = "0x570 - Automatic or manual control of HFCLK128M/HFCLK64M"]
    #[inline(always)]
    pub const fn hfclkalwaysrun(&self) -> &Hfclkalwaysrun {
        &self.hfclkalwaysrun
    }
    #[doc = "0x574 - Automatic or manual control of LFCLK"]
    #[inline(always)]
    pub const fn lfclkalwaysrun(&self) -> &Lfclkalwaysrun {
        &self.lfclkalwaysrun
    }
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstart`] module"]
#[doc(alias = "TASKS_HFCLKSTART")]
pub type TasksHfclkstart = crate::Reg<tasks_hfclkstart::TasksHfclkstartSpec>;
#[doc = "Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: Stop HFCLK128M/HFCLK64M source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstop`] module"]
#[doc(alias = "TASKS_HFCLKSTOP")]
pub type TasksHfclkstop = crate::Reg<tasks_hfclkstop::TasksHfclkstopSpec>;
#[doc = "Stop HFCLK128M/HFCLK64M source"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: Start LFCLK source as selected in LFCLKSRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstart`] module"]
#[doc(alias = "TASKS_LFCLKSTART")]
pub type TasksLfclkstart = crate::Reg<tasks_lfclkstart::TasksLfclkstartSpec>;
#[doc = "Start LFCLK source as selected in LFCLKSRC"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: Stop LFCLK source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstop`] module"]
#[doc(alias = "TASKS_LFCLKSTOP")]
pub type TasksLfclkstop = crate::Reg<tasks_lfclkstop::TasksLfclkstopSpec>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: Start calibration of LFRC oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_cal`] module"]
#[doc(alias = "TASKS_CAL")]
pub type TasksCal = crate::Reg<tasks_cal::TasksCalSpec>;
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "SUBSCRIBE_HFCLKSTART (rw) register accessor: Subscribe configuration for task HFCLKSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclkstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclkstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkstart`] module"]
#[doc(alias = "SUBSCRIBE_HFCLKSTART")]
pub type SubscribeHfclkstart = crate::Reg<subscribe_hfclkstart::SubscribeHfclkstartSpec>;
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "SUBSCRIBE_HFCLKSTOP (rw) register accessor: Subscribe configuration for task HFCLKSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclkstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclkstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkstop`] module"]
#[doc(alias = "SUBSCRIBE_HFCLKSTOP")]
pub type SubscribeHfclkstop = crate::Reg<subscribe_hfclkstop::SubscribeHfclkstopSpec>;
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "SUBSCRIBE_LFCLKSTART (rw) register accessor: Subscribe configuration for task LFCLKSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_lfclkstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_lfclkstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstart`] module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTART")]
pub type SubscribeLfclkstart = crate::Reg<subscribe_lfclkstart::SubscribeLfclkstartSpec>;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "SUBSCRIBE_LFCLKSTOP (rw) register accessor: Subscribe configuration for task LFCLKSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_lfclkstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_lfclkstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstop`] module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTOP")]
pub type SubscribeLfclkstop = crate::Reg<subscribe_lfclkstop::SubscribeLfclkstopSpec>;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "SUBSCRIBE_CAL (rw) register accessor: Subscribe configuration for task CAL\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_cal`] module"]
#[doc(alias = "SUBSCRIBE_CAL")]
pub type SubscribeCal = crate::Reg<subscribe_cal::SubscribeCalSpec>;
#[doc = "Subscribe configuration for task CAL"]
pub mod subscribe_cal;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: HFCLK128M/HFCLK64M source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclkstarted`] module"]
#[doc(alias = "EVENTS_HFCLKSTARTED")]
pub type EventsHfclkstarted = crate::Reg<events_hfclkstarted::EventsHfclkstartedSpec>;
#[doc = "HFCLK128M/HFCLK64M source started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: LFCLK source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lfclkstarted`] module"]
#[doc(alias = "EVENTS_LFCLKSTARTED")]
pub type EventsLfclkstarted = crate::Reg<events_lfclkstarted::EventsLfclkstartedSpec>;
#[doc = "LFCLK source started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: Calibration of LFRC oscillator complete event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_done`] module"]
#[doc(alias = "EVENTS_DONE")]
pub type EventsDone = crate::Reg<events_done::EventsDoneSpec>;
#[doc = "Calibration of LFRC oscillator complete event"]
pub mod events_done;
#[doc = "PUBLISH_HFCLKSTARTED (rw) register accessor: Publish configuration for event HFCLKSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_hfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_hfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_hfclkstarted`] module"]
#[doc(alias = "PUBLISH_HFCLKSTARTED")]
pub type PublishHfclkstarted = crate::Reg<publish_hfclkstarted::PublishHfclkstartedSpec>;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "PUBLISH_LFCLKSTARTED (rw) register accessor: Publish configuration for event LFCLKSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lfclkstarted`] module"]
#[doc(alias = "PUBLISH_LFCLKSTARTED")]
pub type PublishLfclkstarted = crate::Reg<publish_lfclkstarted::PublishLfclkstartedSpec>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
#[doc = "PUBLISH_DONE (rw) register accessor: Publish configuration for event DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_done`] module"]
#[doc(alias = "PUBLISH_DONE")]
pub type PublishDone = crate::Reg<publish_done::PublishDoneSpec>;
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
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
#[doc = "INTPEND (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`] module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "HFCLKRUN (r) register accessor: Status indicating that HFCLKSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkrun`] module"]
#[doc(alias = "HFCLKRUN")]
pub type Hfclkrun = crate::Reg<hfclkrun::HfclkrunSpec>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstat`] module"]
#[doc(alias = "HFCLKSTAT")]
pub type Hfclkstat = crate::Reg<hfclkstat::HfclkstatSpec>;
#[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: Status indicating that LFCLKSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkrun`] module"]
#[doc(alias = "LFCLKRUN")]
pub type Lfclkrun = crate::Reg<lfclkrun::LfclkrunSpec>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkstat`] module"]
#[doc(alias = "LFCLKSTAT")]
pub type Lfclkstat = crate::Reg<lfclkstat::LfclkstatSpec>;
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrccopy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrccopy`] module"]
#[doc(alias = "LFCLKSRCCOPY")]
pub type Lfclksrccopy = crate::Reg<lfclksrccopy::LfclksrccopySpec>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "HFCLKSRC (rw) register accessor: Clock source for HFCLK128M/HFCLK64M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclksrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclksrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclksrc`] module"]
#[doc(alias = "HFCLKSRC")]
pub type Hfclksrc = crate::Reg<hfclksrc::HfclksrcSpec>;
#[doc = "Clock source for HFCLK128M/HFCLK64M"]
pub mod hfclksrc;
#[doc = "LFCLKSRC (rw) register accessor: Clock source for LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrc`] module"]
#[doc(alias = "LFCLKSRC")]
pub type Lfclksrc = crate::Reg<lfclksrc::LfclksrcSpec>;
#[doc = "Clock source for LFCLK"]
pub mod lfclksrc;
#[doc = "HFCLKCTRL (rw) register accessor: HFCLK128M frequency configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkctrl`] module"]
#[doc(alias = "HFCLKCTRL")]
pub type Hfclkctrl = crate::Reg<hfclkctrl::HfclkctrlSpec>;
#[doc = "HFCLK128M frequency configuration"]
pub mod hfclkctrl;
#[doc = "HFCLKALWAYSRUN (rw) register accessor: Automatic or manual control of HFCLK128M/HFCLK64M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkalwaysrun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkalwaysrun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkalwaysrun`] module"]
#[doc(alias = "HFCLKALWAYSRUN")]
pub type Hfclkalwaysrun = crate::Reg<hfclkalwaysrun::HfclkalwaysrunSpec>;
#[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
pub mod hfclkalwaysrun;
#[doc = "LFCLKALWAYSRUN (rw) register accessor: Automatic or manual control of LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkalwaysrun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclkalwaysrun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkalwaysrun`] module"]
#[doc(alias = "LFCLKALWAYSRUN")]
pub type Lfclkalwaysrun = crate::Reg<lfclkalwaysrun::LfclkalwaysrunSpec>;
#[doc = "Automatic or manual control of LFCLK"]
pub mod lfclkalwaysrun;
