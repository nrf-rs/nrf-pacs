#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    #[doc = "0x78 - Enable Constant Latency mode"]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable Low-Power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 0x78],
    #[doc = "0xf8 - Subscribe configuration for task CONSTLAT"]
    pub subscribe_constlat: SUBSCRIBE_CONSTLAT,
    #[doc = "0xfc - Subscribe configuration for task LOWPWR"]
    pub subscribe_lowpwr: SUBSCRIBE_LOWPWR,
    _reserved4: [u8; 0x08],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved5: [u8; 0x08],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved7: [u8; 0x6c],
    #[doc = "0x188 - Publish configuration for event POFWARN"]
    pub publish_pofwarn: PUBLISH_POFWARN,
    _reserved8: [u8; 0x08],
    #[doc = "0x194 - Publish configuration for event SLEEPENTER"]
    pub publish_sleepenter: PUBLISH_SLEEPENTER,
    #[doc = "0x198 - Publish configuration for event SLEEPEXIT"]
    pub publish_sleepexit: PUBLISH_SLEEPEXIT,
    _reserved10: [u8; 0x0164],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 0x0210],
    #[doc = "0x51c..0x524 - Description collection: General purpose retention register"]
    pub gpregret: [GPREGRET; 2],
}
#[doc = "TASKS_CONSTLAT (w) register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable Constant Latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable Low-Power mode (variable latency)"]
pub mod tasks_lowpwr;
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
#[doc = "GPREGRET (rw) register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;
