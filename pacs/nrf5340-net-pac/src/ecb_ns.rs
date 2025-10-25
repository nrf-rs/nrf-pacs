#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_startecb: TasksStartecb,
    tasks_stopecb: TasksStopecb,
    _reserved2: [u8; 0x78],
    subscribe_startecb: SubscribeStartecb,
    subscribe_stopecb: SubscribeStopecb,
    _reserved4: [u8; 0x78],
    events_endecb: EventsEndecb,
    events_errorecb: EventsErrorecb,
    _reserved6: [u8; 0x78],
    publish_endecb: PublishEndecb,
    publish_errorecb: PublishErrorecb,
    _reserved8: [u8; 0x017c],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved10: [u8; 0x01f8],
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
    #[doc = "0x80 - Subscribe configuration for task STARTECB"]
    #[inline(always)]
    pub const fn subscribe_startecb(&self) -> &SubscribeStartecb {
        &self.subscribe_startecb
    }
    #[doc = "0x84 - Subscribe configuration for task STOPECB"]
    #[inline(always)]
    pub const fn subscribe_stopecb(&self) -> &SubscribeStopecb {
        &self.subscribe_stopecb
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
    #[doc = "0x180 - Publish configuration for event ENDECB"]
    #[inline(always)]
    pub const fn publish_endecb(&self) -> &PublishEndecb {
        &self.publish_endecb
    }
    #[doc = "0x184 - Publish configuration for event ERRORECB"]
    #[inline(always)]
    pub const fn publish_errorecb(&self) -> &PublishErrorecb {
        &self.publish_errorecb
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
#[doc = "SUBSCRIBE_STARTECB (rw) register accessor: Subscribe configuration for task STARTECB\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_startecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_startecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startecb`] module"]
#[doc(alias = "SUBSCRIBE_STARTECB")]
pub type SubscribeStartecb = crate::Reg<subscribe_startecb::SubscribeStartecbSpec>;
#[doc = "Subscribe configuration for task STARTECB"]
pub mod subscribe_startecb;
#[doc = "SUBSCRIBE_STOPECB (rw) register accessor: Subscribe configuration for task STOPECB\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_stopecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_stopecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stopecb`] module"]
#[doc(alias = "SUBSCRIBE_STOPECB")]
pub type SubscribeStopecb = crate::Reg<subscribe_stopecb::SubscribeStopecbSpec>;
#[doc = "Subscribe configuration for task STOPECB"]
pub mod subscribe_stopecb;
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
#[doc = "PUBLISH_ENDECB (rw) register accessor: Publish configuration for event ENDECB\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endecb`] module"]
#[doc(alias = "PUBLISH_ENDECB")]
pub type PublishEndecb = crate::Reg<publish_endecb::PublishEndecbSpec>;
#[doc = "Publish configuration for event ENDECB"]
pub mod publish_endecb;
#[doc = "PUBLISH_ERRORECB (rw) register accessor: Publish configuration for event ERRORECB\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_errorecb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_errorecb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_errorecb`] module"]
#[doc(alias = "PUBLISH_ERRORECB")]
pub type PublishErrorecb = crate::Reg<publish_errorecb::PublishErrorecbSpec>;
#[doc = "Publish configuration for event ERRORECB"]
pub mod publish_errorecb;
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
