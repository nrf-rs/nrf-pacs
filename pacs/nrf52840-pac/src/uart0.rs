#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_startrx: TasksStartrx,
    tasks_stoprx: TasksStoprx,
    tasks_starttx: TasksStarttx,
    tasks_stoptx: TasksStoptx,
    _reserved4: [u8; 0x0c],
    tasks_suspend: TasksSuspend,
    _reserved5: [u8; 0xe0],
    events_cts: EventsCts,
    events_ncts: EventsNcts,
    events_rxdrdy: EventsRxdrdy,
    _reserved8: [u8; 0x10],
    events_txdrdy: EventsTxdrdy,
    _reserved9: [u8; 0x04],
    events_error: EventsError,
    _reserved10: [u8; 0x1c],
    events_rxto: EventsRxto,
    _reserved11: [u8; 0xb8],
    shorts: Shorts,
    _reserved12: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved14: [u8; 0x0174],
    errorsrc: Errorsrc,
    _reserved15: [u8; 0x7c],
    enable: Enable,
    _reserved16: [u8; 0x04],
    psel: Psel,
    rxd: Rxd,
    txd: Txd,
    _reserved19: [u8; 0x04],
    baudrate: Baudrate,
    _reserved20: [u8; 0x44],
    config: Config,
}
impl RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    #[inline(always)]
    pub const fn tasks_startrx(&self) -> &TasksStartrx {
        &self.tasks_startrx
    }
    #[doc = "0x04 - Stop UART receiver"]
    #[inline(always)]
    pub const fn tasks_stoprx(&self) -> &TasksStoprx {
        &self.tasks_stoprx
    }
    #[doc = "0x08 - Start UART transmitter"]
    #[inline(always)]
    pub const fn tasks_starttx(&self) -> &TasksStarttx {
        &self.tasks_starttx
    }
    #[doc = "0x0c - Stop UART transmitter"]
    #[inline(always)]
    pub const fn tasks_stoptx(&self) -> &TasksStoptx {
        &self.tasks_stoptx
    }
    #[doc = "0x1c - Suspend UART"]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub const fn events_cts(&self) -> &EventsCts {
        &self.events_cts
    }
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub const fn events_ncts(&self) -> &EventsNcts {
        &self.events_ncts
    }
    #[doc = "0x108 - Data received in RXD"]
    #[inline(always)]
    pub const fn events_rxdrdy(&self) -> &EventsRxdrdy {
        &self.events_rxdrdy
    }
    #[doc = "0x11c - Data sent from TXD"]
    #[inline(always)]
    pub const fn events_txdrdy(&self) -> &EventsTxdrdy {
        &self.events_txdrdy
    }
    #[doc = "0x124 - Error detected"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x144 - Receiver timeout"]
    #[inline(always)]
    pub const fn events_rxto(&self) -> &EventsRxto {
        &self.events_rxto
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x480 - Error source"]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x500 - Enable UART"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508..0x518 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x518 - RXD register. Register is cleared on read and the double buffered byte will be moved to RXD if it exists."]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x51c - TXD register"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn baudrate(&self) -> &Baudrate {
        &self.baudrate
    }
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "TASKS_STARTRX (w) register accessor: Start UART receiver\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startrx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startrx`] module"]
#[doc(alias = "TASKS_STARTRX")]
pub type TasksStartrx = crate::Reg<tasks_startrx::TasksStartrxSpec>;
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "TASKS_STOPRX (w) register accessor: Stop UART receiver\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stoprx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stoprx`] module"]
#[doc(alias = "TASKS_STOPRX")]
pub type TasksStoprx = crate::Reg<tasks_stoprx::TasksStoprxSpec>;
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "TASKS_STARTTX (w) register accessor: Start UART transmitter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_starttx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_starttx`] module"]
#[doc(alias = "TASKS_STARTTX")]
pub type TasksStarttx = crate::Reg<tasks_starttx::TasksStarttxSpec>;
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "TASKS_STOPTX (w) register accessor: Stop UART transmitter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stoptx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stoptx`] module"]
#[doc(alias = "TASKS_STOPTX")]
pub type TasksStoptx = crate::Reg<tasks_stoptx::TasksStoptxSpec>;
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend UART\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`] module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend UART"]
pub mod tasks_suspend;
#[doc = "EVENTS_CTS (rw) register accessor: CTS is activated (set low). Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_cts`] module"]
#[doc(alias = "EVENTS_CTS")]
pub type EventsCts = crate::Reg<events_cts::EventsCtsSpec>;
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "EVENTS_NCTS (rw) register accessor: CTS is deactivated (set high). Not Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ncts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ncts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ncts`] module"]
#[doc(alias = "EVENTS_NCTS")]
pub type EventsNcts = crate::Reg<events_ncts::EventsNctsSpec>;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "EVENTS_RXDRDY (rw) register accessor: Data received in RXD\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxdrdy`] module"]
#[doc(alias = "EVENTS_RXDRDY")]
pub type EventsRxdrdy = crate::Reg<events_rxdrdy::EventsRxdrdySpec>;
#[doc = "Data received in RXD"]
pub mod events_rxdrdy;
#[doc = "EVENTS_TXDRDY (rw) register accessor: Data sent from TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txdrdy`] module"]
#[doc(alias = "EVENTS_TXDRDY")]
pub type EventsTxdrdy = crate::Reg<events_txdrdy::EventsTxdrdySpec>;
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "EVENTS_ERROR (rw) register accessor: Error detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`] module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Error detected"]
pub mod events_error;
#[doc = "EVENTS_RXTO (rw) register accessor: Receiver timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxto`] module"]
#[doc(alias = "EVENTS_RXTO")]
pub type EventsRxto = crate::Reg<events_rxto::EventsRxtoSpec>;
#[doc = "Receiver timeout"]
pub mod events_rxto;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "ERRORSRC (rw) register accessor: Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorsrc`] module"]
#[doc(alias = "ERRORSRC")]
pub type Errorsrc = crate::Reg<errorsrc::ErrorsrcSpec>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: Enable UART\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "RXD (r) register accessor: RXD register. Register is cleared on read and the double buffered byte will be moved to RXD if it exists.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@rxd`] module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "RXD register. Register is cleared on read and the double buffered byte will be moved to RXD if it exists."]
pub mod rxd;
#[doc = "TXD (w) register accessor: TXD register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`] module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "TXD register"]
pub mod txd;
#[doc = "BAUDRATE (rw) register accessor: Baud rate. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`baudrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudrate`] module"]
#[doc(alias = "BAUDRATE")]
pub type Baudrate = crate::Reg<baudrate::BaudrateSpec>;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "CONFIG (rw) register accessor: Configuration of parity and hardware flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
