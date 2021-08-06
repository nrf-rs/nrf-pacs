#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
    pub tasks_hfclkstart: crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>,
    #[doc = "0x04 - Stop HFCLK128M/HFCLK64M source"]
    pub tasks_hfclkstop: crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>,
    #[doc = "0x08 - Start LFCLK source as selected in LFCLKSRC"]
    pub tasks_lfclkstart: crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>,
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    pub tasks_cal: crate::Reg<tasks_cal::TASKS_CAL_SPEC>,
    _reserved5: [u8; 0x6c],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: crate::Reg<subscribe_hfclkstart::SUBSCRIBE_HFCLKSTART_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: crate::Reg<subscribe_hfclkstop::SUBSCRIBE_HFCLKSTOP_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: crate::Reg<subscribe_lfclkstart::SUBSCRIBE_LFCLKSTART_SPEC>,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: crate::Reg<subscribe_lfclkstop::SUBSCRIBE_LFCLKSTOP_SPEC>,
    #[doc = "0x90 - Subscribe configuration for task CAL"]
    pub subscribe_cal: crate::Reg<subscribe_cal::SUBSCRIBE_CAL_SPEC>,
    _reserved10: [u8; 0x6c],
    #[doc = "0x100 - HFCLK128M/HFCLK64M source started"]
    pub events_hfclkstarted: crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>,
    #[doc = "0x104 - LFCLK source started"]
    pub events_lfclkstarted: crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>,
    _reserved12: [u8; 0x14],
    #[doc = "0x11c - Calibration of LFRC oscillator complete event"]
    pub events_done: crate::Reg<events_done::EVENTS_DONE_SPEC>,
    _reserved13: [u8; 0x60],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: crate::Reg<publish_hfclkstarted::PUBLISH_HFCLKSTARTED_SPEC>,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: crate::Reg<publish_lfclkstarted::PUBLISH_LFCLKSTARTED_SPEC>,
    _reserved15: [u8; 0x14],
    #[doc = "0x19c - Publish configuration for event DONE"]
    pub publish_done: crate::Reg<publish_done::PUBLISH_DONE_SPEC>,
    _reserved16: [u8; 0x0160],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: crate::Reg<intpend::INTPEND_SPEC>,
    _reserved20: [u8; 0xf8],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: crate::Reg<hfclkrun::HFCLKRUN_SPEC>,
    #[doc = "0x40c - Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub hfclkstat: crate::Reg<hfclkstat::HFCLKSTAT_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: crate::Reg<lfclkrun::LFCLKRUN_SPEC>,
    #[doc = "0x418 - Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub lfclkstat: crate::Reg<lfclkstat::LFCLKSTAT_SPEC>,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>,
    _reserved25: [u8; 0xf4],
    #[doc = "0x514 - Clock source for HFCLK128M/HFCLK64M"]
    pub hfclksrc: crate::Reg<hfclksrc::HFCLKSRC_SPEC>,
    #[doc = "0x518 - Clock source for LFCLK"]
    pub lfclksrc: crate::Reg<lfclksrc::LFCLKSRC_SPEC>,
    _reserved27: [u8; 0x3c],
    #[doc = "0x558 - HFCLK128M frequency configuration"]
    pub hfclkctrl: crate::Reg<hfclkctrl::HFCLKCTRL_SPEC>,
    _reserved28: [u8; 0x14],
    #[doc = "0x570 - Automatic or manual control of HFCLK128M/HFCLK64M"]
    pub hfclkalwaysrun: crate::Reg<hfclkalwaysrun::HFCLKALWAYSRUN_SPEC>,
    #[doc = "0x574 - Automatic or manual control of LFCLK"]
    pub lfclkalwaysrun: crate::Reg<lfclkalwaysrun::LFCLKALWAYSRUN_SPEC>,
}
#[doc = "TASKS_HFCLKSTART register accessor: an alias for `Reg<TASKS_HFCLKSTART_SPEC>`"]
pub type TASKS_HFCLKSTART = crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>;
#[doc = "Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP register accessor: an alias for `Reg<TASKS_HFCLKSTOP_SPEC>`"]
pub type TASKS_HFCLKSTOP = crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>;
#[doc = "Stop HFCLK128M/HFCLK64M source"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART register accessor: an alias for `Reg<TASKS_LFCLKSTART_SPEC>`"]
pub type TASKS_LFCLKSTART = crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>;
#[doc = "Start LFCLK source as selected in LFCLKSRC"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP register accessor: an alias for `Reg<TASKS_LFCLKSTOP_SPEC>`"]
pub type TASKS_LFCLKSTOP = crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL register accessor: an alias for `Reg<TASKS_CAL_SPEC>`"]
pub type TASKS_CAL = crate::Reg<tasks_cal::TASKS_CAL_SPEC>;
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "SUBSCRIBE_HFCLKSTART register accessor: an alias for `Reg<SUBSCRIBE_HFCLKSTART_SPEC>`"]
pub type SUBSCRIBE_HFCLKSTART = crate::Reg<subscribe_hfclkstart::SUBSCRIBE_HFCLKSTART_SPEC>;
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "SUBSCRIBE_HFCLKSTOP register accessor: an alias for `Reg<SUBSCRIBE_HFCLKSTOP_SPEC>`"]
pub type SUBSCRIBE_HFCLKSTOP = crate::Reg<subscribe_hfclkstop::SUBSCRIBE_HFCLKSTOP_SPEC>;
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "SUBSCRIBE_LFCLKSTART register accessor: an alias for `Reg<SUBSCRIBE_LFCLKSTART_SPEC>`"]
pub type SUBSCRIBE_LFCLKSTART = crate::Reg<subscribe_lfclkstart::SUBSCRIBE_LFCLKSTART_SPEC>;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "SUBSCRIBE_LFCLKSTOP register accessor: an alias for `Reg<SUBSCRIBE_LFCLKSTOP_SPEC>`"]
pub type SUBSCRIBE_LFCLKSTOP = crate::Reg<subscribe_lfclkstop::SUBSCRIBE_LFCLKSTOP_SPEC>;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "SUBSCRIBE_CAL register accessor: an alias for `Reg<SUBSCRIBE_CAL_SPEC>`"]
pub type SUBSCRIBE_CAL = crate::Reg<subscribe_cal::SUBSCRIBE_CAL_SPEC>;
#[doc = "Subscribe configuration for task CAL"]
pub mod subscribe_cal;
#[doc = "EVENTS_HFCLKSTARTED register accessor: an alias for `Reg<EVENTS_HFCLKSTARTED_SPEC>`"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>;
#[doc = "HFCLK128M/HFCLK64M source started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED register accessor: an alias for `Reg<EVENTS_LFCLKSTARTED_SPEC>`"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>;
#[doc = "LFCLK source started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "Calibration of LFRC oscillator complete event"]
pub mod events_done;
#[doc = "PUBLISH_HFCLKSTARTED register accessor: an alias for `Reg<PUBLISH_HFCLKSTARTED_SPEC>`"]
pub type PUBLISH_HFCLKSTARTED = crate::Reg<publish_hfclkstarted::PUBLISH_HFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "PUBLISH_LFCLKSTARTED register accessor: an alias for `Reg<PUBLISH_LFCLKSTARTED_SPEC>`"]
pub type PUBLISH_LFCLKSTARTED = crate::Reg<publish_lfclkstarted::PUBLISH_LFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
#[doc = "PUBLISH_DONE register accessor: an alias for `Reg<PUBLISH_DONE_SPEC>`"]
pub type PUBLISH_DONE = crate::Reg<publish_done::PUBLISH_DONE_SPEC>;
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "INTPEND register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "HFCLKRUN register accessor: an alias for `Reg<HFCLKRUN_SPEC>`"]
pub type HFCLKRUN = crate::Reg<hfclkrun::HFCLKRUN_SPEC>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT register accessor: an alias for `Reg<HFCLKSTAT_SPEC>`"]
pub type HFCLKSTAT = crate::Reg<hfclkstat::HFCLKSTAT_SPEC>;
#[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod hfclkstat;
#[doc = "LFCLKRUN register accessor: an alias for `Reg<LFCLKRUN_SPEC>`"]
pub type LFCLKRUN = crate::Reg<lfclkrun::LFCLKRUN_SPEC>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT register accessor: an alias for `Reg<LFCLKSTAT_SPEC>`"]
pub type LFCLKSTAT = crate::Reg<lfclkstat::LFCLKSTAT_SPEC>;
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY register accessor: an alias for `Reg<LFCLKSRCCOPY_SPEC>`"]
pub type LFCLKSRCCOPY = crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "HFCLKSRC register accessor: an alias for `Reg<HFCLKSRC_SPEC>`"]
pub type HFCLKSRC = crate::Reg<hfclksrc::HFCLKSRC_SPEC>;
#[doc = "Clock source for HFCLK128M/HFCLK64M"]
pub mod hfclksrc;
#[doc = "LFCLKSRC register accessor: an alias for `Reg<LFCLKSRC_SPEC>`"]
pub type LFCLKSRC = crate::Reg<lfclksrc::LFCLKSRC_SPEC>;
#[doc = "Clock source for LFCLK"]
pub mod lfclksrc;
#[doc = "HFCLKCTRL register accessor: an alias for `Reg<HFCLKCTRL_SPEC>`"]
pub type HFCLKCTRL = crate::Reg<hfclkctrl::HFCLKCTRL_SPEC>;
#[doc = "HFCLK128M frequency configuration"]
pub mod hfclkctrl;
#[doc = "HFCLKALWAYSRUN register accessor: an alias for `Reg<HFCLKALWAYSRUN_SPEC>`"]
pub type HFCLKALWAYSRUN = crate::Reg<hfclkalwaysrun::HFCLKALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
pub mod hfclkalwaysrun;
#[doc = "LFCLKALWAYSRUN register accessor: an alias for `Reg<LFCLKALWAYSRUN_SPEC>`"]
pub type LFCLKALWAYSRUN = crate::Reg<lfclkalwaysrun::LFCLKALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of LFCLK"]
pub mod lfclkalwaysrun;
