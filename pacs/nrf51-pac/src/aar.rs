#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Stop resolving addresses."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xf4],
    #[doc = "0x100 - Address resolution procedure completed."]
    pub events_end: EVENTS_END,
    #[doc = "0x104 - Address resolved."]
    pub events_resolved: EVENTS_RESOLVED,
    #[doc = "0x108 - Address not resolved."]
    pub events_notresolved: EVENTS_NOTRESOLVED,
    _reserved5: [u8; 0x01f8],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 0xf4],
    #[doc = "0x400 - Resolution status."]
    pub status: STATUS,
    _reserved8: [u8; 0xfc],
    #[doc = "0x500 - Enable AAR."]
    pub enable: ENABLE,
    #[doc = "0x504 - Number of Identity root Keys in the IRK data structure."]
    pub nirk: NIRK,
    #[doc = "0x508 - Pointer to the IRK data structure."]
    pub irkptr: IRKPTR,
    _reserved11: [u8; 0x04],
    #[doc = "0x510 - Pointer to the resolvable address (6 bytes)."]
    pub addrptr: ADDRPTR,
    #[doc = "0x514 - Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
    pub scratchptr: SCRATCHPTR,
    _reserved13: [u8; 0x0ae4],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop resolving addresses."]
pub mod tasks_stop;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Address resolution procedure completed."]
pub mod events_end;
#[doc = "EVENTS_RESOLVED (rw) register accessor: an alias for `Reg<EVENTS_RESOLVED_SPEC>`"]
pub type EVENTS_RESOLVED = crate::Reg<events_resolved::EVENTS_RESOLVED_SPEC>;
#[doc = "Address resolved."]
pub mod events_resolved;
#[doc = "EVENTS_NOTRESOLVED (rw) register accessor: an alias for `Reg<EVENTS_NOTRESOLVED_SPEC>`"]
pub type EVENTS_NOTRESOLVED = crate::Reg<events_notresolved::EVENTS_NOTRESOLVED_SPEC>;
#[doc = "Address not resolved."]
pub mod events_notresolved;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Resolution status."]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable AAR."]
pub mod enable;
#[doc = "NIRK (rw) register accessor: an alias for `Reg<NIRK_SPEC>`"]
pub type NIRK = crate::Reg<nirk::NIRK_SPEC>;
#[doc = "Number of Identity root Keys in the IRK data structure."]
pub mod nirk;
#[doc = "IRKPTR (rw) register accessor: an alias for `Reg<IRKPTR_SPEC>`"]
pub type IRKPTR = crate::Reg<irkptr::IRKPTR_SPEC>;
#[doc = "Pointer to the IRK data structure."]
pub mod irkptr;
#[doc = "ADDRPTR (rw) register accessor: an alias for `Reg<ADDRPTR_SPEC>`"]
pub type ADDRPTR = crate::Reg<addrptr::ADDRPTR_SPEC>;
#[doc = "Pointer to the resolvable address (6 bytes)."]
pub mod addrptr;
#[doc = "SCRATCHPTR (rw) register accessor: an alias for `Reg<SCRATCHPTR_SPEC>`"]
pub type SCRATCHPTR = crate::Reg<scratchptr::SCRATCHPTR_SPEC>;
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
pub mod scratchptr;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
