#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt"]
    pub tasks_startecb: TASKS_STARTECB,
    #[doc = "0x04 - Abort a possible executing ECB operation"]
    pub tasks_stopecb: TASKS_STOPECB,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Subscribe configuration for task STARTECB"]
    pub subscribe_startecb: SUBSCRIBE_STARTECB,
    #[doc = "0x84 - Subscribe configuration for task STOPECB"]
    pub subscribe_stopecb: SUBSCRIBE_STOPECB,
    _reserved4: [u8; 0x78],
    #[doc = "0x100 - ECB block encrypt complete"]
    pub events_endecb: EVENTS_ENDECB,
    #[doc = "0x104 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    pub events_errorecb: EVENTS_ERRORECB,
    _reserved6: [u8; 0x78],
    #[doc = "0x180 - Publish configuration for event ENDECB"]
    pub publish_endecb: PUBLISH_ENDECB,
    #[doc = "0x184 - Publish configuration for event ERRORECB"]
    pub publish_errorecb: PUBLISH_ERRORECB,
    _reserved8: [u8; 0x017c],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 0x01f8],
    #[doc = "0x504 - ECB block encrypt memory pointers"]
    pub ecbdataptr: ECBDATAPTR,
}
#[doc = "TASKS_STARTECB (w) register accessor: an alias for `Reg<TASKS_STARTECB_SPEC>`"]
pub type TASKS_STARTECB = crate::Reg<tasks_startecb::TASKS_STARTECB_SPEC>;
#[doc = "Start ECB block encrypt"]
pub mod tasks_startecb;
#[doc = "TASKS_STOPECB (w) register accessor: an alias for `Reg<TASKS_STOPECB_SPEC>`"]
pub type TASKS_STOPECB = crate::Reg<tasks_stopecb::TASKS_STOPECB_SPEC>;
#[doc = "Abort a possible executing ECB operation"]
pub mod tasks_stopecb;
#[doc = "SUBSCRIBE_STARTECB (rw) register accessor: an alias for `Reg<SUBSCRIBE_STARTECB_SPEC>`"]
pub type SUBSCRIBE_STARTECB = crate::Reg<subscribe_startecb::SUBSCRIBE_STARTECB_SPEC>;
#[doc = "Subscribe configuration for task STARTECB"]
pub mod subscribe_startecb;
#[doc = "SUBSCRIBE_STOPECB (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOPECB_SPEC>`"]
pub type SUBSCRIBE_STOPECB = crate::Reg<subscribe_stopecb::SUBSCRIBE_STOPECB_SPEC>;
#[doc = "Subscribe configuration for task STOPECB"]
pub mod subscribe_stopecb;
#[doc = "EVENTS_ENDECB (rw) register accessor: an alias for `Reg<EVENTS_ENDECB_SPEC>`"]
pub type EVENTS_ENDECB = crate::Reg<events_endecb::EVENTS_ENDECB_SPEC>;
#[doc = "ECB block encrypt complete"]
pub mod events_endecb;
#[doc = "EVENTS_ERRORECB (rw) register accessor: an alias for `Reg<EVENTS_ERRORECB_SPEC>`"]
pub type EVENTS_ERRORECB = crate::Reg<events_errorecb::EVENTS_ERRORECB_SPEC>;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub mod events_errorecb;
#[doc = "PUBLISH_ENDECB (rw) register accessor: an alias for `Reg<PUBLISH_ENDECB_SPEC>`"]
pub type PUBLISH_ENDECB = crate::Reg<publish_endecb::PUBLISH_ENDECB_SPEC>;
#[doc = "Publish configuration for event ENDECB"]
pub mod publish_endecb;
#[doc = "PUBLISH_ERRORECB (rw) register accessor: an alias for `Reg<PUBLISH_ERRORECB_SPEC>`"]
pub type PUBLISH_ERRORECB = crate::Reg<publish_errorecb::PUBLISH_ERRORECB_SPEC>;
#[doc = "Publish configuration for event ERRORECB"]
pub mod publish_errorecb;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ECBDATAPTR (rw) register accessor: an alias for `Reg<ECBDATAPTR_SPEC>`"]
pub type ECBDATAPTR = crate::Reg<ecbdataptr::ECBDATAPTR_SPEC>;
#[doc = "ECB block encrypt memory pointers"]
pub mod ecbdataptr;
