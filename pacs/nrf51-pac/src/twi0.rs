#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_startrx: TasksStartrx,
    _reserved1: [u8; 0x04],
    tasks_starttx: TasksStarttx,
    _reserved2: [u8; 0x08],
    tasks_stop: TasksStop,
    _reserved3: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved5: [u8; 0xe0],
    events_stopped: EventsStopped,
    events_rxdready: EventsRxdready,
    _reserved7: [u8; 0x10],
    events_txdsent: EventsTxdsent,
    _reserved8: [u8; 0x04],
    events_error: EventsError,
    _reserved9: [u8; 0x10],
    events_bb: EventsBb,
    _reserved10: [u8; 0x0c],
    events_suspended: EventsSuspended,
    _reserved11: [u8; 0xb4],
    shorts: Shorts,
    _reserved12: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved14: [u8; 0x01b8],
    errorsrc: Errorsrc,
    _reserved15: [u8; 0x38],
    enable: Enable,
    _reserved16: [u8; 0x04],
    pselscl: Pselscl,
    pselsda: Pselsda,
    _reserved18: [u8; 0x08],
    rxd: Rxd,
    txd: Txd,
    _reserved20: [u8; 0x04],
    frequency: Frequency,
    _reserved21: [u8; 0x60],
    address: Address,
    _reserved22: [u8; 0x0a70],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Start 2-Wire master receive sequence."]
    #[inline(always)]
    pub const fn tasks_startrx(&self) -> &TasksStartrx {
        &self.tasks_startrx
    }
    #[doc = "0x08 - Start 2-Wire master transmit sequence."]
    #[inline(always)]
    pub const fn tasks_starttx(&self) -> &TasksStarttx {
        &self.tasks_starttx
    }
    #[doc = "0x14 - Stop 2-Wire transaction."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x1c - Suspend 2-Wire transaction."]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x20 - Resume 2-Wire transaction."]
    #[inline(always)]
    pub const fn tasks_resume(&self) -> &TasksResume {
        &self.tasks_resume
    }
    #[doc = "0x104 - Two-wire stopped."]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x108 - Two-wire ready to deliver new RXD byte received."]
    #[inline(always)]
    pub const fn events_rxdready(&self) -> &EventsRxdready {
        &self.events_rxdready
    }
    #[doc = "0x11c - Two-wire finished sending last TXD byte."]
    #[inline(always)]
    pub const fn events_txdsent(&self) -> &EventsTxdsent {
        &self.events_txdsent
    }
    #[doc = "0x124 - Two-wire error detected."]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x138 - Two-wire byte boundary."]
    #[inline(always)]
    pub const fn events_bb(&self) -> &EventsBb {
        &self.events_bb
    }
    #[doc = "0x148 - Two-wire suspended."]
    #[inline(always)]
    pub const fn events_suspended(&self) -> &EventsSuspended {
        &self.events_suspended
    }
    #[doc = "0x200 - Shortcuts for TWI."]
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
    #[doc = "0x4c4 - Two-wire error source. Write error field to 1 to clear error."]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x500 - Enable two-wire master."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508 - Pin select for SCL."]
    #[inline(always)]
    pub const fn pselscl(&self) -> &Pselscl {
        &self.pselscl
    }
    #[doc = "0x50c - Pin select for SDA."]
    #[inline(always)]
    pub const fn pselsda(&self) -> &Pselsda {
        &self.pselsda
    }
    #[doc = "0x518 - RX data register."]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x51c - TX data register."]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x524 - Two-wire frequency."]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x588 - Address used in the two-wire transfer."]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_STARTRX (w) register accessor: Start 2-Wire master receive sequence.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startrx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startrx`] module"]
#[doc(alias = "TASKS_STARTRX")]
pub type TasksStartrx = crate::Reg<tasks_startrx::TasksStartrxSpec>;
#[doc = "Start 2-Wire master receive sequence."]
pub mod tasks_startrx;
#[doc = "TASKS_STARTTX (w) register accessor: Start 2-Wire master transmit sequence.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_starttx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_starttx`] module"]
#[doc(alias = "TASKS_STARTTX")]
pub type TasksStarttx = crate::Reg<tasks_starttx::TasksStarttxSpec>;
#[doc = "Start 2-Wire master transmit sequence."]
pub mod tasks_starttx;
#[doc = "TASKS_STOP (w) register accessor: Stop 2-Wire transaction.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop 2-Wire transaction."]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend 2-Wire transaction.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`] module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend 2-Wire transaction."]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: Resume 2-Wire transaction.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_resume::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_resume`] module"]
#[doc(alias = "TASKS_RESUME")]
pub type TasksResume = crate::Reg<tasks_resume::TasksResumeSpec>;
#[doc = "Resume 2-Wire transaction."]
pub mod tasks_resume;
#[doc = "EVENTS_STOPPED (rw) register accessor: Two-wire stopped.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`] module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "Two-wire stopped."]
pub mod events_stopped;
#[doc = "EVENTS_RXDREADY (rw) register accessor: Two-wire ready to deliver new RXD byte received.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxdready`] module"]
#[doc(alias = "EVENTS_RXDREADY")]
pub type EventsRxdready = crate::Reg<events_rxdready::EventsRxdreadySpec>;
#[doc = "Two-wire ready to deliver new RXD byte received."]
pub mod events_rxdready;
#[doc = "EVENTS_TXDSENT (rw) register accessor: Two-wire finished sending last TXD byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdsent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdsent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txdsent`] module"]
#[doc(alias = "EVENTS_TXDSENT")]
pub type EventsTxdsent = crate::Reg<events_txdsent::EventsTxdsentSpec>;
#[doc = "Two-wire finished sending last TXD byte."]
pub mod events_txdsent;
#[doc = "EVENTS_ERROR (rw) register accessor: Two-wire error detected.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`] module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Two-wire error detected."]
pub mod events_error;
#[doc = "EVENTS_BB (rw) register accessor: Two-wire byte boundary.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_bb`] module"]
#[doc(alias = "EVENTS_BB")]
pub type EventsBb = crate::Reg<events_bb::EventsBbSpec>;
#[doc = "Two-wire byte boundary."]
pub mod events_bb;
#[doc = "EVENTS_SUSPENDED (rw) register accessor: Two-wire suspended.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_suspended::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_suspended::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_suspended`] module"]
#[doc(alias = "EVENTS_SUSPENDED")]
pub type EventsSuspended = crate::Reg<events_suspended::EventsSuspendedSpec>;
#[doc = "Two-wire suspended."]
pub mod events_suspended;
#[doc = "SHORTS (rw) register accessor: Shortcuts for TWI.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts for TWI."]
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
#[doc = "ERRORSRC (rw) register accessor: Two-wire error source. Write error field to 1 to clear error.\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorsrc`] module"]
#[doc(alias = "ERRORSRC")]
pub type Errorsrc = crate::Reg<errorsrc::ErrorsrcSpec>;
#[doc = "Two-wire error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: Enable two-wire master.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable two-wire master."]
pub mod enable;
#[doc = "PSELSCL (rw) register accessor: Pin select for SCL.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselscl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselscl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselscl`] module"]
#[doc(alias = "PSELSCL")]
pub type Pselscl = crate::Reg<pselscl::PselsclSpec>;
#[doc = "Pin select for SCL."]
pub mod pselscl;
#[doc = "PSELSDA (rw) register accessor: Pin select for SDA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselsda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselsda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselsda`] module"]
#[doc(alias = "PSELSDA")]
pub type Pselsda = crate::Reg<pselsda::PselsdaSpec>;
#[doc = "Pin select for SDA."]
pub mod pselsda;
#[doc = "RXD (r) register accessor: RX data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxd`] module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "RX data register."]
pub mod rxd;
#[doc = "TXD (rw) register accessor: TX data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`] module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "TX data register."]
pub mod txd;
#[doc = "FREQUENCY (rw) register accessor: Two-wire frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`] module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "Two-wire frequency."]
pub mod frequency;
#[doc = "ADDRESS (rw) register accessor: Address used in the two-wire transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`] module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Address used in the two-wire transfer."]
pub mod address;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
