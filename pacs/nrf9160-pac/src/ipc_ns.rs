#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
    pub tasks_send: [crate::Reg<tasks_send::TASKS_SEND_SPEC>; 8],
    _reserved1: [u8; 0x60],
    #[doc = "0x80..0xa0 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    pub subscribe_send: [crate::Reg<subscribe_send::SUBSCRIBE_SEND_SPEC>; 8],
    _reserved2: [u8; 0x60],
    #[doc = "0x100..0x120 - Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
    pub events_receive: [crate::Reg<events_receive::EVENTS_RECEIVE_SPEC>; 8],
    _reserved3: [u8; 0x60],
    #[doc = "0x180..0x1a0 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    pub publish_receive: [crate::Reg<publish_receive::PUBLISH_RECEIVE_SPEC>; 8],
    _reserved4: [u8; 0x0160],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: crate::Reg<intpend::INTPEND_SPEC>,
    _reserved8: [u8; 0x0200],
    #[doc = "0x510..0x530 - Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
    pub send_cnf: [crate::Reg<send_cnf::SEND_CNF_SPEC>; 8],
    _reserved9: [u8; 0x60],
    #[doc = "0x590..0x5b0 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
    pub receive_cnf: [crate::Reg<receive_cnf::RECEIVE_CNF_SPEC>; 8],
    _reserved10: [u8; 0x60],
    #[doc = "0x610..0x620 - Description collection: General purpose memory."]
    pub gpmem: [crate::Reg<gpmem::GPMEM_SPEC>; 4],
}
#[doc = "TASKS_SEND register accessor: an alias for `Reg<TASKS_SEND_SPEC>`"]
pub type TASKS_SEND = crate::Reg<tasks_send::TASKS_SEND_SPEC>;
#[doc = "Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
pub mod tasks_send;
#[doc = "SUBSCRIBE_SEND register accessor: an alias for `Reg<SUBSCRIBE_SEND_SPEC>`"]
pub type SUBSCRIBE_SEND = crate::Reg<subscribe_send::SUBSCRIBE_SEND_SPEC>;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "EVENTS_RECEIVE register accessor: an alias for `Reg<EVENTS_RECEIVE_SPEC>`"]
pub type EVENTS_RECEIVE = crate::Reg<events_receive::EVENTS_RECEIVE_SPEC>;
#[doc = "Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
pub mod events_receive;
#[doc = "PUBLISH_RECEIVE register accessor: an alias for `Reg<PUBLISH_RECEIVE_SPEC>`"]
pub type PUBLISH_RECEIVE = crate::Reg<publish_receive::PUBLISH_RECEIVE_SPEC>;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
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
#[doc = "SEND_CNF register accessor: an alias for `Reg<SEND_CNF_SPEC>`"]
pub type SEND_CNF = crate::Reg<send_cnf::SEND_CNF_SPEC>;
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
pub mod send_cnf;
#[doc = "RECEIVE_CNF register accessor: an alias for `Reg<RECEIVE_CNF_SPEC>`"]
pub type RECEIVE_CNF = crate::Reg<receive_cnf::RECEIVE_CNF_SPEC>;
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
pub mod receive_cnf;
#[doc = "GPMEM register accessor: an alias for `Reg<GPMEM_SPEC>`"]
pub type GPMEM = crate::Reg<gpmem::GPMEM_SPEC>;
#[doc = "Description collection: General purpose memory."]
pub mod gpmem;
