#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_push_keyslot: TasksPushKeyslot,
    _reserved1: [u8; 0xfc],
    events_keyslot_pushed: EventsKeyslotPushed,
    events_keyslot_revoked: EventsKeyslotRevoked,
    events_keyslot_error: EventsKeyslotError,
    _reserved4: [u8; 0x01f4],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved8: [u8; 0xfc],
    status: Status,
    _reserved9: [u8; 0xf0],
    selectkeyslot: Selectkeyslot,
}
impl RegisterBlock {
    #[doc = "0x00 - Push a key slot over secure APB"]
    #[inline(always)]
    pub const fn tasks_push_keyslot(&self) -> &TasksPushKeyslot {
        &self.tasks_push_keyslot
    }
    #[doc = "0x100 - Key slot successfully pushed over secure APB"]
    #[inline(always)]
    pub const fn events_keyslot_pushed(&self) -> &EventsKeyslotPushed {
        &self.events_keyslot_pushed
    }
    #[doc = "0x104 - Key slot has been revoked and cannot be tasked for selection"]
    #[inline(always)]
    pub const fn events_keyslot_revoked(&self) -> &EventsKeyslotRevoked {
        &self.events_keyslot_revoked
    }
    #[doc = "0x108 - No key slot selected, no destination address defined, or error during push operation"]
    #[inline(always)]
    pub const fn events_keyslot_error(&self) -> &EventsKeyslotError {
        &self.events_keyslot_error
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x30c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x40c - Status bits for KMU operation"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
    #[inline(always)]
    pub const fn selectkeyslot(&self) -> &Selectkeyslot {
        &self.selectkeyslot
    }
}
#[doc = "TASKS_PUSH_KEYSLOT (w) register accessor: Push a key slot over secure APB\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_push_keyslot::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_push_keyslot`]
module"]
#[doc(alias = "TASKS_PUSH_KEYSLOT")]
pub type TasksPushKeyslot = crate::Reg<tasks_push_keyslot::TasksPushKeyslotSpec>;
#[doc = "Push a key slot over secure APB"]
pub mod tasks_push_keyslot;
#[doc = "EVENTS_KEYSLOT_PUSHED (rw) register accessor: Key slot successfully pushed over secure APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_pushed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_pushed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_keyslot_pushed`]
module"]
#[doc(alias = "EVENTS_KEYSLOT_PUSHED")]
pub type EventsKeyslotPushed = crate::Reg<events_keyslot_pushed::EventsKeyslotPushedSpec>;
#[doc = "Key slot successfully pushed over secure APB"]
pub mod events_keyslot_pushed;
#[doc = "EVENTS_KEYSLOT_REVOKED (rw) register accessor: Key slot has been revoked and cannot be tasked for selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_revoked::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_revoked::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_keyslot_revoked`]
module"]
#[doc(alias = "EVENTS_KEYSLOT_REVOKED")]
pub type EventsKeyslotRevoked = crate::Reg<events_keyslot_revoked::EventsKeyslotRevokedSpec>;
#[doc = "Key slot has been revoked and cannot be tasked for selection"]
pub mod events_keyslot_revoked;
#[doc = "EVENTS_KEYSLOT_ERROR (rw) register accessor: No key slot selected, no destination address defined, or error during push operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_keyslot_error`]
module"]
#[doc(alias = "EVENTS_KEYSLOT_ERROR")]
pub type EventsKeyslotError = crate::Reg<events_keyslot_error::EventsKeyslotErrorSpec>;
#[doc = "No key slot selected, no destination address defined, or error during push operation"]
pub mod events_keyslot_error;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "INTPEND (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "STATUS (r) register accessor: Status bits for KMU operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status bits for KMU operation"]
pub mod status;
#[doc = "SELECTKEYSLOT (rw) register accessor: Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selectkeyslot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selectkeyslot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selectkeyslot`]
module"]
#[doc(alias = "SELECTKEYSLOT")]
pub type Selectkeyslot = crate::Reg<selectkeyslot::SelectkeyslotSpec>;
#[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
pub mod selectkeyslot;
