#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
    pub tasks_startecb: crate::Reg<tasks_startecb::TASKS_STARTECB_SPEC>,
    #[doc = "0x04 - Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
    pub tasks_stopecb: crate::Reg<tasks_stopecb::TASKS_STOPECB_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - ECB block encrypt complete."]
    pub events_endecb: crate::Reg<events_endecb::EVENTS_ENDECB_SPEC>,
    #[doc = "0x104 - ECB block encrypt aborted due to a STOPECB task or due to an error."]
    pub events_errorecb: crate::Reg<events_errorecb::EVENTS_ERRORECB_SPEC>,
    _reserved4: [u8; 0x01fc],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved6: [u8; 0x01f8],
    #[doc = "0x504 - ECB block encrypt memory pointer."]
    pub ecbdataptr: crate::Reg<ecbdataptr::ECBDATAPTR_SPEC>,
    _reserved7: [u8; 0x0af4],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_STARTECB register accessor: an alias for `Reg<TASKS_STARTECB_SPEC>`"]
pub type TASKS_STARTECB = crate::Reg<tasks_startecb::TASKS_STARTECB_SPEC>;
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
pub mod tasks_startecb;
#[doc = "TASKS_STOPECB register accessor: an alias for `Reg<TASKS_STOPECB_SPEC>`"]
pub type TASKS_STOPECB = crate::Reg<tasks_stopecb::TASKS_STOPECB_SPEC>;
#[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
pub mod tasks_stopecb;
#[doc = "EVENTS_ENDECB register accessor: an alias for `Reg<EVENTS_ENDECB_SPEC>`"]
pub type EVENTS_ENDECB = crate::Reg<events_endecb::EVENTS_ENDECB_SPEC>;
#[doc = "ECB block encrypt complete."]
pub mod events_endecb;
#[doc = "EVENTS_ERRORECB register accessor: an alias for `Reg<EVENTS_ERRORECB_SPEC>`"]
pub type EVENTS_ERRORECB = crate::Reg<events_errorecb::EVENTS_ERRORECB_SPEC>;
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error."]
pub mod events_errorecb;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ECBDATAPTR register accessor: an alias for `Reg<ECBDATAPTR_SPEC>`"]
pub type ECBDATAPTR = crate::Reg<ecbdataptr::ECBDATAPTR_SPEC>;
#[doc = "ECB block encrypt memory pointer."]
pub mod ecbdataptr;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
