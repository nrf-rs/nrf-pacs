#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK source"]
    pub tasks_hfclkstart: crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>,
    #[doc = "0x04 - Stop HFCLK source"]
    pub tasks_hfclkstop: crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>,
    _reserved4: [u8; 0x70],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: crate::Reg<subscribe_hfclkstart::SUBSCRIBE_HFCLKSTART_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: crate::Reg<subscribe_hfclkstop::SUBSCRIBE_HFCLKSTOP_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: crate::Reg<subscribe_lfclkstart::SUBSCRIBE_LFCLKSTART_SPEC>,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: crate::Reg<subscribe_lfclkstop::SUBSCRIBE_LFCLKSTOP_SPEC>,
    _reserved8: [u8; 0x70],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>,
    _reserved10: [u8; 0x78],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: crate::Reg<publish_hfclkstarted::PUBLISH_HFCLKSTARTED_SPEC>,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: crate::Reg<publish_lfclkstarted::PUBLISH_LFCLKSTARTED_SPEC>,
    _reserved12: [u8; 0x0178],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: crate::Reg<intpend::INTPEND_SPEC>,
    _reserved16: [u8; 0xf8],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: crate::Reg<hfclkrun::HFCLKRUN_SPEC>,
    #[doc = "0x40c - The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
    pub hfclkstat: crate::Reg<hfclkstat::HFCLKSTAT_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: crate::Reg<lfclkrun::LFCLKRUN_SPEC>,
    #[doc = "0x418 - The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
    pub lfclkstat: crate::Reg<lfclkstat::LFCLKSTAT_SPEC>,
    #[doc = "0x41c - Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
    pub lfclksrccopy: crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>,
    _reserved21: [u8; 0xf8],
    #[doc = "0x518 - Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
    pub lfclksrc: crate::Reg<lfclksrc::LFCLKSRC_SPEC>,
}
#[doc = "TASKS_HFCLKSTART register accessor: an alias for `Reg<TASKS_HFCLKSTART_SPEC>`"]
pub type TASKS_HFCLKSTART = crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>;
#[doc = "Start HFCLK source"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP register accessor: an alias for `Reg<TASKS_HFCLKSTOP_SPEC>`"]
pub type TASKS_HFCLKSTOP = crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>;
#[doc = "Stop HFCLK source"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART register accessor: an alias for `Reg<TASKS_LFCLKSTART_SPEC>`"]
pub type TASKS_LFCLKSTART = crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>;
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP register accessor: an alias for `Reg<TASKS_LFCLKSTOP_SPEC>`"]
pub type TASKS_LFCLKSTOP = crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
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
#[doc = "EVENTS_HFCLKSTARTED register accessor: an alias for `Reg<EVENTS_HFCLKSTARTED_SPEC>`"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>;
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED register accessor: an alias for `Reg<EVENTS_LFCLKSTARTED_SPEC>`"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>;
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "PUBLISH_HFCLKSTARTED register accessor: an alias for `Reg<PUBLISH_HFCLKSTARTED_SPEC>`"]
pub type PUBLISH_HFCLKSTARTED = crate::Reg<publish_hfclkstarted::PUBLISH_HFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "PUBLISH_LFCLKSTARTED register accessor: an alias for `Reg<PUBLISH_LFCLKSTARTED_SPEC>`"]
pub type PUBLISH_LFCLKSTARTED = crate::Reg<publish_lfclkstarted::PUBLISH_LFCLKSTARTED_SPEC>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
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
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
pub mod hfclkstat;
#[doc = "LFCLKRUN register accessor: an alias for `Reg<LFCLKRUN_SPEC>`"]
pub type LFCLKRUN = crate::Reg<lfclkrun::LFCLKRUN_SPEC>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT register accessor: an alias for `Reg<LFCLKSTAT_SPEC>`"]
pub type LFCLKSTAT = crate::Reg<lfclkstat::LFCLKSTAT_SPEC>;
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY register accessor: an alias for `Reg<LFCLKSRCCOPY_SPEC>`"]
pub type LFCLKSRCCOPY = crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>;
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC register accessor: an alias for `Reg<LFCLKSRC_SPEC>`"]
pub type LFCLKSRC = crate::Reg<lfclksrc::LFCLKSRC_SPEC>;
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
pub mod lfclksrc;
