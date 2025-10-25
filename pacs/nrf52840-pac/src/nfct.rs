#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_activate: TasksActivate,
    tasks_disable: TasksDisable,
    tasks_sense: TasksSense,
    tasks_starttx: TasksStarttx,
    _reserved4: [u8; 0x0c],
    tasks_enablerxdata: TasksEnablerxdata,
    _reserved5: [u8; 0x04],
    tasks_goidle: TasksGoidle,
    tasks_gosleep: TasksGosleep,
    _reserved7: [u8; 0xd4],
    events_ready: EventsReady,
    events_fielddetected: EventsFielddetected,
    events_fieldlost: EventsFieldlost,
    events_txframestart: EventsTxframestart,
    events_txframeend: EventsTxframeend,
    events_rxframestart: EventsRxframestart,
    events_rxframeend: EventsRxframeend,
    events_error: EventsError,
    _reserved15: [u8; 0x08],
    events_rxerror: EventsRxerror,
    events_endrx: EventsEndrx,
    events_endtx: EventsEndtx,
    _reserved18: [u8; 0x04],
    events_autocolresstarted: EventsAutocolresstarted,
    _reserved19: [u8; 0x0c],
    events_collision: EventsCollision,
    events_selected: EventsSelected,
    events_started: EventsStarted,
    _reserved22: [u8; 0xac],
    shorts: Shorts,
    _reserved23: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved26: [u8; 0xf8],
    errorstatus: Errorstatus,
    _reserved27: [u8; 0x04],
    framestatus: Framestatus,
    nfctagstate: Nfctagstate,
    _reserved29: [u8; 0x0c],
    sleepstate: Sleepstate,
    _reserved30: [u8; 0x18],
    fieldpresent: Fieldpresent,
    _reserved31: [u8; 0xc4],
    framedelaymin: Framedelaymin,
    framedelaymax: Framedelaymax,
    framedelaymode: Framedelaymode,
    packetptr: Packetptr,
    maxlen: Maxlen,
    txd: Txd,
    rxd: Rxd,
    _reserved38: [u8; 0x68],
    nfcid1_last: Nfcid1Last,
    nfcid1_2nd_last: Nfcid1_2ndLast,
    nfcid1_3rd_last: Nfcid1_3rdLast,
    autocolresconfig: Autocolresconfig,
    sensres: Sensres,
    selres: Selres,
}
impl RegisterBlock {
    #[doc = "0x00 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub const fn tasks_activate(&self) -> &TasksActivate {
        &self.tasks_activate
    }
    #[doc = "0x04 - Disable NFCT peripheral"]
    #[inline(always)]
    pub const fn tasks_disable(&self) -> &TasksDisable {
        &self.tasks_disable
    }
    #[doc = "0x08 - Enable NFC sense field mode, change state to sense mode"]
    #[inline(always)]
    pub const fn tasks_sense(&self) -> &TasksSense {
        &self.tasks_sense
    }
    #[doc = "0x0c - Start transmission of an outgoing frame, change state to transmit"]
    #[inline(always)]
    pub const fn tasks_starttx(&self) -> &TasksStarttx {
        &self.tasks_starttx
    }
    #[doc = "0x1c - Initializes the EasyDMA for receive."]
    #[inline(always)]
    pub const fn tasks_enablerxdata(&self) -> &TasksEnablerxdata {
        &self.tasks_enablerxdata
    }
    #[doc = "0x24 - Force state machine to IDLE state"]
    #[inline(always)]
    pub const fn tasks_goidle(&self) -> &TasksGoidle {
        &self.tasks_goidle
    }
    #[doc = "0x28 - Force state machine to SLEEP_A state"]
    #[inline(always)]
    pub const fn tasks_gosleep(&self) -> &TasksGosleep {
        &self.tasks_gosleep
    }
    #[doc = "0x100 - The NFCT peripheral is ready to receive and send frames"]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x104 - Remote NFC field detected"]
    #[inline(always)]
    pub const fn events_fielddetected(&self) -> &EventsFielddetected {
        &self.events_fielddetected
    }
    #[doc = "0x108 - Remote NFC field lost"]
    #[inline(always)]
    pub const fn events_fieldlost(&self) -> &EventsFieldlost {
        &self.events_fieldlost
    }
    #[doc = "0x10c - Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub const fn events_txframestart(&self) -> &EventsTxframestart {
        &self.events_txframestart
    }
    #[doc = "0x110 - Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub const fn events_txframeend(&self) -> &EventsTxframeend {
        &self.events_txframeend
    }
    #[doc = "0x114 - Marks the end of the first symbol of a received frame"]
    #[inline(always)]
    pub const fn events_rxframestart(&self) -> &EventsRxframestart {
        &self.events_rxframestart
    }
    #[doc = "0x118 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    #[inline(always)]
    pub const fn events_rxframeend(&self) -> &EventsRxframeend {
        &self.events_rxframeend
    }
    #[doc = "0x11c - NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x128 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    #[inline(always)]
    pub const fn events_rxerror(&self) -> &EventsRxerror {
        &self.events_rxerror
    }
    #[doc = "0x12c - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    #[inline(always)]
    pub const fn events_endrx(&self) -> &EventsEndrx {
        &self.events_endrx
    }
    #[doc = "0x130 - Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    #[inline(always)]
    pub const fn events_endtx(&self) -> &EventsEndtx {
        &self.events_endtx
    }
    #[doc = "0x138 - Auto collision resolution process has started"]
    #[inline(always)]
    pub const fn events_autocolresstarted(&self) -> &EventsAutocolresstarted {
        &self.events_autocolresstarted
    }
    #[doc = "0x148 - NFC auto collision resolution error reported."]
    #[inline(always)]
    pub const fn events_collision(&self) -> &EventsCollision {
        &self.events_collision
    }
    #[doc = "0x14c - NFC auto collision resolution successfully completed"]
    #[inline(always)]
    pub const fn events_selected(&self) -> &EventsSelected {
        &self.events_selected
    }
    #[doc = "0x150 - EasyDMA is ready to receive or send frames."]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x404 - NFC Error Status register"]
    #[inline(always)]
    pub const fn errorstatus(&self) -> &Errorstatus {
        &self.errorstatus
    }
    #[doc = "0x40c - Unspecified"]
    #[inline(always)]
    pub const fn framestatus(&self) -> &Framestatus {
        &self.framestatus
    }
    #[doc = "0x410 - NfcTag state register"]
    #[inline(always)]
    pub const fn nfctagstate(&self) -> &Nfctagstate {
        &self.nfctagstate
    }
    #[doc = "0x420 - Sleep state during automatic collision resolution"]
    #[inline(always)]
    pub const fn sleepstate(&self) -> &Sleepstate {
        &self.sleepstate
    }
    #[doc = "0x43c - Indicates the presence or not of a valid field"]
    #[inline(always)]
    pub const fn fieldpresent(&self) -> &Fieldpresent {
        &self.fieldpresent
    }
    #[doc = "0x504 - Minimum frame delay"]
    #[inline(always)]
    pub const fn framedelaymin(&self) -> &Framedelaymin {
        &self.framedelaymin
    }
    #[doc = "0x508 - Maximum frame delay"]
    #[inline(always)]
    pub const fn framedelaymax(&self) -> &Framedelaymax {
        &self.framedelaymax
    }
    #[doc = "0x50c - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub const fn framedelaymode(&self) -> &Framedelaymode {
        &self.framedelaymode
    }
    #[doc = "0x510 - Packet pointer for TXD and RXD data storage in Data RAM"]
    #[inline(always)]
    pub const fn packetptr(&self) -> &Packetptr {
        &self.packetptr
    }
    #[doc = "0x514 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub const fn maxlen(&self) -> &Maxlen {
        &self.maxlen
    }
    #[doc = "0x518..0x520 - Unspecified"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x520..0x528 - Unspecified"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x590 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_last(&self) -> &Nfcid1Last {
        &self.nfcid1_last
    }
    #[doc = "0x594 - Second last NFCID1 part (7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_2nd_last(&self) -> &Nfcid1_2ndLast {
        &self.nfcid1_2nd_last
    }
    #[doc = "0x598 - Third last NFCID1 part (10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_3rd_last(&self) -> &Nfcid1_3rdLast {
        &self.nfcid1_3rd_last
    }
    #[doc = "0x59c - Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
    #[inline(always)]
    pub const fn autocolresconfig(&self) -> &Autocolresconfig {
        &self.autocolresconfig
    }
    #[doc = "0x5a0 - NFC-A SENS_RES auto-response settings"]
    #[inline(always)]
    pub const fn sensres(&self) -> &Sensres {
        &self.sensres
    }
    #[doc = "0x5a4 - NFC-A SEL_RES auto-response settings"]
    #[inline(always)]
    pub const fn selres(&self) -> &Selres {
        &self.selres
    }
}
#[doc = "TASKS_ACTIVATE (w) register accessor: Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_activate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_activate`] module"]
#[doc(alias = "TASKS_ACTIVATE")]
pub type TasksActivate = crate::Reg<tasks_activate::TasksActivateSpec>;
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub mod tasks_activate;
#[doc = "TASKS_DISABLE (w) register accessor: Disable NFCT peripheral\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_disable`] module"]
#[doc(alias = "TASKS_DISABLE")]
pub type TasksDisable = crate::Reg<tasks_disable::TasksDisableSpec>;
#[doc = "Disable NFCT peripheral"]
pub mod tasks_disable;
#[doc = "TASKS_SENSE (w) register accessor: Enable NFC sense field mode, change state to sense mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sense::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_sense`] module"]
#[doc(alias = "TASKS_SENSE")]
pub type TasksSense = crate::Reg<tasks_sense::TasksSenseSpec>;
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub mod tasks_sense;
#[doc = "TASKS_STARTTX (w) register accessor: Start transmission of an outgoing frame, change state to transmit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_starttx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_starttx`] module"]
#[doc(alias = "TASKS_STARTTX")]
pub type TasksStarttx = crate::Reg<tasks_starttx::TasksStarttxSpec>;
#[doc = "Start transmission of an outgoing frame, change state to transmit"]
pub mod tasks_starttx;
#[doc = "TASKS_ENABLERXDATA (w) register accessor: Initializes the EasyDMA for receive.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_enablerxdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_enablerxdata`] module"]
#[doc(alias = "TASKS_ENABLERXDATA")]
pub type TasksEnablerxdata = crate::Reg<tasks_enablerxdata::TasksEnablerxdataSpec>;
#[doc = "Initializes the EasyDMA for receive."]
pub mod tasks_enablerxdata;
#[doc = "TASKS_GOIDLE (w) register accessor: Force state machine to IDLE state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_goidle::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_goidle`] module"]
#[doc(alias = "TASKS_GOIDLE")]
pub type TasksGoidle = crate::Reg<tasks_goidle::TasksGoidleSpec>;
#[doc = "Force state machine to IDLE state"]
pub mod tasks_goidle;
#[doc = "TASKS_GOSLEEP (w) register accessor: Force state machine to SLEEP_A state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_gosleep::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_gosleep`] module"]
#[doc(alias = "TASKS_GOSLEEP")]
pub type TasksGosleep = crate::Reg<tasks_gosleep::TasksGosleepSpec>;
#[doc = "Force state machine to SLEEP_A state"]
pub mod tasks_gosleep;
#[doc = "EVENTS_READY (rw) register accessor: The NFCT peripheral is ready to receive and send frames\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`] module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "The NFCT peripheral is ready to receive and send frames"]
pub mod events_ready;
#[doc = "EVENTS_FIELDDETECTED (rw) register accessor: Remote NFC field detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fielddetected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fielddetected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fielddetected`] module"]
#[doc(alias = "EVENTS_FIELDDETECTED")]
pub type EventsFielddetected = crate::Reg<events_fielddetected::EventsFielddetectedSpec>;
#[doc = "Remote NFC field detected"]
pub mod events_fielddetected;
#[doc = "EVENTS_FIELDLOST (rw) register accessor: Remote NFC field lost\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fieldlost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fieldlost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fieldlost`] module"]
#[doc(alias = "EVENTS_FIELDLOST")]
pub type EventsFieldlost = crate::Reg<events_fieldlost::EventsFieldlostSpec>;
#[doc = "Remote NFC field lost"]
pub mod events_fieldlost;
#[doc = "EVENTS_TXFRAMESTART (rw) register accessor: Marks the start of the first symbol of a transmitted frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txframestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txframestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txframestart`] module"]
#[doc(alias = "EVENTS_TXFRAMESTART")]
pub type EventsTxframestart = crate::Reg<events_txframestart::EventsTxframestartSpec>;
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub mod events_txframestart;
#[doc = "EVENTS_TXFRAMEEND (rw) register accessor: Marks the end of the last transmitted on-air symbol of a frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txframeend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txframeend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txframeend`] module"]
#[doc(alias = "EVENTS_TXFRAMEEND")]
pub type EventsTxframeend = crate::Reg<events_txframeend::EventsTxframeendSpec>;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub mod events_txframeend;
#[doc = "EVENTS_RXFRAMESTART (rw) register accessor: Marks the end of the first symbol of a received frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxframestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxframestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxframestart`] module"]
#[doc(alias = "EVENTS_RXFRAMESTART")]
pub type EventsRxframestart = crate::Reg<events_rxframestart::EventsRxframestartSpec>;
#[doc = "Marks the end of the first symbol of a received frame"]
pub mod events_rxframestart;
#[doc = "EVENTS_RXFRAMEEND (rw) register accessor: Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxframeend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxframeend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxframeend`] module"]
#[doc(alias = "EVENTS_RXFRAMEEND")]
pub type EventsRxframeend = crate::Reg<events_rxframeend::EventsRxframeendSpec>;
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub mod events_rxframeend;
#[doc = "EVENTS_ERROR (rw) register accessor: NFC error reported. The ERRORSTATUS register contains details on the source of the error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`] module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub mod events_error;
#[doc = "EVENTS_RXERROR (rw) register accessor: NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxerror`] module"]
#[doc(alias = "EVENTS_RXERROR")]
pub type EventsRxerror = crate::Reg<events_rxerror::EventsRxerrorSpec>;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub mod events_rxerror;
#[doc = "EVENTS_ENDRX (rw) register accessor: RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endrx`] module"]
#[doc(alias = "EVENTS_ENDRX")]
pub type EventsEndrx = crate::Reg<events_endrx::EventsEndrxSpec>;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub mod events_endrx;
#[doc = "EVENTS_ENDTX (rw) register accessor: Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endtx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endtx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endtx`] module"]
#[doc(alias = "EVENTS_ENDTX")]
pub type EventsEndtx = crate::Reg<events_endtx::EventsEndtxSpec>;
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub mod events_endtx;
#[doc = "EVENTS_AUTOCOLRESSTARTED (rw) register accessor: Auto collision resolution process has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_autocolresstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_autocolresstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_autocolresstarted`] module"]
#[doc(alias = "EVENTS_AUTOCOLRESSTARTED")]
pub type EventsAutocolresstarted =
    crate::Reg<events_autocolresstarted::EventsAutocolresstartedSpec>;
#[doc = "Auto collision resolution process has started"]
pub mod events_autocolresstarted;
#[doc = "EVENTS_COLLISION (rw) register accessor: NFC auto collision resolution error reported.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_collision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_collision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_collision`] module"]
#[doc(alias = "EVENTS_COLLISION")]
pub type EventsCollision = crate::Reg<events_collision::EventsCollisionSpec>;
#[doc = "NFC auto collision resolution error reported."]
pub mod events_collision;
#[doc = "EVENTS_SELECTED (rw) register accessor: NFC auto collision resolution successfully completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_selected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_selected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_selected`] module"]
#[doc(alias = "EVENTS_SELECTED")]
pub type EventsSelected = crate::Reg<events_selected::EventsSelectedSpec>;
#[doc = "NFC auto collision resolution successfully completed"]
pub mod events_selected;
#[doc = "EVENTS_STARTED (rw) register accessor: EasyDMA is ready to receive or send frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`] module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "EasyDMA is ready to receive or send frames."]
pub mod events_started;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "ERRORSTATUS (rw) register accessor: NFC Error Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorstatus`] module"]
#[doc(alias = "ERRORSTATUS")]
pub type Errorstatus = crate::Reg<errorstatus::ErrorstatusSpec>;
#[doc = "NFC Error Status register"]
pub mod errorstatus;
#[doc = "Unspecified"]
pub use self::framestatus::Framestatus;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod framestatus;
#[doc = "NFCTAGSTATE (r) register accessor: NfcTag state register\n\nYou can [`read`](crate::Reg::read) this register and get [`nfctagstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfctagstate`] module"]
#[doc(alias = "NFCTAGSTATE")]
pub type Nfctagstate = crate::Reg<nfctagstate::NfctagstateSpec>;
#[doc = "NfcTag state register"]
pub mod nfctagstate;
#[doc = "SLEEPSTATE (r) register accessor: Sleep state during automatic collision resolution\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepstate`] module"]
#[doc(alias = "SLEEPSTATE")]
pub type Sleepstate = crate::Reg<sleepstate::SleepstateSpec>;
#[doc = "Sleep state during automatic collision resolution"]
pub mod sleepstate;
#[doc = "FIELDPRESENT (r) register accessor: Indicates the presence or not of a valid field\n\nYou can [`read`](crate::Reg::read) this register and get [`fieldpresent::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fieldpresent`] module"]
#[doc(alias = "FIELDPRESENT")]
pub type Fieldpresent = crate::Reg<fieldpresent::FieldpresentSpec>;
#[doc = "Indicates the presence or not of a valid field"]
pub mod fieldpresent;
#[doc = "FRAMEDELAYMIN (rw) register accessor: Minimum frame delay\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framedelaymin`] module"]
#[doc(alias = "FRAMEDELAYMIN")]
pub type Framedelaymin = crate::Reg<framedelaymin::FramedelayminSpec>;
#[doc = "Minimum frame delay"]
pub mod framedelaymin;
#[doc = "FRAMEDELAYMAX (rw) register accessor: Maximum frame delay\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framedelaymax`] module"]
#[doc(alias = "FRAMEDELAYMAX")]
pub type Framedelaymax = crate::Reg<framedelaymax::FramedelaymaxSpec>;
#[doc = "Maximum frame delay"]
pub mod framedelaymax;
#[doc = "FRAMEDELAYMODE (rw) register accessor: Configuration register for the Frame Delay Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framedelaymode`] module"]
#[doc(alias = "FRAMEDELAYMODE")]
pub type Framedelaymode = crate::Reg<framedelaymode::FramedelaymodeSpec>;
#[doc = "Configuration register for the Frame Delay Timer"]
pub mod framedelaymode;
#[doc = "PACKETPTR (rw) register accessor: Packet pointer for TXD and RXD data storage in Data RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packetptr`] module"]
#[doc(alias = "PACKETPTR")]
pub type Packetptr = crate::Reg<packetptr::PacketptrSpec>;
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub mod packetptr;
#[doc = "MAXLEN (rw) register accessor: Size of the RAM buffer allocated to TXD and RXD data storage each\n\nYou can [`read`](crate::Reg::read) this register and get [`maxlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxlen`] module"]
#[doc(alias = "MAXLEN")]
pub type Maxlen = crate::Reg<maxlen::MaxlenSpec>;
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub mod maxlen;
#[doc = "Unspecified"]
pub use self::txd::Txd;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Unspecified"]
pub use self::rxd::Rxd;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "NFCID1_LAST (rw) register accessor: Last NFCID1 part (4, 7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcid1_last`] module"]
#[doc(alias = "NFCID1_LAST")]
pub type Nfcid1Last = crate::Reg<nfcid1_last::Nfcid1LastSpec>;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod nfcid1_last;
#[doc = "NFCID1_2ND_LAST (rw) register accessor: Second last NFCID1 part (7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_2nd_last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_2nd_last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcid1_2nd_last`] module"]
#[doc(alias = "NFCID1_2ND_LAST")]
pub type Nfcid1_2ndLast = crate::Reg<nfcid1_2nd_last::Nfcid1_2ndLastSpec>;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod nfcid1_2nd_last;
#[doc = "NFCID1_3RD_LAST (rw) register accessor: Third last NFCID1 part (10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_3rd_last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_3rd_last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcid1_3rd_last`] module"]
#[doc(alias = "NFCID1_3RD_LAST")]
pub type Nfcid1_3rdLast = crate::Reg<nfcid1_3rd_last::Nfcid1_3rdLastSpec>;
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod nfcid1_3rd_last;
#[doc = "AUTOCOLRESCONFIG (rw) register accessor: Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocolresconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocolresconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocolresconfig`] module"]
#[doc(alias = "AUTOCOLRESCONFIG")]
pub type Autocolresconfig = crate::Reg<autocolresconfig::AutocolresconfigSpec>;
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
pub mod autocolresconfig;
#[doc = "SENSRES (rw) register accessor: NFC-A SENS_RES auto-response settings\n\nYou can [`read`](crate::Reg::read) this register and get [`sensres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sensres`] module"]
#[doc(alias = "SENSRES")]
pub type Sensres = crate::Reg<sensres::SensresSpec>;
#[doc = "NFC-A SENS_RES auto-response settings"]
pub mod sensres;
#[doc = "SELRES (rw) register accessor: NFC-A SEL_RES auto-response settings\n\nYou can [`read`](crate::Reg::read) this register and get [`selres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selres`] module"]
#[doc(alias = "SELRES")]
pub type Selres = crate::Reg<selres::SelresSpec>;
#[doc = "NFC-A SEL_RES auto-response settings"]
pub mod selres;
