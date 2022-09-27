#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure"]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Stop resolving addresses"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0x74],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    _reserved3: [u8; 0x04],
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 0x74],
    #[doc = "0x100 - Address resolution procedure complete"]
    pub events_end: EVENTS_END,
    #[doc = "0x104 - Address resolved"]
    pub events_resolved: EVENTS_RESOLVED,
    #[doc = "0x108 - Address not resolved"]
    pub events_notresolved: EVENTS_NOTRESOLVED,
    _reserved7: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    #[doc = "0x184 - Publish configuration for event RESOLVED"]
    pub publish_resolved: PUBLISH_RESOLVED,
    #[doc = "0x188 - Publish configuration for event NOTRESOLVED"]
    pub publish_notresolved: PUBLISH_NOTRESOLVED,
    _reserved10: [u8; 0x0178],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 0xf4],
    #[doc = "0x400 - Resolution status"]
    pub status: STATUS,
    _reserved13: [u8; 0xfc],
    #[doc = "0x500 - Enable AAR"]
    pub enable: ENABLE,
    #[doc = "0x504 - Number of IRKs"]
    pub nirk: NIRK,
    #[doc = "0x508 - Pointer to IRK data structure"]
    pub irkptr: IRKPTR,
    _reserved16: [u8; 0x04],
    #[doc = "0x510 - Pointer to the resolvable address"]
    pub addrptr: ADDRPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop resolving addresses"]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Address resolution procedure complete"]
pub mod events_end;
#[doc = "EVENTS_RESOLVED (rw) register accessor: an alias for `Reg<EVENTS_RESOLVED_SPEC>`"]
pub type EVENTS_RESOLVED = crate::Reg<events_resolved::EVENTS_RESOLVED_SPEC>;
#[doc = "Address resolved"]
pub mod events_resolved;
#[doc = "EVENTS_NOTRESOLVED (rw) register accessor: an alias for `Reg<EVENTS_NOTRESOLVED_SPEC>`"]
pub type EVENTS_NOTRESOLVED = crate::Reg<events_notresolved::EVENTS_NOTRESOLVED_SPEC>;
#[doc = "Address not resolved"]
pub mod events_notresolved;
#[doc = "PUBLISH_END (rw) register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_RESOLVED (rw) register accessor: an alias for `Reg<PUBLISH_RESOLVED_SPEC>`"]
pub type PUBLISH_RESOLVED = crate::Reg<publish_resolved::PUBLISH_RESOLVED_SPEC>;
#[doc = "Publish configuration for event RESOLVED"]
pub mod publish_resolved;
#[doc = "PUBLISH_NOTRESOLVED (rw) register accessor: an alias for `Reg<PUBLISH_NOTRESOLVED_SPEC>`"]
pub type PUBLISH_NOTRESOLVED = crate::Reg<publish_notresolved::PUBLISH_NOTRESOLVED_SPEC>;
#[doc = "Publish configuration for event NOTRESOLVED"]
pub mod publish_notresolved;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Resolution status"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable AAR"]
pub mod enable;
#[doc = "NIRK (rw) register accessor: an alias for `Reg<NIRK_SPEC>`"]
pub type NIRK = crate::Reg<nirk::NIRK_SPEC>;
#[doc = "Number of IRKs"]
pub mod nirk;
#[doc = "IRKPTR (rw) register accessor: an alias for `Reg<IRKPTR_SPEC>`"]
pub type IRKPTR = crate::Reg<irkptr::IRKPTR_SPEC>;
#[doc = "Pointer to IRK data structure"]
pub mod irkptr;
#[doc = "ADDRPTR (rw) register accessor: an alias for `Reg<ADDRPTR_SPEC>`"]
pub type ADDRPTR = crate::Reg<addrptr::ADDRPTR_SPEC>;
#[doc = "Pointer to the resolvable address"]
pub mod addrptr;
#[doc = "SCRATCHPTR (rw) register accessor: an alias for `Reg<SCRATCHPTR_SPEC>`"]
pub type SCRATCHPTR = crate::Reg<scratchptr::SCRATCHPTR_SPEC>;
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
