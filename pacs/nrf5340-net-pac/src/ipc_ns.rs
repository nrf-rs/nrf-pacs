#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
    pub tasks_send: [TASKS_SEND; 16],
    _reserved1: [u8; 0x40],
    #[doc = "0x80..0xc0 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    pub subscribe_send: [SUBSCRIBE_SEND; 16],
    _reserved2: [u8; 0x40],
    #[doc = "0x100..0x140 - Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    pub events_receive: [EVENTS_RECEIVE; 16],
    _reserved3: [u8; 0x40],
    #[doc = "0x180..0x1c0 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    pub publish_receive: [PUBLISH_RECEIVE; 16],
    _reserved4: [u8; 0x0140],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved8: [u8; 0x0200],
    #[doc = "0x510..0x550 - Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
    pub send_cnf: [SEND_CNF; 16],
    _reserved9: [u8; 0x40],
    #[doc = "0x590..0x5d0 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
    pub receive_cnf: [RECEIVE_CNF; 16],
    _reserved10: [u8; 0x40],
    #[doc = "0x610..0x618 - Description collection: General purpose memory"]
    pub gpmem: [GPMEM; 2],
}
#[doc = "TASKS_SEND (w) register accessor: an alias for `Reg<TASKS_SEND_SPEC>`"]
pub type TASKS_SEND = crate::Reg<tasks_send::TASKS_SEND_SPEC>;
#[doc = "Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
pub mod tasks_send;
#[doc = "SUBSCRIBE_SEND (rw) register accessor: an alias for `Reg<SUBSCRIBE_SEND_SPEC>`"]
pub type SUBSCRIBE_SEND = crate::Reg<subscribe_send::SUBSCRIBE_SEND_SPEC>;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "EVENTS_RECEIVE (rw) register accessor: an alias for `Reg<EVENTS_RECEIVE_SPEC>`"]
pub type EVENTS_RECEIVE = crate::Reg<events_receive::EVENTS_RECEIVE_SPEC>;
#[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub mod events_receive;
#[doc = "PUBLISH_RECEIVE (rw) register accessor: an alias for `Reg<PUBLISH_RECEIVE_SPEC>`"]
pub type PUBLISH_RECEIVE = crate::Reg<publish_receive::PUBLISH_RECEIVE_SPEC>;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
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
#[doc = "INTPEND (r) register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "SEND_CNF (rw) register accessor: an alias for `Reg<SEND_CNF_SPEC>`"]
pub type SEND_CNF = crate::Reg<send_cnf::SEND_CNF_SPEC>;
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
pub mod send_cnf;
#[doc = "RECEIVE_CNF (rw) register accessor: an alias for `Reg<RECEIVE_CNF_SPEC>`"]
pub type RECEIVE_CNF = crate::Reg<receive_cnf::RECEIVE_CNF_SPEC>;
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
pub mod receive_cnf;
#[doc = "GPMEM (rw) register accessor: an alias for `Reg<GPMEM_SPEC>`"]
pub type GPMEM = crate::Reg<gpmem::GPMEM_SPEC>;
#[doc = "Description collection: General purpose memory"]
pub mod gpmem;
