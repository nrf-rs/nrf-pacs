#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK128M/HFCLK64M source"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source as selected in LFCLKSRC"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    pub tasks_cal: TASKS_CAL,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Start HFCLKAUDIO source"]
    pub tasks_hfclkaudiostart: TASKS_HFCLKAUDIOSTART,
    #[doc = "0x1c - Stop HFCLKAUDIO source"]
    pub tasks_hfclkaudiostop: TASKS_HFCLKAUDIOSTOP,
    #[doc = "0x20 - Start HFCLK192M source as selected in HFCLK192MSRC"]
    pub tasks_hfclk192mstart: TASKS_HFCLK192MSTART,
    #[doc = "0x24 - Stop HFCLK192M source"]
    pub tasks_hfclk192mstop: TASKS_HFCLK192MSTOP,
    _reserved9: [u8; 0x58],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: SUBSCRIBE_HFCLKSTART,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: SUBSCRIBE_HFCLKSTOP,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: SUBSCRIBE_LFCLKSTART,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: SUBSCRIBE_LFCLKSTOP,
    #[doc = "0x90 - Subscribe configuration for task CAL"]
    pub subscribe_cal: SUBSCRIBE_CAL,
    _reserved14: [u8; 0x04],
    #[doc = "0x98 - Subscribe configuration for task HFCLKAUDIOSTART"]
    pub subscribe_hfclkaudiostart: SUBSCRIBE_HFCLKAUDIOSTART,
    #[doc = "0x9c - Subscribe configuration for task HFCLKAUDIOSTOP"]
    pub subscribe_hfclkaudiostop: SUBSCRIBE_HFCLKAUDIOSTOP,
    #[doc = "0xa0 - Subscribe configuration for task HFCLK192MSTART"]
    pub subscribe_hfclk192mstart: SUBSCRIBE_HFCLK192MSTART,
    #[doc = "0xa4 - Subscribe configuration for task HFCLK192MSTOP"]
    pub subscribe_hfclk192mstop: SUBSCRIBE_HFCLK192MSTOP,
    _reserved18: [u8; 0x58],
    #[doc = "0x100 - HFCLK128M/HFCLK64M source started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK source started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved20: [u8; 0x14],
    #[doc = "0x11c - Calibration of LFRC oscillator complete event"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x120 - HFCLKAUDIO source started"]
    pub events_hfclkaudiostarted: EVENTS_HFCLKAUDIOSTARTED,
    #[doc = "0x124 - HFCLK192M source started"]
    pub events_hfclk192mstarted: EVENTS_HFCLK192MSTARTED,
    _reserved23: [u8; 0x58],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: PUBLISH_HFCLKSTARTED,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: PUBLISH_LFCLKSTARTED,
    _reserved25: [u8; 0x14],
    #[doc = "0x19c - Publish configuration for event DONE"]
    pub publish_done: PUBLISH_DONE,
    #[doc = "0x1a0 - Publish configuration for event HFCLKAUDIOSTARTED"]
    pub publish_hfclkaudiostarted: PUBLISH_HFCLKAUDIOSTARTED,
    #[doc = "0x1a4 - Publish configuration for event HFCLK192MSTARTED"]
    pub publish_hfclk192mstarted: PUBLISH_HFCLK192MSTARTED,
    _reserved28: [u8; 0x0158],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved32: [u8; 0xf8],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub hfclkstat: HFCLKSTAT,
    _reserved34: [u8; 0x04],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved37: [u8; 0x30],
    #[doc = "0x450 - Status indicating that HFCLKAUDIOSTART task has been triggered"]
    pub hfclkaudiorun: HFCLKAUDIORUN,
    #[doc = "0x454 - Status indicating which HFCLKAUDIO source is running"]
    pub hfclkaudiostat: HFCLKAUDIOSTAT,
    #[doc = "0x458 - Status indicating that HFCLK192MSTART task has been triggered"]
    pub hfclk192mrun: HFCLK192MRUN,
    #[doc = "0x45c - Status indicating which HFCLK192M source is running"]
    pub hfclk192mstat: HFCLK192MSTAT,
    _reserved41: [u8; 0xb4],
    #[doc = "0x514 - Clock source for HFCLK128M/HFCLK64M"]
    pub hfclksrc: HFCLKSRC,
    #[doc = "0x518 - Clock source for LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved43: [u8; 0x3c],
    #[doc = "0x558 - HFCLK128M frequency configuration"]
    pub hfclkctrl: HFCLKCTRL,
    #[doc = "0x55c - Unspecified"]
    pub hfclkaudio: HFCLKAUDIO,
    _reserved45: [u8; 0x10],
    #[doc = "0x570 - Automatic or manual control of HFCLK128M/HFCLK64M"]
    pub hfclkalwaysrun: HFCLKALWAYSRUN,
    #[doc = "0x574 - Automatic or manual control of LFCLK"]
    pub lfclkalwaysrun: LFCLKALWAYSRUN,
    _reserved47: [u8; 0x04],
    #[doc = "0x57c - Automatic or manual control of HFCLKAUDIO"]
    pub hfclkaudioalwaysrun: HFCLKAUDIOALWAYSRUN,
    #[doc = "0x580 - Clock source for HFCLK192M"]
    pub hfclk192msrc: HFCLK192MSRC,
    #[doc = "0x584 - Automatic or manual control of HFCLK192M"]
    pub hfclk192malwaysrun: HFCLK192MALWAYSRUN,
    _reserved50: [u8; 0x30],
    #[doc = "0x5b8 - HFCLK192M frequency configuration"]
    pub hfclk192mctrl: HFCLK192MCTRL,
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: an alias for `Reg<TASKS_HFCLKSTART_SPEC>`"]
pub type TASKS_HFCLKSTART = crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>;
#[doc = "Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: an alias for `Reg<TASKS_HFCLKSTOP_SPEC>`"]
pub type TASKS_HFCLKSTOP = crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>;
#[doc = "Stop HFCLK128M/HFCLK64M source"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: an alias for `Reg<TASKS_LFCLKSTART_SPEC>`"]
pub type TASKS_LFCLKSTART = crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>;
#[doc = "Start LFCLK source as selected in LFCLKSRC"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: an alias for `Reg<TASKS_LFCLKSTOP_SPEC>`"]
pub type TASKS_LFCLKSTOP = crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: an alias for `Reg<TASKS_CAL_SPEC>`"]
pub type TASKS_CAL = crate::Reg<tasks_cal::TASKS_CAL_SPEC>;
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "TASKS_HFCLKAUDIOSTART (w) register accessor: an alias for `Reg<TASKS_HFCLKAUDIOSTART_SPEC>`"]
pub type TASKS_HFCLKAUDIOSTART = crate::Reg<tasks_hfclkaudiostart::TASKS_HFCLKAUDIOSTART_SPEC>;
#[doc = "Start HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostart;
#[doc = "TASKS_HFCLKAUDIOSTOP (w) register accessor: an alias for `Reg<TASKS_HFCLKAUDIOSTOP_SPEC>`"]
pub type TASKS_HFCLKAUDIOSTOP = crate::Reg<tasks_hfclkaudiostop::TASKS_HFCLKAUDIOSTOP_SPEC>;
#[doc = "Stop HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostop;
#[doc = "TASKS_HFCLK192MSTART (w) register accessor: an alias for `Reg<TASKS_HFCLK192MSTART_SPEC>`"]
pub type TASKS_HFCLK192MSTART = crate::Reg<tasks_hfclk192mstart::TASKS_HFCLK192MSTART_SPEC>;
#[doc = "Start HFCLK192M source as selected in HFCLK192MSRC"]
pub mod tasks_hfclk192mstart;
#[doc = "TASKS_HFCLK192MSTOP (w) register accessor: an alias for `Reg<TASKS_HFCLK192MSTOP_SPEC>`"]
pub type TASKS_HFCLK192MSTOP = crate::Reg<tasks_hfclk192mstop::TASKS_HFCLK192MSTOP_SPEC>;
#[doc = "Stop HFCLK192M source"]
pub mod tasks_hfclk192mstop;
#[doc = "SUBSCRIBE_HFCLKSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLKSTART_SPEC>`"]
pub type SUBSCRIBE_HFCLKSTART = crate::Reg<subscribe_hfclkstart::SUBSCRIBE_HFCLKSTART_SPEC>;
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "SUBSCRIBE_HFCLKSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLKSTOP_SPEC>`"]
pub type SUBSCRIBE_HFCLKSTOP = crate::Reg<subscribe_hfclkstop::SUBSCRIBE_HFCLKSTOP_SPEC>;
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "SUBSCRIBE_LFCLKSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_LFCLKSTART_SPEC>`"]
pub type SUBSCRIBE_LFCLKSTART = crate::Reg<subscribe_lfclkstart::SUBSCRIBE_LFCLKSTART_SPEC>;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "SUBSCRIBE_LFCLKSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_LFCLKSTOP_SPEC>`"]
pub type SUBSCRIBE_LFCLKSTOP = crate::Reg<subscribe_lfclkstop::SUBSCRIBE_LFCLKSTOP_SPEC>;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "SUBSCRIBE_CAL (rw) register accessor: an alias for `Reg<SUBSCRIBE_CAL_SPEC>`"]
pub type SUBSCRIBE_CAL = crate::Reg<subscribe_cal::SUBSCRIBE_CAL_SPEC>;
#[doc = "Subscribe configuration for task CAL"]
pub mod subscribe_cal;
#[doc = "SUBSCRIBE_HFCLKAUDIOSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLKAUDIOSTART_SPEC>`"]
pub type SUBSCRIBE_HFCLKAUDIOSTART =
    crate::Reg<subscribe_hfclkaudiostart::SUBSCRIBE_HFCLKAUDIOSTART_SPEC>;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTART"]
pub mod subscribe_hfclkaudiostart;
#[doc = "SUBSCRIBE_HFCLKAUDIOSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLKAUDIOSTOP_SPEC>`"]
pub type SUBSCRIBE_HFCLKAUDIOSTOP =
    crate::Reg<subscribe_hfclkaudiostop::SUBSCRIBE_HFCLKAUDIOSTOP_SPEC>;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTOP"]
pub mod subscribe_hfclkaudiostop;
#[doc = "SUBSCRIBE_HFCLK192MSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLK192MSTART_SPEC>`"]
pub type SUBSCRIBE_HFCLK192MSTART =
    crate::Reg<subscribe_hfclk192mstart::SUBSCRIBE_HFCLK192MSTART_SPEC>;
#[doc = "Subscribe configuration for task HFCLK192MSTART"]
pub mod subscribe_hfclk192mstart;
#[doc = "SUBSCRIBE_HFCLK192MSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_HFCLK192MSTOP_SPEC>`"]
pub type SUBSCRIBE_HFCLK192MSTOP =
    crate::Reg<subscribe_hfclk192mstop::SUBSCRIBE_HFCLK192MSTOP_SPEC>;
#[doc = "Subscribe configuration for task HFCLK192MSTOP"]
pub mod subscribe_hfclk192mstop;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: an alias for `Reg<EVENTS_HFCLKSTARTED_SPEC>`"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>;
#[doc = "HFCLK128M/HFCLK64M source started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: an alias for `Reg<EVENTS_LFCLKSTARTED_SPEC>`"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>;
#[doc = "LFCLK source started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "Calibration of LFRC oscillator complete event"]
pub mod events_done;
#[doc = "EVENTS_HFCLKAUDIOSTARTED (rw) register accessor: an alias for `Reg<EVENTS_HFCLKAUDIOSTARTED_SPEC>`"]
pub type EVENTS_HFCLKAUDIOSTARTED =
    crate::Reg<events_hfclkaudiostarted::EVENTS_HFCLKAUDIOSTARTED_SPEC>;
#[doc = "HFCLKAUDIO source started"]
pub mod events_hfclkaudiostarted;
#[doc = "EVENTS_HFCLK192MSTARTED (rw) register accessor: an alias for `Reg<EVENTS_HFCLK192MSTARTED_SPEC>`"]
pub type EVENTS_HFCLK192MSTARTED =
    crate::Reg<events_hfclk192mstarted::EVENTS_HFCLK192MSTARTED_SPEC>;
#[doc = "HFCLK192M source started"]
pub mod events_hfclk192mstarted;
#[doc = "PUBLISH_HFCLKSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_HFCLKSTARTED_SPEC>`"]
pub type PUBLISH_HFCLKSTARTED = crate::Reg<publish_hfclkstarted::PUBLISH_HFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "PUBLISH_LFCLKSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_LFCLKSTARTED_SPEC>`"]
pub type PUBLISH_LFCLKSTARTED = crate::Reg<publish_lfclkstarted::PUBLISH_LFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
#[doc = "PUBLISH_DONE (rw) register accessor: an alias for `Reg<PUBLISH_DONE_SPEC>`"]
pub type PUBLISH_DONE = crate::Reg<publish_done::PUBLISH_DONE_SPEC>;
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "PUBLISH_HFCLKAUDIOSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_HFCLKAUDIOSTARTED_SPEC>`"]
pub type PUBLISH_HFCLKAUDIOSTARTED =
    crate::Reg<publish_hfclkaudiostarted::PUBLISH_HFCLKAUDIOSTARTED_SPEC>;
#[doc = "Publish configuration for event HFCLKAUDIOSTARTED"]
pub mod publish_hfclkaudiostarted;
#[doc = "PUBLISH_HFCLK192MSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_HFCLK192MSTARTED_SPEC>`"]
pub type PUBLISH_HFCLK192MSTARTED =
    crate::Reg<publish_hfclk192mstarted::PUBLISH_HFCLK192MSTARTED_SPEC>;
#[doc = "Publish configuration for event HFCLK192MSTARTED"]
pub mod publish_hfclk192mstarted;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "INTPEND (r) register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "HFCLKRUN (r) register accessor: an alias for `Reg<HFCLKRUN_SPEC>`"]
pub type HFCLKRUN = crate::Reg<hfclkrun::HFCLKRUN_SPEC>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: an alias for `Reg<HFCLKSTAT_SPEC>`"]
pub type HFCLKSTAT = crate::Reg<hfclkstat::HFCLKSTAT_SPEC>;
#[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: an alias for `Reg<LFCLKRUN_SPEC>`"]
pub type LFCLKRUN = crate::Reg<lfclkrun::LFCLKRUN_SPEC>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: an alias for `Reg<LFCLKSTAT_SPEC>`"]
pub type LFCLKSTAT = crate::Reg<lfclkstat::LFCLKSTAT_SPEC>;
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: an alias for `Reg<LFCLKSRCCOPY_SPEC>`"]
pub type LFCLKSRCCOPY = crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "HFCLKAUDIORUN (r) register accessor: an alias for `Reg<HFCLKAUDIORUN_SPEC>`"]
pub type HFCLKAUDIORUN = crate::Reg<hfclkaudiorun::HFCLKAUDIORUN_SPEC>;
#[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered"]
pub mod hfclkaudiorun;
#[doc = "HFCLKAUDIOSTAT (r) register accessor: an alias for `Reg<HFCLKAUDIOSTAT_SPEC>`"]
pub type HFCLKAUDIOSTAT = crate::Reg<hfclkaudiostat::HFCLKAUDIOSTAT_SPEC>;
#[doc = "Status indicating which HFCLKAUDIO source is running"]
pub mod hfclkaudiostat;
#[doc = "HFCLK192MRUN (r) register accessor: an alias for `Reg<HFCLK192MRUN_SPEC>`"]
pub type HFCLK192MRUN = crate::Reg<hfclk192mrun::HFCLK192MRUN_SPEC>;
#[doc = "Status indicating that HFCLK192MSTART task has been triggered"]
pub mod hfclk192mrun;
#[doc = "HFCLK192MSTAT (r) register accessor: an alias for `Reg<HFCLK192MSTAT_SPEC>`"]
pub type HFCLK192MSTAT = crate::Reg<hfclk192mstat::HFCLK192MSTAT_SPEC>;
#[doc = "Status indicating which HFCLK192M source is running"]
pub mod hfclk192mstat;
#[doc = "HFCLKSRC (rw) register accessor: an alias for `Reg<HFCLKSRC_SPEC>`"]
pub type HFCLKSRC = crate::Reg<hfclksrc::HFCLKSRC_SPEC>;
#[doc = "Clock source for HFCLK128M/HFCLK64M"]
pub mod hfclksrc;
#[doc = "LFCLKSRC (rw) register accessor: an alias for `Reg<LFCLKSRC_SPEC>`"]
pub type LFCLKSRC = crate::Reg<lfclksrc::LFCLKSRC_SPEC>;
#[doc = "Clock source for LFCLK"]
pub mod lfclksrc;
#[doc = "HFCLKCTRL (rw) register accessor: an alias for `Reg<HFCLKCTRL_SPEC>`"]
pub type HFCLKCTRL = crate::Reg<hfclkctrl::HFCLKCTRL_SPEC>;
#[doc = "HFCLK128M frequency configuration"]
pub mod hfclkctrl;
#[doc = "Unspecified"]
pub use hfclkaudio::HFCLKAUDIO;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod hfclkaudio;
#[doc = "HFCLKALWAYSRUN (rw) register accessor: an alias for `Reg<HFCLKALWAYSRUN_SPEC>`"]
pub type HFCLKALWAYSRUN = crate::Reg<hfclkalwaysrun::HFCLKALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
pub mod hfclkalwaysrun;
#[doc = "LFCLKALWAYSRUN (rw) register accessor: an alias for `Reg<LFCLKALWAYSRUN_SPEC>`"]
pub type LFCLKALWAYSRUN = crate::Reg<lfclkalwaysrun::LFCLKALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of LFCLK"]
pub mod lfclkalwaysrun;
#[doc = "HFCLKAUDIOALWAYSRUN (rw) register accessor: an alias for `Reg<HFCLKAUDIOALWAYSRUN_SPEC>`"]
pub type HFCLKAUDIOALWAYSRUN = crate::Reg<hfclkaudioalwaysrun::HFCLKAUDIOALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of HFCLKAUDIO"]
pub mod hfclkaudioalwaysrun;
#[doc = "HFCLK192MSRC (rw) register accessor: an alias for `Reg<HFCLK192MSRC_SPEC>`"]
pub type HFCLK192MSRC = crate::Reg<hfclk192msrc::HFCLK192MSRC_SPEC>;
#[doc = "Clock source for HFCLK192M"]
pub mod hfclk192msrc;
#[doc = "HFCLK192MALWAYSRUN (rw) register accessor: an alias for `Reg<HFCLK192MALWAYSRUN_SPEC>`"]
pub type HFCLK192MALWAYSRUN = crate::Reg<hfclk192malwaysrun::HFCLK192MALWAYSRUN_SPEC>;
#[doc = "Automatic or manual control of HFCLK192M"]
pub mod hfclk192malwaysrun;
#[doc = "HFCLK192MCTRL (rw) register accessor: an alias for `Reg<HFCLK192MCTRL_SPEC>`"]
pub type HFCLK192MCTRL = crate::Reg<hfclk192mctrl::HFCLK192MCTRL_SPEC>;
#[doc = "HFCLK192M frequency configuration"]
pub mod hfclk192mctrl;
