#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    pub tasks_activate: crate::Reg<tasks_activate::TASKS_ACTIVATE_SPEC>,
    #[doc = "0x04 - Disable NFCT peripheral"]
    pub tasks_disable: crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>,
    #[doc = "0x08 - Enable NFC sense field mode, change state to sense mode"]
    pub tasks_sense: crate::Reg<tasks_sense::TASKS_SENSE_SPEC>,
    #[doc = "0x0c - Start transmission of an outgoing frame, change state to transmit"]
    pub tasks_starttx: crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - Initializes the EasyDMA for receive."]
    pub tasks_enablerxdata: crate::Reg<tasks_enablerxdata::TASKS_ENABLERXDATA_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x24 - Force state machine to IDLE state"]
    pub tasks_goidle: crate::Reg<tasks_goidle::TASKS_GOIDLE_SPEC>,
    #[doc = "0x28 - Force state machine to SLEEP_A state"]
    pub tasks_gosleep: crate::Reg<tasks_gosleep::TASKS_GOSLEEP_SPEC>,
    _reserved7: [u8; 0xd4],
    #[doc = "0x100 - The NFCT peripheral is ready to receive and send frames"]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    #[doc = "0x104 - Remote NFC field detected"]
    pub events_fielddetected: crate::Reg<events_fielddetected::EVENTS_FIELDDETECTED_SPEC>,
    #[doc = "0x108 - Remote NFC field lost"]
    pub events_fieldlost: crate::Reg<events_fieldlost::EVENTS_FIELDLOST_SPEC>,
    #[doc = "0x10c - Marks the start of the first symbol of a transmitted frame"]
    pub events_txframestart: crate::Reg<events_txframestart::EVENTS_TXFRAMESTART_SPEC>,
    #[doc = "0x110 - Marks the end of the last transmitted on-air symbol of a frame"]
    pub events_txframeend: crate::Reg<events_txframeend::EVENTS_TXFRAMEEND_SPEC>,
    #[doc = "0x114 - Marks the end of the first symbol of a received frame"]
    pub events_rxframestart: crate::Reg<events_rxframestart::EVENTS_RXFRAMESTART_SPEC>,
    #[doc = "0x118 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    pub events_rxframeend: crate::Reg<events_rxframeend::EVENTS_RXFRAMEEND_SPEC>,
    #[doc = "0x11c - NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x128 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    pub events_rxerror: crate::Reg<events_rxerror::EVENTS_RXERROR_SPEC>,
    #[doc = "0x12c - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    pub events_endrx: crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>,
    #[doc = "0x130 - Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    pub events_endtx: crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x138 - Auto collision resolution process has started"]
    pub events_autocolresstarted:
        crate::Reg<events_autocolresstarted::EVENTS_AUTOCOLRESSTARTED_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x148 - NFC auto collision resolution error reported."]
    pub events_collision: crate::Reg<events_collision::EVENTS_COLLISION_SPEC>,
    #[doc = "0x14c - NFC auto collision resolution successfully completed"]
    pub events_selected: crate::Reg<events_selected::EVENTS_SELECTED_SPEC>,
    #[doc = "0x150 - EasyDMA is ready to receive or send frames."]
    pub events_started: crate::Reg<events_started::EVENTS_STARTED_SPEC>,
    _reserved22: [u8; 0xac],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved23: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved26: [u8; 0xf8],
    #[doc = "0x404 - NFC Error Status register"]
    pub errorstatus: crate::Reg<errorstatus::ERRORSTATUS_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x40c - Unspecified"]
    pub framestatus: FRAMESTATUS,
    #[doc = "0x410 - NfcTag state register"]
    pub nfctagstate: crate::Reg<nfctagstate::NFCTAGSTATE_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x420 - Sleep state during automatic collision resolution"]
    pub sleepstate: crate::Reg<sleepstate::SLEEPSTATE_SPEC>,
    _reserved30: [u8; 0x18],
    #[doc = "0x43c - Indicates the presence or not of a valid field"]
    pub fieldpresent: crate::Reg<fieldpresent::FIELDPRESENT_SPEC>,
    _reserved31: [u8; 0xc4],
    #[doc = "0x504 - Minimum frame delay"]
    pub framedelaymin: crate::Reg<framedelaymin::FRAMEDELAYMIN_SPEC>,
    #[doc = "0x508 - Maximum frame delay"]
    pub framedelaymax: crate::Reg<framedelaymax::FRAMEDELAYMAX_SPEC>,
    #[doc = "0x50c - Configuration register for the Frame Delay Timer"]
    pub framedelaymode: crate::Reg<framedelaymode::FRAMEDELAYMODE_SPEC>,
    #[doc = "0x510 - Packet pointer for TXD and RXD data storage in Data RAM"]
    pub packetptr: crate::Reg<packetptr::PACKETPTR_SPEC>,
    #[doc = "0x514 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    pub maxlen: crate::Reg<maxlen::MAXLEN_SPEC>,
    #[doc = "0x518..0x520 - Unspecified"]
    pub txd: TXD,
    #[doc = "0x520..0x528 - Unspecified"]
    pub rxd: RXD,
    _reserved38: [u8; 0x04],
    #[doc = "0x52c - Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
    pub modulationctrl: crate::Reg<modulationctrl::MODULATIONCTRL_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x538 - Pin select for Modulation control."]
    pub modulationpsel: crate::Reg<modulationpsel::MODULATIONPSEL_SPEC>,
    _reserved40: [u8; 0x54],
    #[doc = "0x590 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    pub nfcid1_last: crate::Reg<nfcid1_last::NFCID1_LAST_SPEC>,
    #[doc = "0x594 - Second last NFCID1 part (7 or 10 bytes ID)"]
    pub nfcid1_2nd_last: crate::Reg<nfcid1_2nd_last::NFCID1_2ND_LAST_SPEC>,
    #[doc = "0x598 - Third last NFCID1 part (10 bytes ID)"]
    pub nfcid1_3rd_last: crate::Reg<nfcid1_3rd_last::NFCID1_3RD_LAST_SPEC>,
    #[doc = "0x59c - Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
    pub autocolresconfig: crate::Reg<autocolresconfig::AUTOCOLRESCONFIG_SPEC>,
    #[doc = "0x5a0 - NFC-A SENS_RES auto-response settings"]
    pub sensres: crate::Reg<sensres::SENSRES_SPEC>,
    #[doc = "0x5a4 - NFC-A SEL_RES auto-response settings"]
    pub selres: crate::Reg<selres::SELRES_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMESTATUS {
    #[doc = "0x00 - Result of last incoming frame"]
    pub rx: crate::Reg<self::framestatus::rx::RX_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod framestatus;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Configuration of outgoing frames"]
    pub frameconfig: crate::Reg<self::txd::frameconfig::FRAMECONFIG_SPEC>,
    #[doc = "0x04 - Size of outgoing frame"]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Configuration of incoming frames"]
    pub frameconfig: crate::Reg<self::rxd::frameconfig::FRAMECONFIG_SPEC>,
    #[doc = "0x04 - Size of last incoming frame"]
    pub amount: crate::Reg<self::rxd::amount::AMOUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "TASKS_ACTIVATE register accessor: an alias for `Reg<TASKS_ACTIVATE_SPEC>`"]
pub type TASKS_ACTIVATE = crate::Reg<tasks_activate::TASKS_ACTIVATE_SPEC>;
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub mod tasks_activate;
#[doc = "TASKS_DISABLE register accessor: an alias for `Reg<TASKS_DISABLE_SPEC>`"]
pub type TASKS_DISABLE = crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>;
#[doc = "Disable NFCT peripheral"]
pub mod tasks_disable;
#[doc = "TASKS_SENSE register accessor: an alias for `Reg<TASKS_SENSE_SPEC>`"]
pub type TASKS_SENSE = crate::Reg<tasks_sense::TASKS_SENSE_SPEC>;
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub mod tasks_sense;
#[doc = "TASKS_STARTTX register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start transmission of an outgoing frame, change state to transmit"]
pub mod tasks_starttx;
#[doc = "TASKS_ENABLERXDATA register accessor: an alias for `Reg<TASKS_ENABLERXDATA_SPEC>`"]
pub type TASKS_ENABLERXDATA = crate::Reg<tasks_enablerxdata::TASKS_ENABLERXDATA_SPEC>;
#[doc = "Initializes the EasyDMA for receive."]
pub mod tasks_enablerxdata;
#[doc = "TASKS_GOIDLE register accessor: an alias for `Reg<TASKS_GOIDLE_SPEC>`"]
pub type TASKS_GOIDLE = crate::Reg<tasks_goidle::TASKS_GOIDLE_SPEC>;
#[doc = "Force state machine to IDLE state"]
pub mod tasks_goidle;
#[doc = "TASKS_GOSLEEP register accessor: an alias for `Reg<TASKS_GOSLEEP_SPEC>`"]
pub type TASKS_GOSLEEP = crate::Reg<tasks_gosleep::TASKS_GOSLEEP_SPEC>;
#[doc = "Force state machine to SLEEP_A state"]
pub mod tasks_gosleep;
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "The NFCT peripheral is ready to receive and send frames"]
pub mod events_ready;
#[doc = "EVENTS_FIELDDETECTED register accessor: an alias for `Reg<EVENTS_FIELDDETECTED_SPEC>`"]
pub type EVENTS_FIELDDETECTED = crate::Reg<events_fielddetected::EVENTS_FIELDDETECTED_SPEC>;
#[doc = "Remote NFC field detected"]
pub mod events_fielddetected;
#[doc = "EVENTS_FIELDLOST register accessor: an alias for `Reg<EVENTS_FIELDLOST_SPEC>`"]
pub type EVENTS_FIELDLOST = crate::Reg<events_fieldlost::EVENTS_FIELDLOST_SPEC>;
#[doc = "Remote NFC field lost"]
pub mod events_fieldlost;
#[doc = "EVENTS_TXFRAMESTART register accessor: an alias for `Reg<EVENTS_TXFRAMESTART_SPEC>`"]
pub type EVENTS_TXFRAMESTART = crate::Reg<events_txframestart::EVENTS_TXFRAMESTART_SPEC>;
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub mod events_txframestart;
#[doc = "EVENTS_TXFRAMEEND register accessor: an alias for `Reg<EVENTS_TXFRAMEEND_SPEC>`"]
pub type EVENTS_TXFRAMEEND = crate::Reg<events_txframeend::EVENTS_TXFRAMEEND_SPEC>;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub mod events_txframeend;
#[doc = "EVENTS_RXFRAMESTART register accessor: an alias for `Reg<EVENTS_RXFRAMESTART_SPEC>`"]
pub type EVENTS_RXFRAMESTART = crate::Reg<events_rxframestart::EVENTS_RXFRAMESTART_SPEC>;
#[doc = "Marks the end of the first symbol of a received frame"]
pub mod events_rxframestart;
#[doc = "EVENTS_RXFRAMEEND register accessor: an alias for `Reg<EVENTS_RXFRAMEEND_SPEC>`"]
pub type EVENTS_RXFRAMEEND = crate::Reg<events_rxframeend::EVENTS_RXFRAMEEND_SPEC>;
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub mod events_rxframeend;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub mod events_error;
#[doc = "EVENTS_RXERROR register accessor: an alias for `Reg<EVENTS_RXERROR_SPEC>`"]
pub type EVENTS_RXERROR = crate::Reg<events_rxerror::EVENTS_RXERROR_SPEC>;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub mod events_rxerror;
#[doc = "EVENTS_ENDRX register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub mod events_endrx;
#[doc = "EVENTS_ENDTX register accessor: an alias for `Reg<EVENTS_ENDTX_SPEC>`"]
pub type EVENTS_ENDTX = crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>;
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub mod events_endtx;
#[doc = "EVENTS_AUTOCOLRESSTARTED register accessor: an alias for `Reg<EVENTS_AUTOCOLRESSTARTED_SPEC>`"]
pub type EVENTS_AUTOCOLRESSTARTED =
    crate::Reg<events_autocolresstarted::EVENTS_AUTOCOLRESSTARTED_SPEC>;
#[doc = "Auto collision resolution process has started"]
pub mod events_autocolresstarted;
#[doc = "EVENTS_COLLISION register accessor: an alias for `Reg<EVENTS_COLLISION_SPEC>`"]
pub type EVENTS_COLLISION = crate::Reg<events_collision::EVENTS_COLLISION_SPEC>;
#[doc = "NFC auto collision resolution error reported."]
pub mod events_collision;
#[doc = "EVENTS_SELECTED register accessor: an alias for `Reg<EVENTS_SELECTED_SPEC>`"]
pub type EVENTS_SELECTED = crate::Reg<events_selected::EVENTS_SELECTED_SPEC>;
#[doc = "NFC auto collision resolution successfully completed"]
pub mod events_selected;
#[doc = "EVENTS_STARTED register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "EasyDMA is ready to receive or send frames."]
pub mod events_started;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ERRORSTATUS register accessor: an alias for `Reg<ERRORSTATUS_SPEC>`"]
pub type ERRORSTATUS = crate::Reg<errorstatus::ERRORSTATUS_SPEC>;
#[doc = "NFC Error Status register"]
pub mod errorstatus;
#[doc = "NFCTAGSTATE register accessor: an alias for `Reg<NFCTAGSTATE_SPEC>`"]
pub type NFCTAGSTATE = crate::Reg<nfctagstate::NFCTAGSTATE_SPEC>;
#[doc = "NfcTag state register"]
pub mod nfctagstate;
#[doc = "SLEEPSTATE register accessor: an alias for `Reg<SLEEPSTATE_SPEC>`"]
pub type SLEEPSTATE = crate::Reg<sleepstate::SLEEPSTATE_SPEC>;
#[doc = "Sleep state during automatic collision resolution"]
pub mod sleepstate;
#[doc = "FIELDPRESENT register accessor: an alias for `Reg<FIELDPRESENT_SPEC>`"]
pub type FIELDPRESENT = crate::Reg<fieldpresent::FIELDPRESENT_SPEC>;
#[doc = "Indicates the presence or not of a valid field"]
pub mod fieldpresent;
#[doc = "FRAMEDELAYMIN register accessor: an alias for `Reg<FRAMEDELAYMIN_SPEC>`"]
pub type FRAMEDELAYMIN = crate::Reg<framedelaymin::FRAMEDELAYMIN_SPEC>;
#[doc = "Minimum frame delay"]
pub mod framedelaymin;
#[doc = "FRAMEDELAYMAX register accessor: an alias for `Reg<FRAMEDELAYMAX_SPEC>`"]
pub type FRAMEDELAYMAX = crate::Reg<framedelaymax::FRAMEDELAYMAX_SPEC>;
#[doc = "Maximum frame delay"]
pub mod framedelaymax;
#[doc = "FRAMEDELAYMODE register accessor: an alias for `Reg<FRAMEDELAYMODE_SPEC>`"]
pub type FRAMEDELAYMODE = crate::Reg<framedelaymode::FRAMEDELAYMODE_SPEC>;
#[doc = "Configuration register for the Frame Delay Timer"]
pub mod framedelaymode;
#[doc = "PACKETPTR register accessor: an alias for `Reg<PACKETPTR_SPEC>`"]
pub type PACKETPTR = crate::Reg<packetptr::PACKETPTR_SPEC>;
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub mod packetptr;
#[doc = "MAXLEN register accessor: an alias for `Reg<MAXLEN_SPEC>`"]
pub type MAXLEN = crate::Reg<maxlen::MAXLEN_SPEC>;
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub mod maxlen;
#[doc = "MODULATIONCTRL register accessor: an alias for `Reg<MODULATIONCTRL_SPEC>`"]
pub type MODULATIONCTRL = crate::Reg<modulationctrl::MODULATIONCTRL_SPEC>;
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
pub mod modulationctrl;
#[doc = "MODULATIONPSEL register accessor: an alias for `Reg<MODULATIONPSEL_SPEC>`"]
pub type MODULATIONPSEL = crate::Reg<modulationpsel::MODULATIONPSEL_SPEC>;
#[doc = "Pin select for Modulation control."]
pub mod modulationpsel;
#[doc = "NFCID1_LAST register accessor: an alias for `Reg<NFCID1_LAST_SPEC>`"]
pub type NFCID1_LAST = crate::Reg<nfcid1_last::NFCID1_LAST_SPEC>;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod nfcid1_last;
#[doc = "NFCID1_2ND_LAST register accessor: an alias for `Reg<NFCID1_2ND_LAST_SPEC>`"]
pub type NFCID1_2ND_LAST = crate::Reg<nfcid1_2nd_last::NFCID1_2ND_LAST_SPEC>;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod nfcid1_2nd_last;
#[doc = "NFCID1_3RD_LAST register accessor: an alias for `Reg<NFCID1_3RD_LAST_SPEC>`"]
pub type NFCID1_3RD_LAST = crate::Reg<nfcid1_3rd_last::NFCID1_3RD_LAST_SPEC>;
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod nfcid1_3rd_last;
#[doc = "AUTOCOLRESCONFIG register accessor: an alias for `Reg<AUTOCOLRESCONFIG_SPEC>`"]
pub type AUTOCOLRESCONFIG = crate::Reg<autocolresconfig::AUTOCOLRESCONFIG_SPEC>;
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
pub mod autocolresconfig;
#[doc = "SENSRES register accessor: an alias for `Reg<SENSRES_SPEC>`"]
pub type SENSRES = crate::Reg<sensres::SENSRES_SPEC>;
#[doc = "NFC-A SENS_RES auto-response settings"]
pub mod sensres;
#[doc = "SELRES register accessor: an alias for `Reg<SELRES_SPEC>`"]
pub type SELRES = crate::Reg<selres::SELRES_SPEC>;
#[doc = "NFC-A SEL_RES auto-response settings"]
pub mod selres;
