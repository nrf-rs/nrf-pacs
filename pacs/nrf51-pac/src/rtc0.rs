#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC Counter."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC Counter."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC Counter."]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set COUNTER to 0xFFFFFFF0."]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - Event on COUNTER increment."]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on COUNTER overflow."]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved6: [u8; 0x38],
    #[doc = "0x140..0x150 - Compare event on CC\\[n\\]
match."]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved7: [u8; 0x01b4],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0x34],
    #[doc = "0x340 - Configures event enable routing to PPI for each RTC event."]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenclr: EVTENCLR,
    _reserved12: [u8; 0x01b8],
    #[doc = "0x504 - Current COUNTER value."]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
    pub prescaler: PRESCALER,
    _reserved14: [u8; 0x34],
    #[doc = "0x540..0x550 - Capture/compare registers."]
    pub cc: [CC; 4],
    _reserved15: [u8; 0x0aac],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start RTC Counter."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop RTC Counter."]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR (w) register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear RTC Counter."]
pub mod tasks_clear;
#[doc = "TASKS_TRIGOVRFLW (w) register accessor: an alias for `Reg<TASKS_TRIGOVRFLW_SPEC>`"]
pub type TASKS_TRIGOVRFLW = crate::Reg<tasks_trigovrflw::TASKS_TRIGOVRFLW_SPEC>;
#[doc = "Set COUNTER to 0xFFFFFFF0."]
pub mod tasks_trigovrflw;
#[doc = "EVENTS_TICK (rw) register accessor: an alias for `Reg<EVENTS_TICK_SPEC>`"]
pub type EVENTS_TICK = crate::Reg<events_tick::EVENTS_TICK_SPEC>;
#[doc = "Event on COUNTER increment."]
pub mod events_tick;
#[doc = "EVENTS_OVRFLW (rw) register accessor: an alias for `Reg<EVENTS_OVRFLW_SPEC>`"]
pub type EVENTS_OVRFLW = crate::Reg<events_ovrflw::EVENTS_OVRFLW_SPEC>;
#[doc = "Event on COUNTER overflow."]
pub mod events_ovrflw;
#[doc = "EVENTS_COMPARE (rw) register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Compare event on CC\\[n\\]
match."]
pub mod events_compare;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "EVTEN (rw) register accessor: an alias for `Reg<EVTEN_SPEC>`"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Configures event enable routing to PPI for each RTC event."]
pub mod evten;
#[doc = "EVTENSET (rw) register accessor: an alias for `Reg<EVTENSET_SPEC>`"]
pub type EVTENSET = crate::Reg<evtenset::EVTENSET_SPEC>;
#[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenset;
#[doc = "EVTENCLR (rw) register accessor: an alias for `Reg<EVTENCLR_SPEC>`"]
pub type EVTENCLR = crate::Reg<evtenclr::EVTENCLR_SPEC>;
#[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenclr;
#[doc = "COUNTER (r) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Current COUNTER value."]
pub mod counter;
#[doc = "PRESCALER (rw) register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
pub mod prescaler;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Capture/compare registers."]
pub mod cc;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
