#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_hfclkstart: TasksHfclkstart,
    tasks_hfclkstop: TasksHfclkstop,
    tasks_lfclkstart: TasksLfclkstart,
    tasks_lfclkstop: TasksLfclkstop,
    _reserved4: [u8; 0x70],
    subscribe_hfclkstart: SubscribeHfclkstart,
    subscribe_hfclkstop: SubscribeHfclkstop,
    subscribe_lfclkstart: SubscribeLfclkstart,
    subscribe_lfclkstop: SubscribeLfclkstop,
    _reserved8: [u8; 0x70],
    events_hfclkstarted: EventsHfclkstarted,
    events_lfclkstarted: EventsLfclkstarted,
    _reserved10: [u8; 0x78],
    publish_hfclkstarted: PublishHfclkstarted,
    publish_lfclkstarted: PublishLfclkstarted,
    _reserved12: [u8; 0x0178],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved16: [u8; 0xf8],
    hfclkrun: Hfclkrun,
    hfclkstat: Hfclkstat,
    _reserved18: [u8; 0x04],
    lfclkrun: Lfclkrun,
    lfclkstat: Lfclkstat,
    lfclksrccopy: Lfclksrccopy,
    _reserved21: [u8; 0xf8],
    lfclksrc: Lfclksrc,
}
impl RegisterBlock {
    #[doc = "0x00 - Start HFCLK source"]
    #[inline(always)]
    pub const fn tasks_hfclkstart(&self) -> &TasksHfclkstart {
        &self.tasks_hfclkstart
    }
    #[doc = "0x04 - Stop HFCLK source"]
    #[inline(always)]
    pub const fn tasks_hfclkstop(&self) -> &TasksHfclkstop {
        &self.tasks_hfclkstop
    }
    #[doc = "0x08 - Start LFCLK source"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(&self) -> &TasksLfclkstart {
        &self.tasks_lfclkstart
    }
    #[doc = "0x0c - Stop LFCLK source"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(&self) -> &TasksLfclkstop {
        &self.tasks_lfclkstop
    }
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclkstart(&self) -> &SubscribeHfclkstart {
        &self.subscribe_hfclkstart
    }
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclkstop(&self) -> &SubscribeHfclkstop {
        &self.subscribe_hfclkstop
    }
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_lfclkstart(&self) -> &SubscribeLfclkstart {
        &self.subscribe_lfclkstart
    }
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_lfclkstop(&self) -> &SubscribeLfclkstop {
        &self.subscribe_lfclkstop
    }
    #[doc = "0x100 - HFCLK oscillator started"]
    #[inline(always)]
    pub const fn events_hfclkstarted(&self) -> &EventsHfclkstarted {
        &self.events_hfclkstarted
    }
    #[doc = "0x104 - LFCLK started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(&self) -> &EventsLfclkstarted {
        &self.events_lfclkstarted
    }
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclkstarted(&self) -> &PublishHfclkstarted {
        &self.publish_hfclkstarted
    }
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_lfclkstarted(&self) -> &PublishLfclkstarted {
        &self.publish_lfclkstarted
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
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkrun(&self) -> &Hfclkrun {
        &self.hfclkrun
    }
    #[doc = "0x40c - The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
    #[inline(always)]
    pub const fn hfclkstat(&self) -> &Hfclkstat {
        &self.hfclkstat
    }
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclkrun(&self) -> &Lfclkrun {
        &self.lfclkrun
    }
    #[doc = "0x418 - The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
    #[inline(always)]
    pub const fn lfclkstat(&self) -> &Lfclkstat {
        &self.lfclkstat
    }
    #[doc = "0x41c - Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclksrccopy(&self) -> &Lfclksrccopy {
        &self.lfclksrccopy
    }
    #[doc = "0x518 - Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
    #[inline(always)]
    pub const fn lfclksrc(&self) -> &Lfclksrc {
        &self.lfclksrc
    }
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: Start HFCLK source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstart`]
module"]
#[doc(alias = "TASKS_HFCLKSTART")]
pub type TasksHfclkstart = crate::Reg<tasks_hfclkstart::TasksHfclkstartSpec>;
#[doc = "Start HFCLK source"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: Stop HFCLK source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_hfclkstop`]
module"]
#[doc(alias = "TASKS_HFCLKSTOP")]
pub type TasksHfclkstop = crate::Reg<tasks_hfclkstop::TasksHfclkstopSpec>;
#[doc = "Stop HFCLK source"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: Start LFCLK source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstart`]
module"]
#[doc(alias = "TASKS_LFCLKSTART")]
pub type TasksLfclkstart = crate::Reg<tasks_lfclkstart::TasksLfclkstartSpec>;
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: Stop LFCLK source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstop`]
module"]
#[doc(alias = "TASKS_LFCLKSTOP")]
pub type TasksLfclkstop = crate::Reg<tasks_lfclkstop::TasksLfclkstopSpec>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "SUBSCRIBE_HFCLKSTART (rw) register accessor: Subscribe configuration for task HFCLKSTART\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_hfclkstart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_hfclkstart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkstart`]
module"]
#[doc(alias = "SUBSCRIBE_HFCLKSTART")]
pub type SubscribeHfclkstart = crate::Reg<subscribe_hfclkstart::SubscribeHfclkstartSpec>;
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "SUBSCRIBE_HFCLKSTOP (rw) register accessor: Subscribe configuration for task HFCLKSTOP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_hfclkstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_hfclkstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_hfclkstop`]
module"]
#[doc(alias = "SUBSCRIBE_HFCLKSTOP")]
pub type SubscribeHfclkstop = crate::Reg<subscribe_hfclkstop::SubscribeHfclkstopSpec>;
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "SUBSCRIBE_LFCLKSTART (rw) register accessor: Subscribe configuration for task LFCLKSTART\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_lfclkstart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_lfclkstart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstart`]
module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTART")]
pub type SubscribeLfclkstart = crate::Reg<subscribe_lfclkstart::SubscribeLfclkstartSpec>;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "SUBSCRIBE_LFCLKSTOP (rw) register accessor: Subscribe configuration for task LFCLKSTOP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_lfclkstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_lfclkstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstop`]
module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTOP")]
pub type SubscribeLfclkstop = crate::Reg<subscribe_lfclkstop::SubscribeLfclkstopSpec>;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: HFCLK oscillator started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_hfclkstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_hfclkstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_hfclkstarted`]
module"]
#[doc(alias = "EVENTS_HFCLKSTARTED")]
pub type EventsHfclkstarted = crate::Reg<events_hfclkstarted::EventsHfclkstartedSpec>;
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: LFCLK started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_lfclkstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_lfclkstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lfclkstarted`]
module"]
#[doc(alias = "EVENTS_LFCLKSTARTED")]
pub type EventsLfclkstarted = crate::Reg<events_lfclkstarted::EventsLfclkstartedSpec>;
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "PUBLISH_HFCLKSTARTED (rw) register accessor: Publish configuration for event HFCLKSTARTED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_hfclkstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_hfclkstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_hfclkstarted`]
module"]
#[doc(alias = "PUBLISH_HFCLKSTARTED")]
pub type PublishHfclkstarted = crate::Reg<publish_hfclkstarted::PublishHfclkstartedSpec>;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "PUBLISH_LFCLKSTARTED (rw) register accessor: Publish configuration for event LFCLKSTARTED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_lfclkstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_lfclkstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lfclkstarted`]
module"]
#[doc(alias = "PUBLISH_LFCLKSTARTED")]
pub type PublishLfclkstarted = crate::Reg<publish_lfclkstarted::PublishLfclkstartedSpec>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
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
#[doc = "HFCLKRUN (r) register accessor: Status indicating that HFCLKSTART task has been triggered\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkrun::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkrun`]
module"]
#[doc(alias = "HFCLKRUN")]
pub type Hfclkrun = crate::Reg<hfclkrun::HfclkrunSpec>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkstat`]
module"]
#[doc(alias = "HFCLKSTAT")]
pub type Hfclkstat = crate::Reg<hfclkstat::HfclkstatSpec>;
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: Status indicating that LFCLKSTART task has been triggered\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkrun::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkrun`]
module"]
#[doc(alias = "LFCLKRUN")]
pub type Lfclkrun = crate::Reg<lfclkrun::LfclkrunSpec>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkstat`]
module"]
#[doc(alias = "LFCLKSTAT")]
pub type Lfclkstat = crate::Reg<lfclkstat::LfclkstatSpec>;
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksrccopy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrccopy`]
module"]
#[doc(alias = "LFCLKSRCCOPY")]
pub type Lfclksrccopy = crate::Reg<lfclksrccopy::LfclksrccopySpec>;
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC (rw) register accessor: Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksrc`]
module"]
#[doc(alias = "LFCLKSRC")]
pub type Lfclksrc = crate::Reg<lfclksrc::LfclksrcSpec>;
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
pub mod lfclksrc;
