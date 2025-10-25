#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    _reserved1: [u8; 0xfc],
    events_timeout: EventsTimeout,
    _reserved2: [u8; 0x0200],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved4: [u8; 0xf4],
    runstatus: Runstatus,
    reqstatus: Reqstatus,
    _reserved6: [u8; 0xfc],
    crv: Crv,
    rren: Rren,
    config: Config,
    _reserved9: [u8; 0xf0],
    rr: [Rr; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Start the watchdog"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x100 - Watchdog timeout"]
    #[inline(always)]
    pub const fn events_timeout(&self) -> &EventsTimeout {
        &self.events_timeout
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
    #[doc = "0x400 - Run status"]
    #[inline(always)]
    pub const fn runstatus(&self) -> &Runstatus {
        &self.runstatus
    }
    #[doc = "0x404 - Request status"]
    #[inline(always)]
    pub const fn reqstatus(&self) -> &Reqstatus {
        &self.reqstatus
    }
    #[doc = "0x504 - Counter reload value"]
    #[inline(always)]
    pub const fn crv(&self) -> &Crv {
        &self.crv
    }
    #[doc = "0x508 - Enable register for reload request registers"]
    #[inline(always)]
    pub const fn rren(&self) -> &Rren {
        &self.rren
    }
    #[doc = "0x50c - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x600..0x620 - Description collection\\[0\\]: Reload request 0"]
    #[inline(always)]
    pub const fn rr(&self, n: usize) -> &Rr {
        &self.rr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x620 - Description collection\\[0\\]: Reload request 0"]
    #[inline(always)]
    pub fn rr_iter(&self) -> impl Iterator<Item = &Rr> {
        self.rr.iter()
    }
}
#[doc = "TASKS_START (w) register accessor: Start the watchdog\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start the watchdog"]
pub mod tasks_start;
#[doc = "EVENTS_TIMEOUT (rw) register accessor: Watchdog timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_timeout`] module"]
#[doc(alias = "EVENTS_TIMEOUT")]
pub type EventsTimeout = crate::Reg<events_timeout::EventsTimeoutSpec>;
#[doc = "Watchdog timeout"]
pub mod events_timeout;
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
#[doc = "RUNSTATUS (r) register accessor: Run status\n\nYou can [`read`](crate::Reg::read) this register and get [`runstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@runstatus`] module"]
#[doc(alias = "RUNSTATUS")]
pub type Runstatus = crate::Reg<runstatus::RunstatusSpec>;
#[doc = "Run status"]
pub mod runstatus;
#[doc = "REQSTATUS (r) register accessor: Request status\n\nYou can [`read`](crate::Reg::read) this register and get [`reqstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqstatus`] module"]
#[doc(alias = "REQSTATUS")]
pub type Reqstatus = crate::Reg<reqstatus::ReqstatusSpec>;
#[doc = "Request status"]
pub mod reqstatus;
#[doc = "CRV (rw) register accessor: Counter reload value\n\nYou can [`read`](crate::Reg::read) this register and get [`crv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crv`] module"]
#[doc(alias = "CRV")]
pub type Crv = crate::Reg<crv::CrvSpec>;
#[doc = "Counter reload value"]
pub mod crv;
#[doc = "RREN (rw) register accessor: Enable register for reload request registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rren`] module"]
#[doc(alias = "RREN")]
pub type Rren = crate::Reg<rren::RrenSpec>;
#[doc = "Enable register for reload request registers"]
pub mod rren;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "RR (w) register accessor: Description collection\\[0\\]: Reload request 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`] module"]
#[doc(alias = "RR")]
pub type Rr = crate::Reg<rr::RrSpec>;
#[doc = "Description collection\\[0\\]: Reload request 0"]
pub mod rr;
