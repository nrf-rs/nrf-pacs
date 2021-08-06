#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    #[doc = "0x78 - Enable Constant Latency mode"]
    pub tasks_constlat: crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>,
    #[doc = "0x7c - Enable Low-Power mode (variable latency)"]
    pub tasks_lowpwr: crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0xf8 - Subscribe configuration for task CONSTLAT"]
    pub subscribe_constlat: crate::Reg<subscribe_constlat::SUBSCRIBE_CONSTLAT_SPEC>,
    #[doc = "0xfc - Subscribe configuration for task LOWPWR"]
    pub subscribe_lowpwr: crate::Reg<subscribe_lowpwr::SUBSCRIBE_LOWPWR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>,
    _reserved7: [u8; 0x6c],
    #[doc = "0x188 - Publish configuration for event POFWARN"]
    pub publish_pofwarn: crate::Reg<publish_pofwarn::PUBLISH_POFWARN_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x194 - Publish configuration for event SLEEPENTER"]
    pub publish_sleepenter: crate::Reg<publish_sleepenter::PUBLISH_SLEEPENTER_SPEC>,
    #[doc = "0x198 - Publish configuration for event SLEEPEXIT"]
    pub publish_sleepexit: crate::Reg<publish_sleepexit::PUBLISH_SLEEPEXIT_SPEC>,
    _reserved10: [u8; 0x0164],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved13: [u8; 0x0210],
    #[doc = "0x51c..0x524 - Description collection: General purpose retention register"]
    pub gpregret: [crate::Reg<gpregret::GPREGRET_SPEC>; 2],
}
#[doc = "TASKS_CONSTLAT register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable Constant Latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable Low-Power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "SUBSCRIBE_CONSTLAT register accessor: an alias for `Reg<SUBSCRIBE_CONSTLAT_SPEC>`"]
pub type SUBSCRIBE_CONSTLAT = crate::Reg<subscribe_constlat::SUBSCRIBE_CONSTLAT_SPEC>;
#[doc = "Subscribe configuration for task CONSTLAT"]
pub mod subscribe_constlat;
#[doc = "SUBSCRIBE_LOWPWR register accessor: an alias for `Reg<SUBSCRIBE_LOWPWR_SPEC>`"]
pub type SUBSCRIBE_LOWPWR = crate::Reg<subscribe_lowpwr::SUBSCRIBE_LOWPWR_SPEC>;
#[doc = "Subscribe configuration for task LOWPWR"]
pub mod subscribe_lowpwr;
#[doc = "EVENTS_POFWARN register accessor: an alias for `Reg<EVENTS_POFWARN_SPEC>`"]
pub type EVENTS_POFWARN = crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER register accessor: an alias for `Reg<EVENTS_SLEEPENTER_SPEC>`"]
pub type EVENTS_SLEEPENTER = crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT register accessor: an alias for `Reg<EVENTS_SLEEPEXIT_SPEC>`"]
pub type EVENTS_SLEEPEXIT = crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "PUBLISH_POFWARN register accessor: an alias for `Reg<PUBLISH_POFWARN_SPEC>`"]
pub type PUBLISH_POFWARN = crate::Reg<publish_pofwarn::PUBLISH_POFWARN_SPEC>;
#[doc = "Publish configuration for event POFWARN"]
pub mod publish_pofwarn;
#[doc = "PUBLISH_SLEEPENTER register accessor: an alias for `Reg<PUBLISH_SLEEPENTER_SPEC>`"]
pub type PUBLISH_SLEEPENTER = crate::Reg<publish_sleepenter::PUBLISH_SLEEPENTER_SPEC>;
#[doc = "Publish configuration for event SLEEPENTER"]
pub mod publish_sleepenter;
#[doc = "PUBLISH_SLEEPEXIT register accessor: an alias for `Reg<PUBLISH_SLEEPEXIT_SPEC>`"]
pub type PUBLISH_SLEEPEXIT = crate::Reg<publish_sleepexit::PUBLISH_SLEEPEXIT_SPEC>;
#[doc = "Publish configuration for event SLEEPEXIT"]
pub mod publish_sleepexit;
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
#[doc = "GPREGRET register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;
