#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_hfclkstart: TasksHfclkstart,
    tasks_hfclkstop: TasksHfclkstop,
    tasks_lfclkstart: TasksLfclkstart,
    tasks_lfclkstop: TasksLfclkstop,
    tasks_cal: TasksCal,
    _reserved5: [u8; 0x04],
    tasks_hfclkaudiostart: TasksHfclkaudiostart,
    tasks_hfclkaudiostop: TasksHfclkaudiostop,
    tasks_hfclk192mstart: TasksHfclk192mstart,
    tasks_hfclk192mstop: TasksHfclk192mstop,
    _reserved9: [u8; 0x58],
    subscribe_hfclkstart: SubscribeHfclkstart,
    subscribe_hfclkstop: SubscribeHfclkstop,
    subscribe_lfclkstart: SubscribeLfclkstart,
    subscribe_lfclkstop: SubscribeLfclkstop,
    subscribe_cal: SubscribeCal,
    _reserved14: [u8; 0x04],
    subscribe_hfclkaudiostart: SubscribeHfclkaudiostart,
    subscribe_hfclkaudiostop: SubscribeHfclkaudiostop,
    subscribe_hfclk192mstart: SubscribeHfclk192mstart,
    subscribe_hfclk192mstop: SubscribeHfclk192mstop,
    _reserved18: [u8; 0x58],
    events_hfclkstarted: EventsHfclkstarted,
    events_lfclkstarted: EventsLfclkstarted,
    _reserved20: [u8; 0x14],
    events_done: EventsDone,
    events_hfclkaudiostarted: EventsHfclkaudiostarted,
    events_hfclk192mstarted: EventsHfclk192mstarted,
    _reserved23: [u8; 0x58],
    publish_hfclkstarted: PublishHfclkstarted,
    publish_lfclkstarted: PublishLfclkstarted,
    _reserved25: [u8; 0x14],
    publish_done: PublishDone,
    publish_hfclkaudiostarted: PublishHfclkaudiostarted,
    publish_hfclk192mstarted: PublishHfclk192mstarted,
    _reserved28: [u8; 0x0158],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved32: [u8; 0xf8],
    hfclkrun: Hfclkrun,
    hfclkstat: Hfclkstat,
    _reserved34: [u8; 0x04],
    lfclkrun: Lfclkrun,
    lfclkstat: Lfclkstat,
    lfclksrccopy: Lfclksrccopy,
    _reserved37: [u8; 0x30],
    hfclkaudiorun: Hfclkaudiorun,
    hfclkaudiostat: Hfclkaudiostat,
    hfclk192mrun: Hfclk192mrun,
    hfclk192mstat: Hfclk192mstat,
    _reserved41: [u8; 0xb4],
    hfclksrc: Hfclksrc,
    lfclksrc: Lfclksrc,
    _reserved43: [u8; 0x3c],
    hfclkctrl: Hfclkctrl,
    hfclkaudio: Hfclkaudio,
    _reserved45: [u8; 0x10],
    hfclkalwaysrun: Hfclkalwaysrun,
    lfclkalwaysrun: Lfclkalwaysrun,
    _reserved47: [u8; 0x04],
    hfclkaudioalwaysrun: Hfclkaudioalwaysrun,
    hfclk192msrc: Hfclk192msrc,
    hfclk192malwaysrun: Hfclk192malwaysrun,
    _reserved50: [u8; 0x30],
    hfclk192mctrl: Hfclk192mctrl,
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
    #[doc = "0x18 - Start HFCLKAUDIO source"]
    #[inline(always)]
    pub const fn tasks_hfclkaudiostart(&self) -> &TasksHfclkaudiostart {
        &self.tasks_hfclkaudiostart
    }
    #[doc = "0x1c - Stop HFCLKAUDIO source"]
    #[inline(always)]
    pub const fn tasks_hfclkaudiostop(&self) -> &TasksHfclkaudiostop {
        &self.tasks_hfclkaudiostop
    }
    #[doc = "0x20 - Start HFCLK192M source as selected in HFCLK192MSRC"]
    #[inline(always)]
    pub const fn tasks_hfclk192mstart(&self) -> &TasksHfclk192mstart {
        &self.tasks_hfclk192mstart
    }
    #[doc = "0x24 - Stop HFCLK192M source"]
    #[inline(always)]
    pub const fn tasks_hfclk192mstop(&self) -> &TasksHfclk192mstop {
        &self.tasks_hfclk192mstop
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
    #[doc = "0x98 - Subscribe configuration for task HFCLKAUDIOSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclkaudiostart(&self) -> &SubscribeHfclkaudiostart {
        &self.subscribe_hfclkaudiostart
    }
    #[doc = "0x9c - Subscribe configuration for task HFCLKAUDIOSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclkaudiostop(&self) -> &SubscribeHfclkaudiostop {
        &self.subscribe_hfclkaudiostop
    }
    #[doc = "0xa0 - Subscribe configuration for task HFCLK192MSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclk192mstart(&self) -> &SubscribeHfclk192mstart {
        &self.subscribe_hfclk192mstart
    }
    #[doc = "0xa4 - Subscribe configuration for task HFCLK192MSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclk192mstop(&self) -> &SubscribeHfclk192mstop {
        &self.subscribe_hfclk192mstop
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
    #[doc = "0x120 - HFCLKAUDIO source started"]
    #[inline(always)]
    pub const fn events_hfclkaudiostarted(&self) -> &EventsHfclkaudiostarted {
        &self.events_hfclkaudiostarted
    }
    #[doc = "0x124 - HFCLK192M source started"]
    #[inline(always)]
    pub const fn events_hfclk192mstarted(&self) -> &EventsHfclk192mstarted {
        &self.events_hfclk192mstarted
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
    #[doc = "0x1a0 - Publish configuration for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclkaudiostarted(&self) -> &PublishHfclkaudiostarted {
        &self.publish_hfclkaudiostarted
    }
    #[doc = "0x1a4 - Publish configuration for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclk192mstarted(&self) -> &PublishHfclk192mstarted {
        &self.publish_hfclk192mstarted
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
    #[doc = "0x450 - Status indicating that HFCLKAUDIOSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkaudiorun(&self) -> &Hfclkaudiorun {
        &self.hfclkaudiorun
    }
    #[doc = "0x454 - Status indicating which HFCLKAUDIO source is running"]
    #[inline(always)]
    pub const fn hfclkaudiostat(&self) -> &Hfclkaudiostat {
        &self.hfclkaudiostat
    }
    #[doc = "0x458 - Status indicating that HFCLK192MSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclk192mrun(&self) -> &Hfclk192mrun {
        &self.hfclk192mrun
    }
    #[doc = "0x45c - Status indicating which HFCLK192M source is running"]
    #[inline(always)]
    pub const fn hfclk192mstat(&self) -> &Hfclk192mstat {
        &self.hfclk192mstat
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
    #[doc = "0x55c - Unspecified"]
    #[inline(always)]
    pub const fn hfclkaudio(&self) -> &Hfclkaudio {
        &self.hfclkaudio
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
    #[doc = "0x57c - Automatic or manual control of HFCLKAUDIO"]
    #[inline(always)]
    pub const fn hfclkaudioalwaysrun(&self) -> &Hfclkaudioalwaysrun {
        &self.hfclkaudioalwaysrun
    }
    #[doc = "0x580 - Clock source for HFCLK192M"]
    #[inline(always)]
    pub const fn hfclk192msrc(&self) -> &Hfclk192msrc {
        &self.hfclk192msrc
    }
    #[doc = "0x584 - Automatic or manual control of HFCLK192M"]
    #[inline(always)]
    pub const fn hfclk192malwaysrun(&self) -> &Hfclk192malwaysrun {
        &self.hfclk192malwaysrun
    }
    #[doc = "0x5b8 - HFCLK192M frequency configuration"]
    #[inline(always)]
    pub const fn hfclk192mctrl(&self) -> &Hfclk192mctrl {
        &self.hfclk192mctrl
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
#[doc = "TASKS_HFCLKAUDIOSTART (w) register accessor: Start HFCLKAUDIO source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkaudiostart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkaudiostart`] module"]
#[doc(alias = "TASKS_HFCLKAUDIOSTART")]
pub type TasksHfclkaudiostart = crate::Reg<tasks_hfclkaudiostart::TasksHfclkaudiostartSpec>;
#[doc = "Start HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostart;
#[doc = "TASKS_HFCLKAUDIOSTOP (w) register accessor: Stop HFCLKAUDIO source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkaudiostop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkaudiostop`] module"]
#[doc(alias = "TASKS_HFCLKAUDIOSTOP")]
pub type TasksHfclkaudiostop = crate::Reg<tasks_hfclkaudiostop::TasksHfclkaudiostopSpec>;
#[doc = "Stop HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostop;
#[doc = "TASKS_HFCLK192MSTART (w) register accessor: Start HFCLK192M source as selected in HFCLK192MSRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclk192mstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclk192mstart`] module"]
#[doc(alias = "TASKS_HFCLK192MSTART")]
pub type TasksHfclk192mstart = crate::Reg<tasks_hfclk192mstart::TasksHfclk192mstartSpec>;
#[doc = "Start HFCLK192M source as selected in HFCLK192MSRC"]
pub mod tasks_hfclk192mstart;
#[doc = "TASKS_HFCLK192MSTOP (w) register accessor: Stop HFCLK192M source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclk192mstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclk192mstop`] module"]
#[doc(alias = "TASKS_HFCLK192MSTOP")]
pub type TasksHfclk192mstop = crate::Reg<tasks_hfclk192mstop::TasksHfclk192mstopSpec>;
#[doc = "Stop HFCLK192M source"]
pub mod tasks_hfclk192mstop;
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
#[doc = "SUBSCRIBE_HFCLKAUDIOSTART (rw) register accessor: Subscribe configuration for task HFCLKAUDIOSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclkaudiostart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclkaudiostart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkaudiostart`] module"]
#[doc(alias = "SUBSCRIBE_HFCLKAUDIOSTART")]
pub type SubscribeHfclkaudiostart =
    crate::Reg<subscribe_hfclkaudiostart::SubscribeHfclkaudiostartSpec>;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTART"]
pub mod subscribe_hfclkaudiostart;
#[doc = "SUBSCRIBE_HFCLKAUDIOSTOP (rw) register accessor: Subscribe configuration for task HFCLKAUDIOSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclkaudiostop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclkaudiostop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkaudiostop`] module"]
#[doc(alias = "SUBSCRIBE_HFCLKAUDIOSTOP")]
pub type SubscribeHfclkaudiostop =
    crate::Reg<subscribe_hfclkaudiostop::SubscribeHfclkaudiostopSpec>;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTOP"]
pub mod subscribe_hfclkaudiostop;
#[doc = "SUBSCRIBE_HFCLK192MSTART (rw) register accessor: Subscribe configuration for task HFCLK192MSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclk192mstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclk192mstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclk192mstart`] module"]
#[doc(alias = "SUBSCRIBE_HFCLK192MSTART")]
pub type SubscribeHfclk192mstart =
    crate::Reg<subscribe_hfclk192mstart::SubscribeHfclk192mstartSpec>;
#[doc = "Subscribe configuration for task HFCLK192MSTART"]
pub mod subscribe_hfclk192mstart;
#[doc = "SUBSCRIBE_HFCLK192MSTOP (rw) register accessor: Subscribe configuration for task HFCLK192MSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_hfclk192mstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_hfclk192mstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclk192mstop`] module"]
#[doc(alias = "SUBSCRIBE_HFCLK192MSTOP")]
pub type SubscribeHfclk192mstop = crate::Reg<subscribe_hfclk192mstop::SubscribeHfclk192mstopSpec>;
#[doc = "Subscribe configuration for task HFCLK192MSTOP"]
pub mod subscribe_hfclk192mstop;
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
#[doc = "EVENTS_HFCLKAUDIOSTARTED (rw) register accessor: HFCLKAUDIO source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkaudiostarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkaudiostarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclkaudiostarted`] module"]
#[doc(alias = "EVENTS_HFCLKAUDIOSTARTED")]
pub type EventsHfclkaudiostarted =
    crate::Reg<events_hfclkaudiostarted::EventsHfclkaudiostartedSpec>;
#[doc = "HFCLKAUDIO source started"]
pub mod events_hfclkaudiostarted;
#[doc = "EVENTS_HFCLK192MSTARTED (rw) register accessor: HFCLK192M source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclk192mstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclk192mstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclk192mstarted`] module"]
#[doc(alias = "EVENTS_HFCLK192MSTARTED")]
pub type EventsHfclk192mstarted = crate::Reg<events_hfclk192mstarted::EventsHfclk192mstartedSpec>;
#[doc = "HFCLK192M source started"]
pub mod events_hfclk192mstarted;
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
#[doc = "PUBLISH_HFCLKAUDIOSTARTED (rw) register accessor: Publish configuration for event HFCLKAUDIOSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_hfclkaudiostarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_hfclkaudiostarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_hfclkaudiostarted`] module"]
#[doc(alias = "PUBLISH_HFCLKAUDIOSTARTED")]
pub type PublishHfclkaudiostarted =
    crate::Reg<publish_hfclkaudiostarted::PublishHfclkaudiostartedSpec>;
#[doc = "Publish configuration for event HFCLKAUDIOSTARTED"]
pub mod publish_hfclkaudiostarted;
#[doc = "PUBLISH_HFCLK192MSTARTED (rw) register accessor: Publish configuration for event HFCLK192MSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_hfclk192mstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_hfclk192mstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_hfclk192mstarted`] module"]
#[doc(alias = "PUBLISH_HFCLK192MSTARTED")]
pub type PublishHfclk192mstarted =
    crate::Reg<publish_hfclk192mstarted::PublishHfclk192mstartedSpec>;
#[doc = "Publish configuration for event HFCLK192MSTARTED"]
pub mod publish_hfclk192mstarted;
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
#[doc = "HFCLKAUDIORUN (r) register accessor: Status indicating that HFCLKAUDIOSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkaudiorun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkaudiorun`] module"]
#[doc(alias = "HFCLKAUDIORUN")]
pub type Hfclkaudiorun = crate::Reg<hfclkaudiorun::HfclkaudiorunSpec>;
#[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered"]
pub mod hfclkaudiorun;
#[doc = "HFCLKAUDIOSTAT (r) register accessor: Status indicating which HFCLKAUDIO source is running\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkaudiostat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkaudiostat`] module"]
#[doc(alias = "HFCLKAUDIOSTAT")]
pub type Hfclkaudiostat = crate::Reg<hfclkaudiostat::HfclkaudiostatSpec>;
#[doc = "Status indicating which HFCLKAUDIO source is running"]
pub mod hfclkaudiostat;
#[doc = "HFCLK192MRUN (r) register accessor: Status indicating that HFCLK192MSTART task has been triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192mrun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclk192mrun`] module"]
#[doc(alias = "HFCLK192MRUN")]
pub type Hfclk192mrun = crate::Reg<hfclk192mrun::Hfclk192mrunSpec>;
#[doc = "Status indicating that HFCLK192MSTART task has been triggered"]
pub mod hfclk192mrun;
#[doc = "HFCLK192MSTAT (r) register accessor: Status indicating which HFCLK192M source is running\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192mstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclk192mstat`] module"]
#[doc(alias = "HFCLK192MSTAT")]
pub type Hfclk192mstat = crate::Reg<hfclk192mstat::Hfclk192mstatSpec>;
#[doc = "Status indicating which HFCLK192M source is running"]
pub mod hfclk192mstat;
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
#[doc = "Unspecified"]
pub use self::hfclkaudio::Hfclkaudio;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod hfclkaudio;
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
#[doc = "HFCLKAUDIOALWAYSRUN (rw) register accessor: Automatic or manual control of HFCLKAUDIO\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkaudioalwaysrun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkaudioalwaysrun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkaudioalwaysrun`] module"]
#[doc(alias = "HFCLKAUDIOALWAYSRUN")]
pub type Hfclkaudioalwaysrun = crate::Reg<hfclkaudioalwaysrun::HfclkaudioalwaysrunSpec>;
#[doc = "Automatic or manual control of HFCLKAUDIO"]
pub mod hfclkaudioalwaysrun;
#[doc = "HFCLK192MSRC (rw) register accessor: Clock source for HFCLK192M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192msrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclk192msrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclk192msrc`] module"]
#[doc(alias = "HFCLK192MSRC")]
pub type Hfclk192msrc = crate::Reg<hfclk192msrc::Hfclk192msrcSpec>;
#[doc = "Clock source for HFCLK192M"]
pub mod hfclk192msrc;
#[doc = "HFCLK192MALWAYSRUN (rw) register accessor: Automatic or manual control of HFCLK192M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192malwaysrun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclk192malwaysrun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclk192malwaysrun`] module"]
#[doc(alias = "HFCLK192MALWAYSRUN")]
pub type Hfclk192malwaysrun = crate::Reg<hfclk192malwaysrun::Hfclk192malwaysrunSpec>;
#[doc = "Automatic or manual control of HFCLK192M"]
pub mod hfclk192malwaysrun;
#[doc = "HFCLK192MCTRL (rw) register accessor: HFCLK192M frequency configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclk192mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclk192mctrl`] module"]
#[doc(alias = "HFCLK192MCTRL")]
pub type Hfclk192mctrl = crate::Reg<hfclk192mctrl::Hfclk192mctrlSpec>;
#[doc = "HFCLK192M frequency configuration"]
pub mod hfclk192mctrl;
