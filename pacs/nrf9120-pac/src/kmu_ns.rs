#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Push a key slot over secure APB"]
    pub tasks_push_keyslot: TASKS_PUSH_KEYSLOT,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - Key slot successfully pushed over secure APB"]
    pub events_keyslot_pushed: EVENTS_KEYSLOT_PUSHED,
    #[doc = "0x104 - Key slot has been revoked and cannot be tasked for selection"]
    pub events_keyslot_revoked: EVENTS_KEYSLOT_REVOKED,
    #[doc = "0x108 - No key slot selected, no destination address defined, or error during push operation"]
    pub events_keyslot_error: EVENTS_KEYSLOT_ERROR,
    _reserved4: [u8; 0x01f4],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved8: [u8; 0xfc],
    #[doc = "0x40c - Status bits for KMU operation"]
    pub status: STATUS,
    _reserved9: [u8; 0xf0],
    #[doc = "0x500 - Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
    pub selectkeyslot: SELECTKEYSLOT,
}
#[doc = "TASKS_PUSH_KEYSLOT (w) register accessor: an alias for `Reg<TASKS_PUSH_KEYSLOT_SPEC>`"]
pub type TASKS_PUSH_KEYSLOT = crate::Reg<tasks_push_keyslot::TASKS_PUSH_KEYSLOT_SPEC>;
#[doc = "Push a key slot over secure APB"]
pub mod tasks_push_keyslot;
#[doc = "EVENTS_KEYSLOT_PUSHED (rw) register accessor: an alias for `Reg<EVENTS_KEYSLOT_PUSHED_SPEC>`"]
pub type EVENTS_KEYSLOT_PUSHED = crate::Reg<events_keyslot_pushed::EVENTS_KEYSLOT_PUSHED_SPEC>;
#[doc = "Key slot successfully pushed over secure APB"]
pub mod events_keyslot_pushed;
#[doc = "EVENTS_KEYSLOT_REVOKED (rw) register accessor: an alias for `Reg<EVENTS_KEYSLOT_REVOKED_SPEC>`"]
pub type EVENTS_KEYSLOT_REVOKED = crate::Reg<events_keyslot_revoked::EVENTS_KEYSLOT_REVOKED_SPEC>;
#[doc = "Key slot has been revoked and cannot be tasked for selection"]
pub mod events_keyslot_revoked;
#[doc = "EVENTS_KEYSLOT_ERROR (rw) register accessor: an alias for `Reg<EVENTS_KEYSLOT_ERROR_SPEC>`"]
pub type EVENTS_KEYSLOT_ERROR = crate::Reg<events_keyslot_error::EVENTS_KEYSLOT_ERROR_SPEC>;
#[doc = "No key slot selected, no destination address defined, or error during push operation"]
pub mod events_keyslot_error;
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
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status bits for KMU operation"]
pub mod status;
#[doc = "SELECTKEYSLOT (rw) register accessor: an alias for `Reg<SELECTKEYSLOT_SPEC>`"]
pub type SELECTKEYSLOT = crate::Reg<selectkeyslot::SELECTKEYSLOT_SPEC>;
#[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
pub mod selectkeyslot;
