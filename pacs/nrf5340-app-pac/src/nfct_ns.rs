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
    _reserved7: [u8; 0x54],
    subscribe_activate: SubscribeActivate,
    subscribe_disable: SubscribeDisable,
    subscribe_sense: SubscribeSense,
    subscribe_starttx: SubscribeStarttx,
    _reserved11: [u8; 0x0c],
    subscribe_enablerxdata: SubscribeEnablerxdata,
    _reserved12: [u8; 0x04],
    subscribe_goidle: SubscribeGoidle,
    subscribe_gosleep: SubscribeGosleep,
    _reserved14: [u8; 0x54],
    events_ready: EventsReady,
    events_fielddetected: EventsFielddetected,
    events_fieldlost: EventsFieldlost,
    events_txframestart: EventsTxframestart,
    events_txframeend: EventsTxframeend,
    events_rxframestart: EventsRxframestart,
    events_rxframeend: EventsRxframeend,
    events_error: EventsError,
    _reserved22: [u8; 0x08],
    events_rxerror: EventsRxerror,
    events_endrx: EventsEndrx,
    events_endtx: EventsEndtx,
    _reserved25: [u8; 0x04],
    events_autocolresstarted: EventsAutocolresstarted,
    _reserved26: [u8; 0x0c],
    events_collision: EventsCollision,
    events_selected: EventsSelected,
    events_started: EventsStarted,
    _reserved29: [u8; 0x2c],
    publish_ready: PublishReady,
    publish_fielddetected: PublishFielddetected,
    publish_fieldlost: PublishFieldlost,
    publish_txframestart: PublishTxframestart,
    publish_txframeend: PublishTxframeend,
    publish_rxframestart: PublishRxframestart,
    publish_rxframeend: PublishRxframeend,
    publish_error: PublishError,
    _reserved37: [u8; 0x08],
    publish_rxerror: PublishRxerror,
    publish_endrx: PublishEndrx,
    publish_endtx: PublishEndtx,
    _reserved40: [u8; 0x04],
    publish_autocolresstarted: PublishAutocolresstarted,
    _reserved41: [u8; 0x0c],
    publish_collision: PublishCollision,
    publish_selected: PublishSelected,
    publish_started: PublishStarted,
    _reserved44: [u8; 0x2c],
    shorts: Shorts,
    _reserved45: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved48: [u8; 0xf8],
    errorstatus: Errorstatus,
    _reserved49: [u8; 0x04],
    framestatus: Framestatus,
    nfctagstate: Nfctagstate,
    _reserved51: [u8; 0x0c],
    sleepstate: Sleepstate,
    _reserved52: [u8; 0x18],
    fieldpresent: Fieldpresent,
    _reserved53: [u8; 0xc4],
    framedelaymin: Framedelaymin,
    framedelaymax: Framedelaymax,
    framedelaymode: Framedelaymode,
    packetptr: Packetptr,
    maxlen: Maxlen,
    txd: Txd,
    rxd: Rxd,
    _reserved60: [u8; 0x04],
    modulationctrl: Modulationctrl,
    _reserved61: [u8; 0x08],
    modulationpsel: Modulationpsel,
    _reserved62: [u8; 0x54],
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
    #[doc = "0x80 - Subscribe configuration for task ACTIVATE"]
    #[inline(always)]
    pub const fn subscribe_activate(&self) -> &SubscribeActivate {
        &self.subscribe_activate
    }
    #[doc = "0x84 - Subscribe configuration for task DISABLE"]
    #[inline(always)]
    pub const fn subscribe_disable(&self) -> &SubscribeDisable {
        &self.subscribe_disable
    }
    #[doc = "0x88 - Subscribe configuration for task SENSE"]
    #[inline(always)]
    pub const fn subscribe_sense(&self) -> &SubscribeSense {
        &self.subscribe_sense
    }
    #[doc = "0x8c - Subscribe configuration for task STARTTX"]
    #[inline(always)]
    pub const fn subscribe_starttx(&self) -> &SubscribeStarttx {
        &self.subscribe_starttx
    }
    #[doc = "0x9c - Subscribe configuration for task ENABLERXDATA"]
    #[inline(always)]
    pub const fn subscribe_enablerxdata(&self) -> &SubscribeEnablerxdata {
        &self.subscribe_enablerxdata
    }
    #[doc = "0xa4 - Subscribe configuration for task GOIDLE"]
    #[inline(always)]
    pub const fn subscribe_goidle(&self) -> &SubscribeGoidle {
        &self.subscribe_goidle
    }
    #[doc = "0xa8 - Subscribe configuration for task GOSLEEP"]
    #[inline(always)]
    pub const fn subscribe_gosleep(&self) -> &SubscribeGosleep {
        &self.subscribe_gosleep
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
    #[doc = "0x180 - Publish configuration for event READY"]
    #[inline(always)]
    pub const fn publish_ready(&self) -> &PublishReady {
        &self.publish_ready
    }
    #[doc = "0x184 - Publish configuration for event FIELDDETECTED"]
    #[inline(always)]
    pub const fn publish_fielddetected(&self) -> &PublishFielddetected {
        &self.publish_fielddetected
    }
    #[doc = "0x188 - Publish configuration for event FIELDLOST"]
    #[inline(always)]
    pub const fn publish_fieldlost(&self) -> &PublishFieldlost {
        &self.publish_fieldlost
    }
    #[doc = "0x18c - Publish configuration for event TXFRAMESTART"]
    #[inline(always)]
    pub const fn publish_txframestart(&self) -> &PublishTxframestart {
        &self.publish_txframestart
    }
    #[doc = "0x190 - Publish configuration for event TXFRAMEEND"]
    #[inline(always)]
    pub const fn publish_txframeend(&self) -> &PublishTxframeend {
        &self.publish_txframeend
    }
    #[doc = "0x194 - Publish configuration for event RXFRAMESTART"]
    #[inline(always)]
    pub const fn publish_rxframestart(&self) -> &PublishRxframestart {
        &self.publish_rxframestart
    }
    #[doc = "0x198 - Publish configuration for event RXFRAMEEND"]
    #[inline(always)]
    pub const fn publish_rxframeend(&self) -> &PublishRxframeend {
        &self.publish_rxframeend
    }
    #[doc = "0x19c - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(&self) -> &PublishError {
        &self.publish_error
    }
    #[doc = "0x1a8 - Publish configuration for event RXERROR"]
    #[inline(always)]
    pub const fn publish_rxerror(&self) -> &PublishRxerror {
        &self.publish_rxerror
    }
    #[doc = "0x1ac - Publish configuration for event ENDRX"]
    #[inline(always)]
    pub const fn publish_endrx(&self) -> &PublishEndrx {
        &self.publish_endrx
    }
    #[doc = "0x1b0 - Publish configuration for event ENDTX"]
    #[inline(always)]
    pub const fn publish_endtx(&self) -> &PublishEndtx {
        &self.publish_endtx
    }
    #[doc = "0x1b8 - Publish configuration for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub const fn publish_autocolresstarted(&self) -> &PublishAutocolresstarted {
        &self.publish_autocolresstarted
    }
    #[doc = "0x1c8 - Publish configuration for event COLLISION"]
    #[inline(always)]
    pub const fn publish_collision(&self) -> &PublishCollision {
        &self.publish_collision
    }
    #[doc = "0x1cc - Publish configuration for event SELECTED"]
    #[inline(always)]
    pub const fn publish_selected(&self) -> &PublishSelected {
        &self.publish_selected
    }
    #[doc = "0x1d0 - Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(&self) -> &PublishStarted {
        &self.publish_started
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
    #[doc = "0x410 - Current operating state of NFC tag"]
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
    #[doc = "0x52c - Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
    #[inline(always)]
    pub const fn modulationctrl(&self) -> &Modulationctrl {
        &self.modulationctrl
    }
    #[doc = "0x538 - Pin select for Modulation control"]
    #[inline(always)]
    pub const fn modulationpsel(&self) -> &Modulationpsel {
        &self.modulationpsel
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
#[doc = "SUBSCRIBE_ACTIVATE (rw) register accessor: Subscribe configuration for task ACTIVATE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_activate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_activate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_activate`] module"]
#[doc(alias = "SUBSCRIBE_ACTIVATE")]
pub type SubscribeActivate = crate::Reg<subscribe_activate::SubscribeActivateSpec>;
#[doc = "Subscribe configuration for task ACTIVATE"]
pub mod subscribe_activate;
#[doc = "SUBSCRIBE_DISABLE (rw) register accessor: Subscribe configuration for task DISABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_disable`] module"]
#[doc(alias = "SUBSCRIBE_DISABLE")]
pub type SubscribeDisable = crate::Reg<subscribe_disable::SubscribeDisableSpec>;
#[doc = "Subscribe configuration for task DISABLE"]
pub mod subscribe_disable;
#[doc = "SUBSCRIBE_SENSE (rw) register accessor: Subscribe configuration for task SENSE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_sense`] module"]
#[doc(alias = "SUBSCRIBE_SENSE")]
pub type SubscribeSense = crate::Reg<subscribe_sense::SubscribeSenseSpec>;
#[doc = "Subscribe configuration for task SENSE"]
pub mod subscribe_sense;
#[doc = "SUBSCRIBE_STARTTX (rw) register accessor: Subscribe configuration for task STARTTX\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_starttx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_starttx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_starttx`] module"]
#[doc(alias = "SUBSCRIBE_STARTTX")]
pub type SubscribeStarttx = crate::Reg<subscribe_starttx::SubscribeStarttxSpec>;
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "SUBSCRIBE_ENABLERXDATA (rw) register accessor: Subscribe configuration for task ENABLERXDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_enablerxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_enablerxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_enablerxdata`] module"]
#[doc(alias = "SUBSCRIBE_ENABLERXDATA")]
pub type SubscribeEnablerxdata = crate::Reg<subscribe_enablerxdata::SubscribeEnablerxdataSpec>;
#[doc = "Subscribe configuration for task ENABLERXDATA"]
pub mod subscribe_enablerxdata;
#[doc = "SUBSCRIBE_GOIDLE (rw) register accessor: Subscribe configuration for task GOIDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_goidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_goidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_goidle`] module"]
#[doc(alias = "SUBSCRIBE_GOIDLE")]
pub type SubscribeGoidle = crate::Reg<subscribe_goidle::SubscribeGoidleSpec>;
#[doc = "Subscribe configuration for task GOIDLE"]
pub mod subscribe_goidle;
#[doc = "SUBSCRIBE_GOSLEEP (rw) register accessor: Subscribe configuration for task GOSLEEP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_gosleep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_gosleep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_gosleep`] module"]
#[doc(alias = "SUBSCRIBE_GOSLEEP")]
pub type SubscribeGosleep = crate::Reg<subscribe_gosleep::SubscribeGosleepSpec>;
#[doc = "Subscribe configuration for task GOSLEEP"]
pub mod subscribe_gosleep;
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
#[doc = "PUBLISH_READY (rw) register accessor: Publish configuration for event READY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ready`] module"]
#[doc(alias = "PUBLISH_READY")]
pub type PublishReady = crate::Reg<publish_ready::PublishReadySpec>;
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "PUBLISH_FIELDDETECTED (rw) register accessor: Publish configuration for event FIELDDETECTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_fielddetected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_fielddetected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_fielddetected`] module"]
#[doc(alias = "PUBLISH_FIELDDETECTED")]
pub type PublishFielddetected = crate::Reg<publish_fielddetected::PublishFielddetectedSpec>;
#[doc = "Publish configuration for event FIELDDETECTED"]
pub mod publish_fielddetected;
#[doc = "PUBLISH_FIELDLOST (rw) register accessor: Publish configuration for event FIELDLOST\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_fieldlost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_fieldlost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_fieldlost`] module"]
#[doc(alias = "PUBLISH_FIELDLOST")]
pub type PublishFieldlost = crate::Reg<publish_fieldlost::PublishFieldlostSpec>;
#[doc = "Publish configuration for event FIELDLOST"]
pub mod publish_fieldlost;
#[doc = "PUBLISH_TXFRAMESTART (rw) register accessor: Publish configuration for event TXFRAMESTART\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_txframestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_txframestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txframestart`] module"]
#[doc(alias = "PUBLISH_TXFRAMESTART")]
pub type PublishTxframestart = crate::Reg<publish_txframestart::PublishTxframestartSpec>;
#[doc = "Publish configuration for event TXFRAMESTART"]
pub mod publish_txframestart;
#[doc = "PUBLISH_TXFRAMEEND (rw) register accessor: Publish configuration for event TXFRAMEEND\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_txframeend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_txframeend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txframeend`] module"]
#[doc(alias = "PUBLISH_TXFRAMEEND")]
pub type PublishTxframeend = crate::Reg<publish_txframeend::PublishTxframeendSpec>;
#[doc = "Publish configuration for event TXFRAMEEND"]
pub mod publish_txframeend;
#[doc = "PUBLISH_RXFRAMESTART (rw) register accessor: Publish configuration for event RXFRAMESTART\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxframestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxframestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxframestart`] module"]
#[doc(alias = "PUBLISH_RXFRAMESTART")]
pub type PublishRxframestart = crate::Reg<publish_rxframestart::PublishRxframestartSpec>;
#[doc = "Publish configuration for event RXFRAMESTART"]
pub mod publish_rxframestart;
#[doc = "PUBLISH_RXFRAMEEND (rw) register accessor: Publish configuration for event RXFRAMEEND\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxframeend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxframeend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxframeend`] module"]
#[doc(alias = "PUBLISH_RXFRAMEEND")]
pub type PublishRxframeend = crate::Reg<publish_rxframeend::PublishRxframeendSpec>;
#[doc = "Publish configuration for event RXFRAMEEND"]
pub mod publish_rxframeend;
#[doc = "PUBLISH_ERROR (rw) register accessor: Publish configuration for event ERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_error`] module"]
#[doc(alias = "PUBLISH_ERROR")]
pub type PublishError = crate::Reg<publish_error::PublishErrorSpec>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_RXERROR (rw) register accessor: Publish configuration for event RXERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxerror`] module"]
#[doc(alias = "PUBLISH_RXERROR")]
pub type PublishRxerror = crate::Reg<publish_rxerror::PublishRxerrorSpec>;
#[doc = "Publish configuration for event RXERROR"]
pub mod publish_rxerror;
#[doc = "PUBLISH_ENDRX (rw) register accessor: Publish configuration for event ENDRX\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endrx`] module"]
#[doc(alias = "PUBLISH_ENDRX")]
pub type PublishEndrx = crate::Reg<publish_endrx::PublishEndrxSpec>;
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "PUBLISH_ENDTX (rw) register accessor: Publish configuration for event ENDTX\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endtx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endtx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endtx`] module"]
#[doc(alias = "PUBLISH_ENDTX")]
pub type PublishEndtx = crate::Reg<publish_endtx::PublishEndtxSpec>;
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "PUBLISH_AUTOCOLRESSTARTED (rw) register accessor: Publish configuration for event AUTOCOLRESSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_autocolresstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_autocolresstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_autocolresstarted`] module"]
#[doc(alias = "PUBLISH_AUTOCOLRESSTARTED")]
pub type PublishAutocolresstarted =
    crate::Reg<publish_autocolresstarted::PublishAutocolresstartedSpec>;
#[doc = "Publish configuration for event AUTOCOLRESSTARTED"]
pub mod publish_autocolresstarted;
#[doc = "PUBLISH_COLLISION (rw) register accessor: Publish configuration for event COLLISION\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_collision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_collision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_collision`] module"]
#[doc(alias = "PUBLISH_COLLISION")]
pub type PublishCollision = crate::Reg<publish_collision::PublishCollisionSpec>;
#[doc = "Publish configuration for event COLLISION"]
pub mod publish_collision;
#[doc = "PUBLISH_SELECTED (rw) register accessor: Publish configuration for event SELECTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_selected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_selected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_selected`] module"]
#[doc(alias = "PUBLISH_SELECTED")]
pub type PublishSelected = crate::Reg<publish_selected::PublishSelectedSpec>;
#[doc = "Publish configuration for event SELECTED"]
pub mod publish_selected;
#[doc = "PUBLISH_STARTED (rw) register accessor: Publish configuration for event STARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_started`] module"]
#[doc(alias = "PUBLISH_STARTED")]
pub type PublishStarted = crate::Reg<publish_started::PublishStartedSpec>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
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
#[doc = "NFCTAGSTATE (r) register accessor: Current operating state of NFC tag\n\nYou can [`read`](crate::Reg::read) this register and get [`nfctagstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfctagstate`] module"]
#[doc(alias = "NFCTAGSTATE")]
pub type Nfctagstate = crate::Reg<nfctagstate::NfctagstateSpec>;
#[doc = "Current operating state of NFC tag"]
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
#[doc = "MODULATIONCTRL (rw) register accessor: Enables the modulation output to a GPIO pin which can be connected to a second external antenna.\n\nYou can [`read`](crate::Reg::read) this register and get [`modulationctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modulationctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulationctrl`] module"]
#[doc(alias = "MODULATIONCTRL")]
pub type Modulationctrl = crate::Reg<modulationctrl::ModulationctrlSpec>;
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
pub mod modulationctrl;
#[doc = "MODULATIONPSEL (rw) register accessor: Pin select for Modulation control\n\nYou can [`read`](crate::Reg::read) this register and get [`modulationpsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modulationpsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulationpsel`] module"]
#[doc(alias = "MODULATIONPSEL")]
pub type Modulationpsel = crate::Reg<modulationpsel::ModulationpselSpec>;
#[doc = "Pin select for Modulation control"]
pub mod modulationpsel;
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
