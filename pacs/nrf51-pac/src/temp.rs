#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement."]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop temperature measurement."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Temperature measurement complete, data ready event."]
    pub events_datardy: crate::Reg<events_datardy::EVENTS_DATARDY_SPEC>,
    _reserved3: [u8; 0x0200],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x01fc],
    #[doc = "0x508 - Die temperature in degC, 2's complement format, 0.25 degC pecision."]
    pub temp: crate::Reg<temp::TEMP_SPEC>,
    _reserved6: [u8; 0x0af0],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start temperature measurement."]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop temperature measurement."]
pub mod tasks_stop;
#[doc = "EVENTS_DATARDY register accessor: an alias for `Reg<EVENTS_DATARDY_SPEC>`"]
pub type EVENTS_DATARDY = crate::Reg<events_datardy::EVENTS_DATARDY_SPEC>;
#[doc = "Temperature measurement complete, data ready event."]
pub mod events_datardy;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "TEMP register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision."]
pub mod temp;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
