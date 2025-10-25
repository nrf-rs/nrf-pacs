#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_startecb: TasksStartecb,
    tasks_stopecb: TasksStopecb,
    _reserved2: [u8; 0xf8],
    events_endecb: EventsEndecb,
    events_errorecb: EventsErrorecb,
    _reserved4: [u8; 0x01fc],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved6: [u8; 0x01f8],
    ecbdataptr: Ecbdataptr,
}
impl RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt"]
    #[inline(always)]
    pub const fn tasks_startecb(&self) -> &TasksStartecb {
        &self.tasks_startecb
    }
    #[doc = "0x04 - Abort a possible executing ECB operation"]
    #[inline(always)]
    pub const fn tasks_stopecb(&self) -> &TasksStopecb {
        &self.tasks_stopecb
    }
    #[doc = "0x100 - ECB block encrypt complete"]
    #[inline(always)]
    pub const fn events_endecb(&self) -> &EventsEndecb {
        &self.events_endecb
    }
    #[doc = "0x104 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub const fn events_errorecb(&self) -> &EventsErrorecb {
        &self.events_errorecb
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
    #[doc = "0x504 - ECB block encrypt memory pointers"]
    #[inline(always)]
    pub const fn ecbdataptr(&self) -> &Ecbdataptr {
        &self.ecbdataptr
    }
}
#[doc = "TASKS_STARTECB (w) register accessor: Start ECB block encrypt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startecb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startecb`] module"]
#[doc(alias = "TASKS_STARTECB")]
pub type TasksStartecb = crate::Reg<tasks_startecb::TasksStartecbSpec>;
#[doc = "Start ECB block encrypt"]
pub mod tasks_startecb;
#[doc = "TASKS_STOPECB (w) register accessor: Abort a possible executing ECB operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stopecb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stopecb`] module"]
#[doc(alias = "TASKS_STOPECB")]
pub type TasksStopecb = crate::Reg<tasks_stopecb::TasksStopecbSpec>;
#[doc = "Abort a possible executing ECB operation"]
pub mod tasks_stopecb;
#[doc = "EVENTS_ENDECB (rw) register accessor: ECB block encrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endecb`] module"]
#[doc(alias = "EVENTS_ENDECB")]
pub type EventsEndecb = crate::Reg<events_endecb::EventsEndecbSpec>;
#[doc = "ECB block encrypt complete"]
pub mod events_endecb;
#[doc = "EVENTS_ERRORECB (rw) register accessor: ECB block encrypt aborted because of a STOPECB task or due to an error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_errorecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_errorecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_errorecb`] module"]
#[doc(alias = "EVENTS_ERRORECB")]
pub type EventsErrorecb = crate::Reg<events_errorecb::EventsErrorecbSpec>;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub mod events_errorecb;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ECBDATAPTR (rw) register accessor: ECB block encrypt memory pointers\n\nYou can [`read`](crate::Reg::read) this register and get [`ecbdataptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecbdataptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecbdataptr`] module"]
#[doc(alias = "ECBDATAPTR")]
pub type Ecbdataptr = crate::Reg<ecbdataptr::EcbdataptrSpec>;
#[doc = "ECB block encrypt memory pointers"]
pub mod ecbdataptr;
