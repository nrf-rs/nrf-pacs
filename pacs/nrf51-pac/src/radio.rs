#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_txen: TasksTxen,
    tasks_rxen: TasksRxen,
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_disable: TasksDisable,
    tasks_rssistart: TasksRssistart,
    tasks_rssistop: TasksRssistop,
    tasks_bcstart: TasksBcstart,
    tasks_bcstop: TasksBcstop,
    _reserved9: [u8; 0xdc],
    events_ready: EventsReady,
    events_address: EventsAddress,
    events_payload: EventsPayload,
    events_end: EventsEnd,
    events_disabled: EventsDisabled,
    events_devmatch: EventsDevmatch,
    events_devmiss: EventsDevmiss,
    events_rssiend: EventsRssiend,
    _reserved17: [u8; 0x08],
    events_bcmatch: EventsBcmatch,
    _reserved18: [u8; 0xd4],
    shorts: Shorts,
    _reserved19: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved21: [u8; 0xf4],
    crcstatus: Crcstatus,
    _reserved22: [u8; 0x04],
    rxmatch: Rxmatch,
    rxcrc: Rxcrc,
    dai: Dai,
    _reserved25: [u8; 0xf0],
    packetptr: Packetptr,
    frequency: Frequency,
    txpower: Txpower,
    mode: Mode,
    pcnf0: Pcnf0,
    pcnf1: Pcnf1,
    base0: Base0,
    base1: Base1,
    prefix0: Prefix0,
    prefix1: Prefix1,
    txaddress: Txaddress,
    rxaddresses: Rxaddresses,
    crccnf: Crccnf,
    crcpoly: Crcpoly,
    crcinit: Crcinit,
    test: Test,
    tifs: Tifs,
    rssisample: Rssisample,
    _reserved43: [u8; 0x04],
    state: State,
    datawhiteiv: Datawhiteiv,
    _reserved45: [u8; 0x08],
    bcc: Bcc,
    _reserved46: [u8; 0x9c],
    dab: [Dab; 8],
    dap: [Dap; 8],
    dacnf: Dacnf,
    _reserved49: [u8; 0xe0],
    override0: Override0,
    override1: Override1,
    override2: Override2,
    override3: Override3,
    override4: Override4,
    _reserved54: [u8; 0x08c4],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Enable radio in TX mode."]
    #[inline(always)]
    pub const fn tasks_txen(&self) -> &TasksTxen {
        &self.tasks_txen
    }
    #[doc = "0x04 - Enable radio in RX mode."]
    #[inline(always)]
    pub const fn tasks_rxen(&self) -> &TasksRxen {
        &self.tasks_rxen
    }
    #[doc = "0x08 - Start radio."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x0c - Stop radio."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x10 - Disable radio."]
    #[inline(always)]
    pub const fn tasks_disable(&self) -> &TasksDisable {
        &self.tasks_disable
    }
    #[doc = "0x14 - Start the RSSI and take one sample of the receive signal strength."]
    #[inline(always)]
    pub const fn tasks_rssistart(&self) -> &TasksRssistart {
        &self.tasks_rssistart
    }
    #[doc = "0x18 - Stop the RSSI measurement."]
    #[inline(always)]
    pub const fn tasks_rssistop(&self) -> &TasksRssistop {
        &self.tasks_rssistop
    }
    #[doc = "0x1c - Start the bit counter."]
    #[inline(always)]
    pub const fn tasks_bcstart(&self) -> &TasksBcstart {
        &self.tasks_bcstart
    }
    #[doc = "0x20 - Stop the bit counter."]
    #[inline(always)]
    pub const fn tasks_bcstop(&self) -> &TasksBcstop {
        &self.tasks_bcstop
    }
    #[doc = "0x100 - Ready event."]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x104 - Address event."]
    #[inline(always)]
    pub const fn events_address(&self) -> &EventsAddress {
        &self.events_address
    }
    #[doc = "0x108 - Payload event."]
    #[inline(always)]
    pub const fn events_payload(&self) -> &EventsPayload {
        &self.events_payload
    }
    #[doc = "0x10c - End event."]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x110 - Disable event."]
    #[inline(always)]
    pub const fn events_disabled(&self) -> &EventsDisabled {
        &self.events_disabled
    }
    #[doc = "0x114 - A device address match occurred on the last received packet."]
    #[inline(always)]
    pub const fn events_devmatch(&self) -> &EventsDevmatch {
        &self.events_devmatch
    }
    #[doc = "0x118 - No device address match occurred on the last received packet."]
    #[inline(always)]
    pub const fn events_devmiss(&self) -> &EventsDevmiss {
        &self.events_devmiss
    }
    #[doc = "0x11c - Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
    #[inline(always)]
    pub const fn events_rssiend(&self) -> &EventsRssiend {
        &self.events_rssiend
    }
    #[doc = "0x128 - Bit counter reached bit count value specified in BCC register."]
    #[inline(always)]
    pub const fn events_bcmatch(&self) -> &EventsBcmatch {
        &self.events_bcmatch
    }
    #[doc = "0x200 - Shortcuts for the radio."]
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
    #[doc = "0x400 - CRC status of received packet."]
    #[inline(always)]
    pub const fn crcstatus(&self) -> &Crcstatus {
        &self.crcstatus
    }
    #[doc = "0x408 - Received address."]
    #[inline(always)]
    pub const fn rxmatch(&self) -> &Rxmatch {
        &self.rxmatch
    }
    #[doc = "0x40c - Received CRC."]
    #[inline(always)]
    pub const fn rxcrc(&self) -> &Rxcrc {
        &self.rxcrc
    }
    #[doc = "0x410 - Device address match index."]
    #[inline(always)]
    pub const fn dai(&self) -> &Dai {
        &self.dai
    }
    #[doc = "0x504 - Packet pointer. Decision point: START task."]
    #[inline(always)]
    pub const fn packetptr(&self) -> &Packetptr {
        &self.packetptr
    }
    #[doc = "0x508 - Frequency."]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x50c - Output power."]
    #[inline(always)]
    pub const fn txpower(&self) -> &Txpower {
        &self.txpower
    }
    #[doc = "0x510 - Data rate and modulation."]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x514 - Packet configuration 0."]
    #[inline(always)]
    pub const fn pcnf0(&self) -> &Pcnf0 {
        &self.pcnf0
    }
    #[doc = "0x518 - Packet configuration 1."]
    #[inline(always)]
    pub const fn pcnf1(&self) -> &Pcnf1 {
        &self.pcnf1
    }
    #[doc = "0x51c - Radio base address 0. Decision point: START task."]
    #[inline(always)]
    pub const fn base0(&self) -> &Base0 {
        &self.base0
    }
    #[doc = "0x520 - Radio base address 1. Decision point: START task."]
    #[inline(always)]
    pub const fn base1(&self) -> &Base1 {
        &self.base1
    }
    #[doc = "0x524 - Prefixes bytes for logical addresses 0 to 3."]
    #[inline(always)]
    pub const fn prefix0(&self) -> &Prefix0 {
        &self.prefix0
    }
    #[doc = "0x528 - Prefixes bytes for logical addresses 4 to 7."]
    #[inline(always)]
    pub const fn prefix1(&self) -> &Prefix1 {
        &self.prefix1
    }
    #[doc = "0x52c - Transmit address select."]
    #[inline(always)]
    pub const fn txaddress(&self) -> &Txaddress {
        &self.txaddress
    }
    #[doc = "0x530 - Receive address select."]
    #[inline(always)]
    pub const fn rxaddresses(&self) -> &Rxaddresses {
        &self.rxaddresses
    }
    #[doc = "0x534 - CRC configuration."]
    #[inline(always)]
    pub const fn crccnf(&self) -> &Crccnf {
        &self.crccnf
    }
    #[doc = "0x538 - CRC polynomial."]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x53c - CRC initial value."]
    #[inline(always)]
    pub const fn crcinit(&self) -> &Crcinit {
        &self.crcinit
    }
    #[doc = "0x540 - Test features enable register."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x544 - Inter Frame Spacing in microseconds."]
    #[inline(always)]
    pub const fn tifs(&self) -> &Tifs {
        &self.tifs
    }
    #[doc = "0x548 - RSSI sample."]
    #[inline(always)]
    pub const fn rssisample(&self) -> &Rssisample {
        &self.rssisample
    }
    #[doc = "0x550 - Current radio state."]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x554 - Data whitening initial value."]
    #[inline(always)]
    pub const fn datawhiteiv(&self) -> &Datawhiteiv {
        &self.datawhiteiv
    }
    #[doc = "0x560 - Bit counter compare."]
    #[inline(always)]
    pub const fn bcc(&self) -> &Bcc {
        &self.bcc
    }
    #[doc = "0x600..0x620 - Device address base segment."]
    #[inline(always)]
    pub const fn dab(&self, n: usize) -> &Dab {
        &self.dab[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x620 - Device address base segment."]
    #[inline(always)]
    pub fn dab_iter(&self) -> impl Iterator<Item = &Dab> {
        self.dab.iter()
    }
    #[doc = "0x620..0x640 - Device address prefix."]
    #[inline(always)]
    pub const fn dap(&self, n: usize) -> &Dap {
        &self.dap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x640 - Device address prefix."]
    #[inline(always)]
    pub fn dap_iter(&self) -> impl Iterator<Item = &Dap> {
        self.dap.iter()
    }
    #[doc = "0x640 - Device address match configuration."]
    #[inline(always)]
    pub const fn dacnf(&self) -> &Dacnf {
        &self.dacnf
    }
    #[doc = "0x724 - Trim value override register 0."]
    #[inline(always)]
    pub const fn override0(&self) -> &Override0 {
        &self.override0
    }
    #[doc = "0x728 - Trim value override register 1."]
    #[inline(always)]
    pub const fn override1(&self) -> &Override1 {
        &self.override1
    }
    #[doc = "0x72c - Trim value override register 2."]
    #[inline(always)]
    pub const fn override2(&self) -> &Override2 {
        &self.override2
    }
    #[doc = "0x730 - Trim value override register 3."]
    #[inline(always)]
    pub const fn override3(&self) -> &Override3 {
        &self.override3
    }
    #[doc = "0x734 - Trim value override register 4."]
    #[inline(always)]
    pub const fn override4(&self) -> &Override4 {
        &self.override4
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_TXEN (w) register accessor: Enable radio in TX mode.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_txen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_txen`] module"]
#[doc(alias = "TASKS_TXEN")]
pub type TasksTxen = crate::Reg<tasks_txen::TasksTxenSpec>;
#[doc = "Enable radio in TX mode."]
pub mod tasks_txen;
#[doc = "TASKS_RXEN (w) register accessor: Enable radio in RX mode.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rxen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rxen`] module"]
#[doc(alias = "TASKS_RXEN")]
pub type TasksRxen = crate::Reg<tasks_rxen::TasksRxenSpec>;
#[doc = "Enable radio in RX mode."]
pub mod tasks_rxen;
#[doc = "TASKS_START (w) register accessor: Start radio.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start radio."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop radio.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop radio."]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE (w) register accessor: Disable radio.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_disable`] module"]
#[doc(alias = "TASKS_DISABLE")]
pub type TasksDisable = crate::Reg<tasks_disable::TasksDisableSpec>;
#[doc = "Disable radio."]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART (w) register accessor: Start the RSSI and take one sample of the receive signal strength.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rssistart`] module"]
#[doc(alias = "TASKS_RSSISTART")]
pub type TasksRssistart = crate::Reg<tasks_rssistart::TasksRssistartSpec>;
#[doc = "Start the RSSI and take one sample of the receive signal strength."]
pub mod tasks_rssistart;
#[doc = "TASKS_RSSISTOP (w) register accessor: Stop the RSSI measurement.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rssistop`] module"]
#[doc(alias = "TASKS_RSSISTOP")]
pub type TasksRssistop = crate::Reg<tasks_rssistop::TasksRssistopSpec>;
#[doc = "Stop the RSSI measurement."]
pub mod tasks_rssistop;
#[doc = "TASKS_BCSTART (w) register accessor: Start the bit counter.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstart`] module"]
#[doc(alias = "TASKS_BCSTART")]
pub type TasksBcstart = crate::Reg<tasks_bcstart::TasksBcstartSpec>;
#[doc = "Start the bit counter."]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP (w) register accessor: Stop the bit counter.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstop`] module"]
#[doc(alias = "TASKS_BCSTOP")]
pub type TasksBcstop = crate::Reg<tasks_bcstop::TasksBcstopSpec>;
#[doc = "Stop the bit counter."]
pub mod tasks_bcstop;
#[doc = "EVENTS_READY (rw) register accessor: Ready event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`] module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "Ready event."]
pub mod events_ready;
#[doc = "EVENTS_ADDRESS (rw) register accessor: Address event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_address`] module"]
#[doc(alias = "EVENTS_ADDRESS")]
pub type EventsAddress = crate::Reg<events_address::EventsAddressSpec>;
#[doc = "Address event."]
pub mod events_address;
#[doc = "EVENTS_PAYLOAD (rw) register accessor: Payload event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_payload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_payload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_payload`] module"]
#[doc(alias = "EVENTS_PAYLOAD")]
pub type EventsPayload = crate::Reg<events_payload::EventsPayloadSpec>;
#[doc = "Payload event."]
pub mod events_payload;
#[doc = "EVENTS_END (rw) register accessor: End event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`] module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "End event."]
pub mod events_end;
#[doc = "EVENTS_DISABLED (rw) register accessor: Disable event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_disabled`] module"]
#[doc(alias = "EVENTS_DISABLED")]
pub type EventsDisabled = crate::Reg<events_disabled::EventsDisabledSpec>;
#[doc = "Disable event."]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH (rw) register accessor: A device address match occurred on the last received packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmatch`] module"]
#[doc(alias = "EVENTS_DEVMATCH")]
pub type EventsDevmatch = crate::Reg<events_devmatch::EventsDevmatchSpec>;
#[doc = "A device address match occurred on the last received packet."]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS (rw) register accessor: No device address match occurred on the last received packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmiss`] module"]
#[doc(alias = "EVENTS_DEVMISS")]
pub type EventsDevmiss = crate::Reg<events_devmiss::EventsDevmissSpec>;
#[doc = "No device address match occurred on the last received packet."]
pub mod events_devmiss;
#[doc = "EVENTS_RSSIEND (rw) register accessor: Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rssiend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rssiend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rssiend`] module"]
#[doc(alias = "EVENTS_RSSIEND")]
pub type EventsRssiend = crate::Reg<events_rssiend::EventsRssiendSpec>;
#[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
pub mod events_rssiend;
#[doc = "EVENTS_BCMATCH (rw) register accessor: Bit counter reached bit count value specified in BCC register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bcmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bcmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_bcmatch`] module"]
#[doc(alias = "EVENTS_BCMATCH")]
pub type EventsBcmatch = crate::Reg<events_bcmatch::EventsBcmatchSpec>;
#[doc = "Bit counter reached bit count value specified in BCC register."]
pub mod events_bcmatch;
#[doc = "SHORTS (rw) register accessor: Shortcuts for the radio.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts for the radio."]
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
#[doc = "CRCSTATUS (r) register accessor: CRC status of received packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcstatus`] module"]
#[doc(alias = "CRCSTATUS")]
pub type Crcstatus = crate::Reg<crcstatus::CrcstatusSpec>;
#[doc = "CRC status of received packet."]
pub mod crcstatus;
#[doc = "RXMATCH (r) register accessor: Received address.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmatch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmatch`] module"]
#[doc(alias = "RXMATCH")]
pub type Rxmatch = crate::Reg<rxmatch::RxmatchSpec>;
#[doc = "Received address."]
pub mod rxmatch;
#[doc = "RXCRC (r) register accessor: Received CRC.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`] module"]
#[doc(alias = "RXCRC")]
pub type Rxcrc = crate::Reg<rxcrc::RxcrcSpec>;
#[doc = "Received CRC."]
pub mod rxcrc;
#[doc = "DAI (r) register accessor: Device address match index.\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dai`] module"]
#[doc(alias = "DAI")]
pub type Dai = crate::Reg<dai::DaiSpec>;
#[doc = "Device address match index."]
pub mod dai;
#[doc = "PACKETPTR (rw) register accessor: Packet pointer. Decision point: START task.\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packetptr`] module"]
#[doc(alias = "PACKETPTR")]
pub type Packetptr = crate::Reg<packetptr::PacketptrSpec>;
#[doc = "Packet pointer. Decision point: START task."]
pub mod packetptr;
#[doc = "FREQUENCY (rw) register accessor: Frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`] module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "Frequency."]
pub mod frequency;
#[doc = "TXPOWER (rw) register accessor: Output power.\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpower`] module"]
#[doc(alias = "TXPOWER")]
pub type Txpower = crate::Reg<txpower::TxpowerSpec>;
#[doc = "Output power."]
pub mod txpower;
#[doc = "MODE (rw) register accessor: Data rate and modulation.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Data rate and modulation."]
pub mod mode;
#[doc = "PCNF0 (rw) register accessor: Packet configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf0`] module"]
#[doc(alias = "PCNF0")]
pub type Pcnf0 = crate::Reg<pcnf0::Pcnf0Spec>;
#[doc = "Packet configuration 0."]
pub mod pcnf0;
#[doc = "PCNF1 (rw) register accessor: Packet configuration 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf1`] module"]
#[doc(alias = "PCNF1")]
pub type Pcnf1 = crate::Reg<pcnf1::Pcnf1Spec>;
#[doc = "Packet configuration 1."]
pub mod pcnf1;
#[doc = "BASE0 (rw) register accessor: Radio base address 0. Decision point: START task.\n\nYou can [`read`](crate::Reg::read) this register and get [`base0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base0`] module"]
#[doc(alias = "BASE0")]
pub type Base0 = crate::Reg<base0::Base0Spec>;
#[doc = "Radio base address 0. Decision point: START task."]
pub mod base0;
#[doc = "BASE1 (rw) register accessor: Radio base address 1. Decision point: START task.\n\nYou can [`read`](crate::Reg::read) this register and get [`base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base1`] module"]
#[doc(alias = "BASE1")]
pub type Base1 = crate::Reg<base1::Base1Spec>;
#[doc = "Radio base address 1. Decision point: START task."]
pub mod base1;
#[doc = "PREFIX0 (rw) register accessor: Prefixes bytes for logical addresses 0 to 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix0`] module"]
#[doc(alias = "PREFIX0")]
pub type Prefix0 = crate::Reg<prefix0::Prefix0Spec>;
#[doc = "Prefixes bytes for logical addresses 0 to 3."]
pub mod prefix0;
#[doc = "PREFIX1 (rw) register accessor: Prefixes bytes for logical addresses 4 to 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix1`] module"]
#[doc(alias = "PREFIX1")]
pub type Prefix1 = crate::Reg<prefix1::Prefix1Spec>;
#[doc = "Prefixes bytes for logical addresses 4 to 7."]
pub mod prefix1;
#[doc = "TXADDRESS (rw) register accessor: Transmit address select.\n\nYou can [`read`](crate::Reg::read) this register and get [`txaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txaddress`] module"]
#[doc(alias = "TXADDRESS")]
pub type Txaddress = crate::Reg<txaddress::TxaddressSpec>;
#[doc = "Transmit address select."]
pub mod txaddress;
#[doc = "RXADDRESSES (rw) register accessor: Receive address select.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddresses::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddresses::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxaddresses`] module"]
#[doc(alias = "RXADDRESSES")]
pub type Rxaddresses = crate::Reg<rxaddresses::RxaddressesSpec>;
#[doc = "Receive address select."]
pub mod rxaddresses;
#[doc = "CRCCNF (rw) register accessor: CRC configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crccnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccnf`] module"]
#[doc(alias = "CRCCNF")]
pub type Crccnf = crate::Reg<crccnf::CrccnfSpec>;
#[doc = "CRC configuration."]
pub mod crccnf;
#[doc = "CRCPOLY (rw) register accessor: CRC polynomial.\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`] module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC polynomial."]
pub mod crcpoly;
#[doc = "CRCINIT (rw) register accessor: CRC initial value.\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinit`] module"]
#[doc(alias = "CRCINIT")]
pub type Crcinit = crate::Reg<crcinit::CrcinitSpec>;
#[doc = "CRC initial value."]
pub mod crcinit;
#[doc = "TEST (rw) register accessor: Test features enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test features enable register."]
pub mod test;
#[doc = "TIFS (rw) register accessor: Inter Frame Spacing in microseconds.\n\nYou can [`read`](crate::Reg::read) this register and get [`tifs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifs`] module"]
#[doc(alias = "TIFS")]
pub type Tifs = crate::Reg<tifs::TifsSpec>;
#[doc = "Inter Frame Spacing in microseconds."]
pub mod tifs;
#[doc = "RSSISAMPLE (r) register accessor: RSSI sample.\n\nYou can [`read`](crate::Reg::read) this register and get [`rssisample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssisample`] module"]
#[doc(alias = "RSSISAMPLE")]
pub type Rssisample = crate::Reg<rssisample::RssisampleSpec>;
#[doc = "RSSI sample."]
pub mod rssisample;
#[doc = "STATE (r) register accessor: Current radio state.\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Current radio state."]
pub mod state;
#[doc = "DATAWHITEIV (rw) register accessor: Data whitening initial value.\n\nYou can [`read`](crate::Reg::read) this register and get [`datawhiteiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datawhiteiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datawhiteiv`] module"]
#[doc(alias = "DATAWHITEIV")]
pub type Datawhiteiv = crate::Reg<datawhiteiv::DatawhiteivSpec>;
#[doc = "Data whitening initial value."]
pub mod datawhiteiv;
#[doc = "BCC (rw) register accessor: Bit counter compare.\n\nYou can [`read`](crate::Reg::read) this register and get [`bcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcc`] module"]
#[doc(alias = "BCC")]
pub type Bcc = crate::Reg<bcc::BccSpec>;
#[doc = "Bit counter compare."]
pub mod bcc;
#[doc = "DAB (rw) register accessor: Device address base segment.\n\nYou can [`read`](crate::Reg::read) this register and get [`dab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dab`] module"]
#[doc(alias = "DAB")]
pub type Dab = crate::Reg<dab::DabSpec>;
#[doc = "Device address base segment."]
pub mod dab;
#[doc = "DAP (rw) register accessor: Device address prefix.\n\nYou can [`read`](crate::Reg::read) this register and get [`dap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dap`] module"]
#[doc(alias = "DAP")]
pub type Dap = crate::Reg<dap::DapSpec>;
#[doc = "Device address prefix."]
pub mod dap;
#[doc = "DACNF (rw) register accessor: Device address match configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`dacnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacnf`] module"]
#[doc(alias = "DACNF")]
pub type Dacnf = crate::Reg<dacnf::DacnfSpec>;
#[doc = "Device address match configuration."]
pub mod dacnf;
#[doc = "OVERRIDE0 (rw) register accessor: Trim value override register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`override0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@override0`] module"]
#[doc(alias = "OVERRIDE0")]
pub type Override0 = crate::Reg<override0::Override0Spec>;
#[doc = "Trim value override register 0."]
pub mod override0;
#[doc = "OVERRIDE1 (rw) register accessor: Trim value override register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`override1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@override1`] module"]
#[doc(alias = "OVERRIDE1")]
pub type Override1 = crate::Reg<override1::Override1Spec>;
#[doc = "Trim value override register 1."]
pub mod override1;
#[doc = "OVERRIDE2 (rw) register accessor: Trim value override register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`override2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@override2`] module"]
#[doc(alias = "OVERRIDE2")]
pub type Override2 = crate::Reg<override2::Override2Spec>;
#[doc = "Trim value override register 2."]
pub mod override2;
#[doc = "OVERRIDE3 (rw) register accessor: Trim value override register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`override3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@override3`] module"]
#[doc(alias = "OVERRIDE3")]
pub type Override3 = crate::Reg<override3::Override3Spec>;
#[doc = "Trim value override register 3."]
pub mod override3;
#[doc = "OVERRIDE4 (rw) register accessor: Trim value override register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`override4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@override4`] module"]
#[doc(alias = "OVERRIDE4")]
pub type Override4 = crate::Reg<override4::Override4Spec>;
#[doc = "Trim value override register 4."]
pub mod override4;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
