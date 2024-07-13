#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x70],
    #[doc = "0x70 - Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
    pub tasks_pwmreqstart: TASKS_PWMREQSTART,
    #[doc = "0x74 - Stop requesting forcing PWM mode in external DC/DC voltage regulator"]
    pub tasks_pwmreqstop: TASKS_PWMREQSTOP,
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved4: [u8; 0x70],
    #[doc = "0xf0 - Subscribe configuration for task PWMREQSTART"]
    pub subscribe_pwmreqstart: SUBSCRIBE_PWMREQSTART,
    #[doc = "0xf4 - Subscribe configuration for task PWMREQSTOP"]
    pub subscribe_pwmreqstop: SUBSCRIBE_PWMREQSTOP,
    #[doc = "0xf8 - Subscribe configuration for task CONSTLAT"]
    pub subscribe_constlat: SUBSCRIBE_CONSTLAT,
    #[doc = "0xfc - Subscribe configuration for task LOWPWR"]
    pub subscribe_lowpwr: SUBSCRIBE_LOWPWR,
    _reserved8: [u8; 0x08],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved9: [u8; 0x08],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved11: [u8; 0x6c],
    #[doc = "0x188 - Publish configuration for event POFWARN"]
    pub publish_pofwarn: PUBLISH_POFWARN,
    _reserved12: [u8; 0x08],
    #[doc = "0x194 - Publish configuration for event SLEEPENTER"]
    pub publish_sleepenter: PUBLISH_SLEEPENTER,
    #[doc = "0x198 - Publish configuration for event SLEEPEXIT"]
    pub publish_sleepexit: PUBLISH_SLEEPEXIT,
    _reserved14: [u8; 0x0164],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved17: [u8; 0xf4],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved18: [u8; 0x3c],
    #[doc = "0x440 - Modem domain power status"]
    pub powerstatus: POWERSTATUS,
    _reserved19: [u8; 0xd8],
    #[doc = "0x51c..0x524 - Description collection: General purpose retention register"]
    pub gpregret: [GPREGRET; 2],
    _reserved20: [u8; 0xec],
    #[doc = "0x610..0x618 - LTE Modem"]
    pub ltemodem: LTEMODEM,
}
#[doc = "TASKS_PWMREQSTART (w) register accessor: an alias for `Reg<TASKS_PWMREQSTART_SPEC>`"]
pub type TASKS_PWMREQSTART = crate::Reg<tasks_pwmreqstart::TASKS_PWMREQSTART_SPEC>;
#[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
pub mod tasks_pwmreqstart;
#[doc = "TASKS_PWMREQSTOP (w) register accessor: an alias for `Reg<TASKS_PWMREQSTOP_SPEC>`"]
pub type TASKS_PWMREQSTOP = crate::Reg<tasks_pwmreqstop::TASKS_PWMREQSTOP_SPEC>;
#[doc = "Stop requesting forcing PWM mode in external DC/DC voltage regulator"]
pub mod tasks_pwmreqstop;
#[doc = "TASKS_CONSTLAT (w) register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "SUBSCRIBE_PWMREQSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_PWMREQSTART_SPEC>`"]
pub type SUBSCRIBE_PWMREQSTART = crate::Reg<subscribe_pwmreqstart::SUBSCRIBE_PWMREQSTART_SPEC>;
#[doc = "Subscribe configuration for task PWMREQSTART"]
pub mod subscribe_pwmreqstart;
#[doc = "SUBSCRIBE_PWMREQSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_PWMREQSTOP_SPEC>`"]
pub type SUBSCRIBE_PWMREQSTOP = crate::Reg<subscribe_pwmreqstop::SUBSCRIBE_PWMREQSTOP_SPEC>;
#[doc = "Subscribe configuration for task PWMREQSTOP"]
pub mod subscribe_pwmreqstop;
#[doc = "SUBSCRIBE_CONSTLAT (rw) register accessor: an alias for `Reg<SUBSCRIBE_CONSTLAT_SPEC>`"]
pub type SUBSCRIBE_CONSTLAT = crate::Reg<subscribe_constlat::SUBSCRIBE_CONSTLAT_SPEC>;
#[doc = "Subscribe configuration for task CONSTLAT"]
pub mod subscribe_constlat;
#[doc = "SUBSCRIBE_LOWPWR (rw) register accessor: an alias for `Reg<SUBSCRIBE_LOWPWR_SPEC>`"]
pub type SUBSCRIBE_LOWPWR = crate::Reg<subscribe_lowpwr::SUBSCRIBE_LOWPWR_SPEC>;
#[doc = "Subscribe configuration for task LOWPWR"]
pub mod subscribe_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: an alias for `Reg<EVENTS_POFWARN_SPEC>`"]
pub type EVENTS_POFWARN = crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER (rw) register accessor: an alias for `Reg<EVENTS_SLEEPENTER_SPEC>`"]
pub type EVENTS_SLEEPENTER = crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT (rw) register accessor: an alias for `Reg<EVENTS_SLEEPEXIT_SPEC>`"]
pub type EVENTS_SLEEPEXIT = crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "PUBLISH_POFWARN (rw) register accessor: an alias for `Reg<PUBLISH_POFWARN_SPEC>`"]
pub type PUBLISH_POFWARN = crate::Reg<publish_pofwarn::PUBLISH_POFWARN_SPEC>;
#[doc = "Publish configuration for event POFWARN"]
pub mod publish_pofwarn;
#[doc = "PUBLISH_SLEEPENTER (rw) register accessor: an alias for `Reg<PUBLISH_SLEEPENTER_SPEC>`"]
pub type PUBLISH_SLEEPENTER = crate::Reg<publish_sleepenter::PUBLISH_SLEEPENTER_SPEC>;
#[doc = "Publish configuration for event SLEEPENTER"]
pub mod publish_sleepenter;
#[doc = "PUBLISH_SLEEPEXIT (rw) register accessor: an alias for `Reg<PUBLISH_SLEEPEXIT_SPEC>`"]
pub type PUBLISH_SLEEPEXIT = crate::Reg<publish_sleepexit::PUBLISH_SLEEPEXIT_SPEC>;
#[doc = "Publish configuration for event SLEEPEXIT"]
pub mod publish_sleepexit;
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
#[doc = "RESETREAS (rw) register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "POWERSTATUS (r) register accessor: an alias for `Reg<POWERSTATUS_SPEC>`"]
pub type POWERSTATUS = crate::Reg<powerstatus::POWERSTATUS_SPEC>;
#[doc = "Modem domain power status"]
pub mod powerstatus;
#[doc = "GPREGRET (rw) register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;
#[doc = "LTE Modem"]
pub use ltemodem::LTEMODEM;
#[doc = r"Cluster"]
#[doc = "LTE Modem"]
pub mod ltemodem;
