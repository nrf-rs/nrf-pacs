#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC COUNTER"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop RTC COUNTER"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Clear RTC COUNTER"]
    pub tasks_clear: crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>,
    #[doc = "0x0c - Set COUNTER to 0xFFFFF0"]
    pub tasks_trigovrflw: crate::Reg<tasks_trigovrflw::TASKS_TRIGOVRFLW_SPEC>,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - Event on COUNTER increment"]
    pub events_tick: crate::Reg<events_tick::EVENTS_TICK_SPEC>,
    #[doc = "0x104 - Event on COUNTER overflow"]
    pub events_ovrflw: crate::Reg<events_ovrflw::EVENTS_OVRFLW_SPEC>,
    _reserved6: [u8; 0x38],
    #[doc = "0x140..0x150 - Description collection\\[0\\]: Compare event on CC\\[0\\]
match"]
    pub events_compare: [crate::Reg<events_compare::EVENTS_COMPARE_SPEC>; 4],
    _reserved7: [u8; 0x01b4],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0x34],
    #[doc = "0x340 - Enable or disable event routing"]
    pub evten: crate::Reg<evten::EVTEN_SPEC>,
    #[doc = "0x344 - Enable event routing"]
    pub evtenset: crate::Reg<evtenset::EVTENSET_SPEC>,
    #[doc = "0x348 - Disable event routing"]
    pub evtenclr: crate::Reg<evtenclr::EVTENCLR_SPEC>,
    _reserved12: [u8; 0x01b8],
    #[doc = "0x504 - Current COUNTER value"]
    pub counter: crate::Reg<counter::COUNTER_SPEC>,
    #[doc = "0x508 - 12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
    pub prescaler: crate::Reg<prescaler::PRESCALER_SPEC>,
    _reserved14: [u8; 0x34],
    #[doc = "0x540..0x550 - Description collection\\[0\\]: Compare register 0"]
    pub cc: [crate::Reg<cc::CC_SPEC>; 4],
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start RTC COUNTER"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop RTC COUNTER"]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear RTC COUNTER"]
pub mod tasks_clear;
#[doc = "TASKS_TRIGOVRFLW register accessor: an alias for `Reg<TASKS_TRIGOVRFLW_SPEC>`"]
pub type TASKS_TRIGOVRFLW = crate::Reg<tasks_trigovrflw::TASKS_TRIGOVRFLW_SPEC>;
#[doc = "Set COUNTER to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "EVENTS_TICK register accessor: an alias for `Reg<EVENTS_TICK_SPEC>`"]
pub type EVENTS_TICK = crate::Reg<events_tick::EVENTS_TICK_SPEC>;
#[doc = "Event on COUNTER increment"]
pub mod events_tick;
#[doc = "EVENTS_OVRFLW register accessor: an alias for `Reg<EVENTS_OVRFLW_SPEC>`"]
pub type EVENTS_OVRFLW = crate::Reg<events_ovrflw::EVENTS_OVRFLW_SPEC>;
#[doc = "Event on COUNTER overflow"]
pub mod events_ovrflw;
#[doc = "EVENTS_COMPARE register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\]
match"]
pub mod events_compare;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "EVTEN register accessor: an alias for `Reg<EVTEN_SPEC>`"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "EVTENSET register accessor: an alias for `Reg<EVTENSET_SPEC>`"]
pub type EVTENSET = crate::Reg<evtenset::EVTENSET_SPEC>;
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "EVTENCLR register accessor: an alias for `Reg<EVTENCLR_SPEC>`"]
pub type EVTENCLR = crate::Reg<evtenclr::EVTENCLR_SPEC>;
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "COUNTER register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Current COUNTER value"]
pub mod counter;
#[doc = "PRESCALER register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
pub mod prescaler;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Description collection\\[0\\]: Compare register 0"]
pub mod cc;
