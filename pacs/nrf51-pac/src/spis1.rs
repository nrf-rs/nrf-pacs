#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x24],
    tasks_acquire: TasksAcquire,
    tasks_release: TasksRelease,
    _reserved2: [u8; 0xd8],
    events_end: EventsEnd,
    _reserved3: [u8; 0x08],
    events_endrx: EventsEndrx,
    _reserved4: [u8; 0x14],
    events_acquired: EventsAcquired,
    _reserved5: [u8; 0xd4],
    shorts: Shorts,
    _reserved6: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved8: [u8; 0xf4],
    semstat: Semstat,
    _reserved9: [u8; 0x3c],
    status: Status,
    _reserved10: [u8; 0xbc],
    enable: Enable,
    _reserved11: [u8; 0x04],
    pselsck: Pselsck,
    pselmiso: Pselmiso,
    pselmosi: Pselmosi,
    pselcsn: Pselcsn,
    _reserved15: [u8; 0x1c],
    rxdptr: Rxdptr,
    maxrx: Maxrx,
    amountrx: Amountrx,
    _reserved18: [u8; 0x04],
    txdptr: Txdptr,
    maxtx: Maxtx,
    amounttx: Amounttx,
    _reserved21: [u8; 0x04],
    config: Config,
    _reserved22: [u8; 0x04],
    def: Def,
    _reserved23: [u8; 0x60],
    orc: Orc,
    _reserved24: [u8; 0x0a38],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x24 - Acquire SPI semaphore."]
    #[inline(always)]
    pub const fn tasks_acquire(&self) -> &TasksAcquire {
        &self.tasks_acquire
    }
    #[doc = "0x28 - Release SPI semaphore."]
    #[inline(always)]
    pub const fn tasks_release(&self) -> &TasksRelease {
        &self.tasks_release
    }
    #[doc = "0x104 - Granted transaction completed."]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x110 - End of RXD buffer reached"]
    #[inline(always)]
    pub const fn events_endrx(&self) -> &EventsEndrx {
        &self.events_endrx
    }
    #[doc = "0x128 - Semaphore acquired."]
    #[inline(always)]
    pub const fn events_acquired(&self) -> &EventsAcquired {
        &self.events_acquired
    }
    #[doc = "0x200 - Shortcuts for SPIS."]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x304 - Interrupt enable set register."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Interrupt enable clear register."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x400 - Semaphore status."]
    #[inline(always)]
    pub const fn semstat(&self) -> &Semstat {
        &self.semstat
    }
    #[doc = "0x440 - Status from last transaction."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Enable SPIS."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508 - Pin select for SCK."]
    #[inline(always)]
    pub const fn pselsck(&self) -> &Pselsck {
        &self.pselsck
    }
    #[doc = "0x50c - Pin select for MISO."]
    #[inline(always)]
    pub const fn pselmiso(&self) -> &Pselmiso {
        &self.pselmiso
    }
    #[doc = "0x510 - Pin select for MOSI."]
    #[inline(always)]
    pub const fn pselmosi(&self) -> &Pselmosi {
        &self.pselmosi
    }
    #[doc = "0x514 - Pin select for CSN."]
    #[inline(always)]
    pub const fn pselcsn(&self) -> &Pselcsn {
        &self.pselcsn
    }
    #[doc = "0x534 - RX data pointer."]
    #[inline(always)]
    pub const fn rxdptr(&self) -> &Rxdptr {
        &self.rxdptr
    }
    #[doc = "0x538 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub const fn maxrx(&self) -> &Maxrx {
        &self.maxrx
    }
    #[doc = "0x53c - Number of bytes received in last granted transaction."]
    #[inline(always)]
    pub const fn amountrx(&self) -> &Amountrx {
        &self.amountrx
    }
    #[doc = "0x544 - TX data pointer."]
    #[inline(always)]
    pub const fn txdptr(&self) -> &Txdptr {
        &self.txdptr
    }
    #[doc = "0x548 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub const fn maxtx(&self) -> &Maxtx {
        &self.maxtx
    }
    #[doc = "0x54c - Number of bytes transmitted in last granted transaction."]
    #[inline(always)]
    pub const fn amounttx(&self) -> &Amounttx {
        &self.amounttx
    }
    #[doc = "0x554 - Configuration register."]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x55c - Default character."]
    #[inline(always)]
    pub const fn def(&self) -> &Def {
        &self.def
    }
    #[doc = "0x5c0 - Over-read character."]
    #[inline(always)]
    pub const fn orc(&self) -> &Orc {
        &self.orc
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_ACQUIRE (w) register accessor: Acquire SPI semaphore.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_acquire::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_acquire`] module"]
#[doc(alias = "TASKS_ACQUIRE")]
pub type TasksAcquire = crate::Reg<tasks_acquire::TasksAcquireSpec>;
#[doc = "Acquire SPI semaphore."]
pub mod tasks_acquire;
#[doc = "TASKS_RELEASE (w) register accessor: Release SPI semaphore.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_release`] module"]
#[doc(alias = "TASKS_RELEASE")]
pub type TasksRelease = crate::Reg<tasks_release::TasksReleaseSpec>;
#[doc = "Release SPI semaphore."]
pub mod tasks_release;
#[doc = "EVENTS_END (rw) register accessor: Granted transaction completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`] module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Granted transaction completed."]
pub mod events_end;
#[doc = "EVENTS_ENDRX (rw) register accessor: End of RXD buffer reached\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endrx`] module"]
#[doc(alias = "EVENTS_ENDRX")]
pub type EventsEndrx = crate::Reg<events_endrx::EventsEndrxSpec>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_ACQUIRED (rw) register accessor: Semaphore acquired.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_acquired::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_acquired::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_acquired`] module"]
#[doc(alias = "EVENTS_ACQUIRED")]
pub type EventsAcquired = crate::Reg<events_acquired::EventsAcquiredSpec>;
#[doc = "Semaphore acquired."]
pub mod events_acquired;
#[doc = "SHORTS (rw) register accessor: Shortcuts for SPIS.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts for SPIS."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "SEMSTAT (r) register accessor: Semaphore status.\n\nYou can [`read`](crate::Reg::read) this register and get [`semstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@semstat`] module"]
#[doc(alias = "SEMSTAT")]
pub type Semstat = crate::Reg<semstat::SemstatSpec>;
#[doc = "Semaphore status."]
pub mod semstat;
#[doc = "STATUS (rw) register accessor: Status from last transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status from last transaction."]
pub mod status;
#[doc = "ENABLE (rw) register accessor: Enable SPIS.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable SPIS."]
pub mod enable;
#[doc = "PSELSCK (rw) register accessor: Pin select for SCK.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselsck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselsck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselsck`] module"]
#[doc(alias = "PSELSCK")]
pub type Pselsck = crate::Reg<pselsck::PselsckSpec>;
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "PSELMISO (rw) register accessor: Pin select for MISO.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselmiso::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselmiso::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselmiso`] module"]
#[doc(alias = "PSELMISO")]
pub type Pselmiso = crate::Reg<pselmiso::PselmisoSpec>;
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "PSELMOSI (rw) register accessor: Pin select for MOSI.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselmosi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselmosi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselmosi`] module"]
#[doc(alias = "PSELMOSI")]
pub type Pselmosi = crate::Reg<pselmosi::PselmosiSpec>;
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "PSELCSN (rw) register accessor: Pin select for CSN.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselcsn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselcsn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselcsn`] module"]
#[doc(alias = "PSELCSN")]
pub type Pselcsn = crate::Reg<pselcsn::PselcsnSpec>;
#[doc = "Pin select for CSN."]
pub mod pselcsn;
#[doc = "RXDPTR (rw) register accessor: RX data pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdptr`] module"]
#[doc(alias = "RXDPTR")]
pub type Rxdptr = crate::Reg<rxdptr::RxdptrSpec>;
#[doc = "RX data pointer."]
pub mod rxdptr;
#[doc = "MAXRX (rw) register accessor: Maximum number of bytes in the receive buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxrx`] module"]
#[doc(alias = "MAXRX")]
pub type Maxrx = crate::Reg<maxrx::MaxrxSpec>;
#[doc = "Maximum number of bytes in the receive buffer."]
pub mod maxrx;
#[doc = "AMOUNTRX (r) register accessor: Number of bytes received in last granted transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`amountrx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amountrx`] module"]
#[doc(alias = "AMOUNTRX")]
pub type Amountrx = crate::Reg<amountrx::AmountrxSpec>;
#[doc = "Number of bytes received in last granted transaction."]
pub mod amountrx;
#[doc = "TXDPTR (rw) register accessor: TX data pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdptr`] module"]
#[doc(alias = "TXDPTR")]
pub type Txdptr = crate::Reg<txdptr::TxdptrSpec>;
#[doc = "TX data pointer."]
pub mod txdptr;
#[doc = "MAXTX (rw) register accessor: Maximum number of bytes in the transmit buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxtx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxtx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxtx`] module"]
#[doc(alias = "MAXTX")]
pub type Maxtx = crate::Reg<maxtx::MaxtxSpec>;
#[doc = "Maximum number of bytes in the transmit buffer."]
pub mod maxtx;
#[doc = "AMOUNTTX (r) register accessor: Number of bytes transmitted in last granted transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`amounttx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amounttx`] module"]
#[doc(alias = "AMOUNTTX")]
pub type Amounttx = crate::Reg<amounttx::AmounttxSpec>;
#[doc = "Number of bytes transmitted in last granted transaction."]
pub mod amounttx;
#[doc = "CONFIG (rw) register accessor: Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "DEF (rw) register accessor: Default character.\n\nYou can [`read`](crate::Reg::read) this register and get [`def::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`def::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@def`] module"]
#[doc(alias = "DEF")]
pub type Def = crate::Reg<def::DefSpec>;
#[doc = "Default character."]
pub mod def;
#[doc = "ORC (rw) register accessor: Over-read character.\n\nYou can [`read`](crate::Reg::read) this register and get [`orc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`orc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orc`] module"]
#[doc(alias = "ORC")]
pub type Orc = crate::Reg<orc::OrcSpec>;
#[doc = "Over-read character."]
pub mod orc;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
