#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    pub tasks_txen: TASKS_TXEN,
    #[doc = "0x04 - Enable RADIO in RX mode"]
    pub tasks_rxen: TASKS_RXEN,
    #[doc = "0x08 - Start RADIO"]
    pub tasks_start: TASKS_START,
    #[doc = "0x0c - Stop RADIO"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x10 - Disable RADIO"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength"]
    pub tasks_rssistart: TASKS_RSSISTART,
    #[doc = "0x18 - Stop the RSSI measurement"]
    pub tasks_rssistop: TASKS_RSSISTOP,
    #[doc = "0x1c - Start the bit counter"]
    pub tasks_bcstart: TASKS_BCSTART,
    #[doc = "0x20 - Stop the bit counter"]
    pub tasks_bcstop: TASKS_BCSTOP,
    #[doc = "0x24 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    pub tasks_edstart: TASKS_EDSTART,
    #[doc = "0x28 - Stop the energy detect measurement"]
    pub tasks_edstop: TASKS_EDSTOP,
    #[doc = "0x2c - Start the clear channel assessment used in IEEE 802.15.4 mode"]
    pub tasks_ccastart: TASKS_CCASTART,
    #[doc = "0x30 - Stop the clear channel assessment"]
    pub tasks_ccastop: TASKS_CCASTOP,
    _reserved13: [u8; 0x4c],
    #[doc = "0x80 - Subscribe configuration for task TXEN"]
    pub subscribe_txen: SUBSCRIBE_TXEN,
    #[doc = "0x84 - Subscribe configuration for task RXEN"]
    pub subscribe_rxen: SUBSCRIBE_RXEN,
    #[doc = "0x88 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x8c - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x90 - Subscribe configuration for task DISABLE"]
    pub subscribe_disable: SUBSCRIBE_DISABLE,
    #[doc = "0x94 - Subscribe configuration for task RSSISTART"]
    pub subscribe_rssistart: SUBSCRIBE_RSSISTART,
    #[doc = "0x98 - Subscribe configuration for task RSSISTOP"]
    pub subscribe_rssistop: SUBSCRIBE_RSSISTOP,
    #[doc = "0x9c - Subscribe configuration for task BCSTART"]
    pub subscribe_bcstart: SUBSCRIBE_BCSTART,
    #[doc = "0xa0 - Subscribe configuration for task BCSTOP"]
    pub subscribe_bcstop: SUBSCRIBE_BCSTOP,
    #[doc = "0xa4 - Subscribe configuration for task EDSTART"]
    pub subscribe_edstart: SUBSCRIBE_EDSTART,
    #[doc = "0xa8 - Subscribe configuration for task EDSTOP"]
    pub subscribe_edstop: SUBSCRIBE_EDSTOP,
    #[doc = "0xac - Subscribe configuration for task CCASTART"]
    pub subscribe_ccastart: SUBSCRIBE_CCASTART,
    #[doc = "0xb0 - Subscribe configuration for task CCASTOP"]
    pub subscribe_ccastop: SUBSCRIBE_CCASTOP,
    _reserved26: [u8; 0x4c],
    #[doc = "0x100 - RADIO has ramped up and is ready to be started"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Address sent or received"]
    pub events_address: EVENTS_ADDRESS,
    #[doc = "0x108 - Packet payload sent or received"]
    pub events_payload: EVENTS_PAYLOAD,
    #[doc = "0x10c - Packet sent or received"]
    pub events_end: EVENTS_END,
    #[doc = "0x110 - RADIO has been disabled"]
    pub events_disabled: EVENTS_DISABLED,
    #[doc = "0x114 - A device address match occurred on the last received packet"]
    pub events_devmatch: EVENTS_DEVMATCH,
    #[doc = "0x118 - No device address match occurred on the last received packet"]
    pub events_devmiss: EVENTS_DEVMISS,
    #[doc = "0x11c - Sampling of receive signal strength complete"]
    pub events_rssiend: EVENTS_RSSIEND,
    _reserved34: [u8; 0x08],
    #[doc = "0x128 - Bit counter reached bit count value"]
    pub events_bcmatch: EVENTS_BCMATCH,
    _reserved35: [u8; 0x04],
    #[doc = "0x130 - Packet received with CRC ok"]
    pub events_crcok: EVENTS_CRCOK,
    #[doc = "0x134 - Packet received with CRC error"]
    pub events_crcerror: EVENTS_CRCERROR,
    #[doc = "0x138 - IEEE 802.15.4 length field received"]
    pub events_framestart: EVENTS_FRAMESTART,
    #[doc = "0x13c - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    pub events_edend: EVENTS_EDEND,
    #[doc = "0x140 - The sampling of energy detection has stopped"]
    pub events_edstopped: EVENTS_EDSTOPPED,
    #[doc = "0x144 - Wireless medium in idle - clear to send"]
    pub events_ccaidle: EVENTS_CCAIDLE,
    #[doc = "0x148 - Wireless medium busy - do not send"]
    pub events_ccabusy: EVENTS_CCABUSY,
    #[doc = "0x14c - The CCA has stopped"]
    pub events_ccastopped: EVENTS_CCASTOPPED,
    #[doc = "0x150 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    pub events_rateboost: EVENTS_RATEBOOST,
    #[doc = "0x154 - RADIO has ramped up and is ready to be started TX path"]
    pub events_txready: EVENTS_TXREADY,
    #[doc = "0x158 - RADIO has ramped up and is ready to be started RX path"]
    pub events_rxready: EVENTS_RXREADY,
    #[doc = "0x15c - MAC header match found"]
    pub events_mhrmatch: EVENTS_MHRMATCH,
    _reserved47: [u8; 0x08],
    #[doc = "0x168 - Preamble indicator"]
    pub events_sync: EVENTS_SYNC,
    #[doc = "0x16c - Generated when last bit is sent on air, or received from air"]
    pub events_phyend: EVENTS_PHYEND,
    #[doc = "0x170 - CTE is present (early warning right after receiving CTEInfo byte)"]
    pub events_ctepresent: EVENTS_CTEPRESENT,
    _reserved50: [u8; 0x0c],
    #[doc = "0x180 - Publish configuration for event READY"]
    pub publish_ready: PUBLISH_READY,
    #[doc = "0x184 - Publish configuration for event ADDRESS"]
    pub publish_address: PUBLISH_ADDRESS,
    #[doc = "0x188 - Publish configuration for event PAYLOAD"]
    pub publish_payload: PUBLISH_PAYLOAD,
    #[doc = "0x18c - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    #[doc = "0x190 - Publish configuration for event DISABLED"]
    pub publish_disabled: PUBLISH_DISABLED,
    #[doc = "0x194 - Publish configuration for event DEVMATCH"]
    pub publish_devmatch: PUBLISH_DEVMATCH,
    #[doc = "0x198 - Publish configuration for event DEVMISS"]
    pub publish_devmiss: PUBLISH_DEVMISS,
    #[doc = "0x19c - Publish configuration for event RSSIEND"]
    pub publish_rssiend: PUBLISH_RSSIEND,
    _reserved58: [u8; 0x08],
    #[doc = "0x1a8 - Publish configuration for event BCMATCH"]
    pub publish_bcmatch: PUBLISH_BCMATCH,
    _reserved59: [u8; 0x04],
    #[doc = "0x1b0 - Publish configuration for event CRCOK"]
    pub publish_crcok: PUBLISH_CRCOK,
    #[doc = "0x1b4 - Publish configuration for event CRCERROR"]
    pub publish_crcerror: PUBLISH_CRCERROR,
    #[doc = "0x1b8 - Publish configuration for event FRAMESTART"]
    pub publish_framestart: PUBLISH_FRAMESTART,
    #[doc = "0x1bc - Publish configuration for event EDEND"]
    pub publish_edend: PUBLISH_EDEND,
    #[doc = "0x1c0 - Publish configuration for event EDSTOPPED"]
    pub publish_edstopped: PUBLISH_EDSTOPPED,
    #[doc = "0x1c4 - Publish configuration for event CCAIDLE"]
    pub publish_ccaidle: PUBLISH_CCAIDLE,
    #[doc = "0x1c8 - Publish configuration for event CCABUSY"]
    pub publish_ccabusy: PUBLISH_CCABUSY,
    #[doc = "0x1cc - Publish configuration for event CCASTOPPED"]
    pub publish_ccastopped: PUBLISH_CCASTOPPED,
    #[doc = "0x1d0 - Publish configuration for event RATEBOOST"]
    pub publish_rateboost: PUBLISH_RATEBOOST,
    #[doc = "0x1d4 - Publish configuration for event TXREADY"]
    pub publish_txready: PUBLISH_TXREADY,
    #[doc = "0x1d8 - Publish configuration for event RXREADY"]
    pub publish_rxready: PUBLISH_RXREADY,
    #[doc = "0x1dc - Publish configuration for event MHRMATCH"]
    pub publish_mhrmatch: PUBLISH_MHRMATCH,
    _reserved71: [u8; 0x08],
    #[doc = "0x1e8 - Publish configuration for event SYNC"]
    pub publish_sync: PUBLISH_SYNC,
    #[doc = "0x1ec - Publish configuration for event PHYEND"]
    pub publish_phyend: PUBLISH_PHYEND,
    #[doc = "0x1f0 - Publish configuration for event CTEPRESENT"]
    pub publish_ctepresent: PUBLISH_CTEPRESENT,
    _reserved74: [u8; 0x0c],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved75: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved77: [u8; 0xf4],
    #[doc = "0x400 - CRC status"]
    pub crcstatus: CRCSTATUS,
    _reserved78: [u8; 0x04],
    #[doc = "0x408 - Received address"]
    pub rxmatch: RXMATCH,
    #[doc = "0x40c - CRC field of previously received packet"]
    pub rxcrc: RXCRC,
    #[doc = "0x410 - Device address match index"]
    pub dai: DAI,
    #[doc = "0x414 - Payload status"]
    pub pdustat: PDUSTAT,
    _reserved82: [u8; 0x34],
    #[doc = "0x44c - CTEInfo parsed from received packet"]
    pub ctestatus: CTESTATUS,
    _reserved83: [u8; 0x08],
    #[doc = "0x458 - DFE status information"]
    pub dfestatus: DFESTATUS,
    _reserved84: [u8; 0xa8],
    #[doc = "0x504 - Packet pointer"]
    pub packetptr: PACKETPTR,
    #[doc = "0x508 - Frequency"]
    pub frequency: FREQUENCY,
    #[doc = "0x50c - Output power"]
    pub txpower: TXPOWER,
    #[doc = "0x510 - Data rate and modulation"]
    pub mode: MODE,
    #[doc = "0x514 - Packet configuration register 0"]
    pub pcnf0: PCNF0,
    #[doc = "0x518 - Packet configuration register 1"]
    pub pcnf1: PCNF1,
    #[doc = "0x51c - Base address 0"]
    pub base0: BASE0,
    #[doc = "0x520 - Base address 1"]
    pub base1: BASE1,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0-3"]
    pub prefix0: PREFIX0,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4-7"]
    pub prefix1: PREFIX1,
    #[doc = "0x52c - Transmit address select"]
    pub txaddress: TXADDRESS,
    #[doc = "0x530 - Receive address select"]
    pub rxaddresses: RXADDRESSES,
    #[doc = "0x534 - CRC configuration"]
    pub crccnf: CRCCNF,
    #[doc = "0x538 - CRC polynomial"]
    pub crcpoly: CRCPOLY,
    #[doc = "0x53c - CRC initial value"]
    pub crcinit: CRCINIT,
    _reserved99: [u8; 0x04],
    #[doc = "0x544 - Interframe spacing in us"]
    pub tifs: TIFS,
    #[doc = "0x548 - RSSI sample"]
    pub rssisample: RSSISAMPLE,
    _reserved101: [u8; 0x04],
    #[doc = "0x550 - Current radio state"]
    pub state: STATE,
    #[doc = "0x554 - Data whitening initial value"]
    pub datawhiteiv: DATAWHITEIV,
    _reserved103: [u8; 0x08],
    #[doc = "0x560 - Bit counter compare"]
    pub bcc: BCC,
    _reserved104: [u8; 0x9c],
    #[doc = "0x600..0x620 - Description collection: Device address base segment n"]
    pub dab: [DAB; 8],
    #[doc = "0x620..0x640 - Description collection: Device address prefix n"]
    pub dap: [DAP; 8],
    #[doc = "0x640 - Device address match configuration"]
    pub dacnf: DACNF,
    #[doc = "0x644 - Search pattern configuration"]
    pub mhrmatchconf: MHRMATCHCONF,
    #[doc = "0x648 - Pattern mask"]
    pub mhrmatchmas: MHRMATCHMAS,
    _reserved109: [u8; 0x04],
    #[doc = "0x650 - Radio mode configuration register 0"]
    pub modecnf0: MODECNF0,
    _reserved110: [u8; 0x0c],
    #[doc = "0x660 - IEEE 802.15.4 start of frame delimiter"]
    pub sfd: SFD,
    #[doc = "0x664 - IEEE 802.15.4 energy detect loop count"]
    pub edcnt: EDCNT,
    #[doc = "0x668 - IEEE 802.15.4 energy detect level"]
    pub edsample: EDSAMPLE,
    #[doc = "0x66c - IEEE 802.15.4 clear channel assessment control"]
    pub ccactrl: CCACTRL,
    _reserved114: [u8; 0x0290],
    #[doc = "0x900 - Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
    pub dfemode: DFEMODE,
    #[doc = "0x904 - Configuration for CTE inline mode"]
    pub cteinlineconf: CTEINLINECONF,
    _reserved116: [u8; 0x08],
    #[doc = "0x910 - Various configuration for Direction finding"]
    pub dfectrl1: DFECTRL1,
    #[doc = "0x914 - Start offset for Direction finding"]
    pub dfectrl2: DFECTRL2,
    _reserved118: [u8; 0x10],
    #[doc = "0x928 - GPIO patterns to be used for each antenna"]
    pub switchpattern: SWITCHPATTERN,
    #[doc = "0x92c - Clear the GPIO pattern array for antenna control"]
    pub clearpattern: CLEARPATTERN,
    #[doc = "0x930..0x950 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x950..0x95c - DFE packet EasyDMA channel"]
    pub dfepacket: DFEPACKET,
    _reserved122: [u8; 0x06a0],
    #[doc = "0xffc - Peripheral power control"]
    pub power: POWER,
}
#[doc = "TASKS_TXEN (w) register accessor: an alias for `Reg<TASKS_TXEN_SPEC>`"]
pub type TASKS_TXEN = crate::Reg<tasks_txen::TASKS_TXEN_SPEC>;
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "TASKS_RXEN (w) register accessor: an alias for `Reg<TASKS_RXEN_SPEC>`"]
pub type TASKS_RXEN = crate::Reg<tasks_rxen::TASKS_RXEN_SPEC>;
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE (w) register accessor: an alias for `Reg<TASKS_DISABLE_SPEC>`"]
pub type TASKS_DISABLE = crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>;
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART (w) register accessor: an alias for `Reg<TASKS_RSSISTART_SPEC>`"]
pub type TASKS_RSSISTART = crate::Reg<tasks_rssistart::TASKS_RSSISTART_SPEC>;
#[doc = "Start the RSSI and take one single sample of the receive signal strength"]
pub mod tasks_rssistart;
#[doc = "TASKS_RSSISTOP (w) register accessor: an alias for `Reg<TASKS_RSSISTOP_SPEC>`"]
pub type TASKS_RSSISTOP = crate::Reg<tasks_rssistop::TASKS_RSSISTOP_SPEC>;
#[doc = "Stop the RSSI measurement"]
pub mod tasks_rssistop;
#[doc = "TASKS_BCSTART (w) register accessor: an alias for `Reg<TASKS_BCSTART_SPEC>`"]
pub type TASKS_BCSTART = crate::Reg<tasks_bcstart::TASKS_BCSTART_SPEC>;
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP (w) register accessor: an alias for `Reg<TASKS_BCSTOP_SPEC>`"]
pub type TASKS_BCSTOP = crate::Reg<tasks_bcstop::TASKS_BCSTOP_SPEC>;
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "TASKS_EDSTART (w) register accessor: an alias for `Reg<TASKS_EDSTART_SPEC>`"]
pub type TASKS_EDSTART = crate::Reg<tasks_edstart::TASKS_EDSTART_SPEC>;
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub mod tasks_edstart;
#[doc = "TASKS_EDSTOP (w) register accessor: an alias for `Reg<TASKS_EDSTOP_SPEC>`"]
pub type TASKS_EDSTOP = crate::Reg<tasks_edstop::TASKS_EDSTOP_SPEC>;
#[doc = "Stop the energy detect measurement"]
pub mod tasks_edstop;
#[doc = "TASKS_CCASTART (w) register accessor: an alias for `Reg<TASKS_CCASTART_SPEC>`"]
pub type TASKS_CCASTART = crate::Reg<tasks_ccastart::TASKS_CCASTART_SPEC>;
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
pub mod tasks_ccastart;
#[doc = "TASKS_CCASTOP (w) register accessor: an alias for `Reg<TASKS_CCASTOP_SPEC>`"]
pub type TASKS_CCASTOP = crate::Reg<tasks_ccastop::TASKS_CCASTOP_SPEC>;
#[doc = "Stop the clear channel assessment"]
pub mod tasks_ccastop;
#[doc = "SUBSCRIBE_TXEN (rw) register accessor: an alias for `Reg<SUBSCRIBE_TXEN_SPEC>`"]
pub type SUBSCRIBE_TXEN = crate::Reg<subscribe_txen::SUBSCRIBE_TXEN_SPEC>;
#[doc = "Subscribe configuration for task TXEN"]
pub mod subscribe_txen;
#[doc = "SUBSCRIBE_RXEN (rw) register accessor: an alias for `Reg<SUBSCRIBE_RXEN_SPEC>`"]
pub type SUBSCRIBE_RXEN = crate::Reg<subscribe_rxen::SUBSCRIBE_RXEN_SPEC>;
#[doc = "Subscribe configuration for task RXEN"]
pub mod subscribe_rxen;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_DISABLE (rw) register accessor: an alias for `Reg<SUBSCRIBE_DISABLE_SPEC>`"]
pub type SUBSCRIBE_DISABLE = crate::Reg<subscribe_disable::SUBSCRIBE_DISABLE_SPEC>;
#[doc = "Subscribe configuration for task DISABLE"]
pub mod subscribe_disable;
#[doc = "SUBSCRIBE_RSSISTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_RSSISTART_SPEC>`"]
pub type SUBSCRIBE_RSSISTART = crate::Reg<subscribe_rssistart::SUBSCRIBE_RSSISTART_SPEC>;
#[doc = "Subscribe configuration for task RSSISTART"]
pub mod subscribe_rssistart;
#[doc = "SUBSCRIBE_RSSISTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_RSSISTOP_SPEC>`"]
pub type SUBSCRIBE_RSSISTOP = crate::Reg<subscribe_rssistop::SUBSCRIBE_RSSISTOP_SPEC>;
#[doc = "Subscribe configuration for task RSSISTOP"]
pub mod subscribe_rssistop;
#[doc = "SUBSCRIBE_BCSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_BCSTART_SPEC>`"]
pub type SUBSCRIBE_BCSTART = crate::Reg<subscribe_bcstart::SUBSCRIBE_BCSTART_SPEC>;
#[doc = "Subscribe configuration for task BCSTART"]
pub mod subscribe_bcstart;
#[doc = "SUBSCRIBE_BCSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_BCSTOP_SPEC>`"]
pub type SUBSCRIBE_BCSTOP = crate::Reg<subscribe_bcstop::SUBSCRIBE_BCSTOP_SPEC>;
#[doc = "Subscribe configuration for task BCSTOP"]
pub mod subscribe_bcstop;
#[doc = "SUBSCRIBE_EDSTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_EDSTART_SPEC>`"]
pub type SUBSCRIBE_EDSTART = crate::Reg<subscribe_edstart::SUBSCRIBE_EDSTART_SPEC>;
#[doc = "Subscribe configuration for task EDSTART"]
pub mod subscribe_edstart;
#[doc = "SUBSCRIBE_EDSTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_EDSTOP_SPEC>`"]
pub type SUBSCRIBE_EDSTOP = crate::Reg<subscribe_edstop::SUBSCRIBE_EDSTOP_SPEC>;
#[doc = "Subscribe configuration for task EDSTOP"]
pub mod subscribe_edstop;
#[doc = "SUBSCRIBE_CCASTART (rw) register accessor: an alias for `Reg<SUBSCRIBE_CCASTART_SPEC>`"]
pub type SUBSCRIBE_CCASTART = crate::Reg<subscribe_ccastart::SUBSCRIBE_CCASTART_SPEC>;
#[doc = "Subscribe configuration for task CCASTART"]
pub mod subscribe_ccastart;
#[doc = "SUBSCRIBE_CCASTOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_CCASTOP_SPEC>`"]
pub type SUBSCRIBE_CCASTOP = crate::Reg<subscribe_ccastop::SUBSCRIBE_CCASTOP_SPEC>;
#[doc = "Subscribe configuration for task CCASTOP"]
pub mod subscribe_ccastop;
#[doc = "EVENTS_READY (rw) register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "EVENTS_ADDRESS (rw) register accessor: an alias for `Reg<EVENTS_ADDRESS_SPEC>`"]
pub type EVENTS_ADDRESS = crate::Reg<events_address::EVENTS_ADDRESS_SPEC>;
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "EVENTS_PAYLOAD (rw) register accessor: an alias for `Reg<EVENTS_PAYLOAD_SPEC>`"]
pub type EVENTS_PAYLOAD = crate::Reg<events_payload::EVENTS_PAYLOAD_SPEC>;
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Packet sent or received"]
pub mod events_end;
#[doc = "EVENTS_DISABLED (rw) register accessor: an alias for `Reg<EVENTS_DISABLED_SPEC>`"]
pub type EVENTS_DISABLED = crate::Reg<events_disabled::EVENTS_DISABLED_SPEC>;
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH (rw) register accessor: an alias for `Reg<EVENTS_DEVMATCH_SPEC>`"]
pub type EVENTS_DEVMATCH = crate::Reg<events_devmatch::EVENTS_DEVMATCH_SPEC>;
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS (rw) register accessor: an alias for `Reg<EVENTS_DEVMISS_SPEC>`"]
pub type EVENTS_DEVMISS = crate::Reg<events_devmiss::EVENTS_DEVMISS_SPEC>;
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "EVENTS_RSSIEND (rw) register accessor: an alias for `Reg<EVENTS_RSSIEND_SPEC>`"]
pub type EVENTS_RSSIEND = crate::Reg<events_rssiend::EVENTS_RSSIEND_SPEC>;
#[doc = "Sampling of receive signal strength complete"]
pub mod events_rssiend;
#[doc = "EVENTS_BCMATCH (rw) register accessor: an alias for `Reg<EVENTS_BCMATCH_SPEC>`"]
pub type EVENTS_BCMATCH = crate::Reg<events_bcmatch::EVENTS_BCMATCH_SPEC>;
#[doc = "Bit counter reached bit count value"]
pub mod events_bcmatch;
#[doc = "EVENTS_CRCOK (rw) register accessor: an alias for `Reg<EVENTS_CRCOK_SPEC>`"]
pub type EVENTS_CRCOK = crate::Reg<events_crcok::EVENTS_CRCOK_SPEC>;
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "EVENTS_CRCERROR (rw) register accessor: an alias for `Reg<EVENTS_CRCERROR_SPEC>`"]
pub type EVENTS_CRCERROR = crate::Reg<events_crcerror::EVENTS_CRCERROR_SPEC>;
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "EVENTS_FRAMESTART (rw) register accessor: an alias for `Reg<EVENTS_FRAMESTART_SPEC>`"]
pub type EVENTS_FRAMESTART = crate::Reg<events_framestart::EVENTS_FRAMESTART_SPEC>;
#[doc = "IEEE 802.15.4 length field received"]
pub mod events_framestart;
#[doc = "EVENTS_EDEND (rw) register accessor: an alias for `Reg<EVENTS_EDEND_SPEC>`"]
pub type EVENTS_EDEND = crate::Reg<events_edend::EVENTS_EDEND_SPEC>;
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
pub mod events_edend;
#[doc = "EVENTS_EDSTOPPED (rw) register accessor: an alias for `Reg<EVENTS_EDSTOPPED_SPEC>`"]
pub type EVENTS_EDSTOPPED = crate::Reg<events_edstopped::EVENTS_EDSTOPPED_SPEC>;
#[doc = "The sampling of energy detection has stopped"]
pub mod events_edstopped;
#[doc = "EVENTS_CCAIDLE (rw) register accessor: an alias for `Reg<EVENTS_CCAIDLE_SPEC>`"]
pub type EVENTS_CCAIDLE = crate::Reg<events_ccaidle::EVENTS_CCAIDLE_SPEC>;
#[doc = "Wireless medium in idle - clear to send"]
pub mod events_ccaidle;
#[doc = "EVENTS_CCABUSY (rw) register accessor: an alias for `Reg<EVENTS_CCABUSY_SPEC>`"]
pub type EVENTS_CCABUSY = crate::Reg<events_ccabusy::EVENTS_CCABUSY_SPEC>;
#[doc = "Wireless medium busy - do not send"]
pub mod events_ccabusy;
#[doc = "EVENTS_CCASTOPPED (rw) register accessor: an alias for `Reg<EVENTS_CCASTOPPED_SPEC>`"]
pub type EVENTS_CCASTOPPED = crate::Reg<events_ccastopped::EVENTS_CCASTOPPED_SPEC>;
#[doc = "The CCA has stopped"]
pub mod events_ccastopped;
#[doc = "EVENTS_RATEBOOST (rw) register accessor: an alias for `Reg<EVENTS_RATEBOOST_SPEC>`"]
pub type EVENTS_RATEBOOST = crate::Reg<events_rateboost::EVENTS_RATEBOOST_SPEC>;
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
pub mod events_rateboost;
#[doc = "EVENTS_TXREADY (rw) register accessor: an alias for `Reg<EVENTS_TXREADY_SPEC>`"]
pub type EVENTS_TXREADY = crate::Reg<events_txready::EVENTS_TXREADY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started TX path"]
pub mod events_txready;
#[doc = "EVENTS_RXREADY (rw) register accessor: an alias for `Reg<EVENTS_RXREADY_SPEC>`"]
pub type EVENTS_RXREADY = crate::Reg<events_rxready::EVENTS_RXREADY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started RX path"]
pub mod events_rxready;
#[doc = "EVENTS_MHRMATCH (rw) register accessor: an alias for `Reg<EVENTS_MHRMATCH_SPEC>`"]
pub type EVENTS_MHRMATCH = crate::Reg<events_mhrmatch::EVENTS_MHRMATCH_SPEC>;
#[doc = "MAC header match found"]
pub mod events_mhrmatch;
#[doc = "EVENTS_SYNC (rw) register accessor: an alias for `Reg<EVENTS_SYNC_SPEC>`"]
pub type EVENTS_SYNC = crate::Reg<events_sync::EVENTS_SYNC_SPEC>;
#[doc = "Preamble indicator"]
pub mod events_sync;
#[doc = "EVENTS_PHYEND (rw) register accessor: an alias for `Reg<EVENTS_PHYEND_SPEC>`"]
pub type EVENTS_PHYEND = crate::Reg<events_phyend::EVENTS_PHYEND_SPEC>;
#[doc = "Generated when last bit is sent on air, or received from air"]
pub mod events_phyend;
#[doc = "EVENTS_CTEPRESENT (rw) register accessor: an alias for `Reg<EVENTS_CTEPRESENT_SPEC>`"]
pub type EVENTS_CTEPRESENT = crate::Reg<events_ctepresent::EVENTS_CTEPRESENT_SPEC>;
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)"]
pub mod events_ctepresent;
#[doc = "PUBLISH_READY (rw) register accessor: an alias for `Reg<PUBLISH_READY_SPEC>`"]
pub type PUBLISH_READY = crate::Reg<publish_ready::PUBLISH_READY_SPEC>;
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "PUBLISH_ADDRESS (rw) register accessor: an alias for `Reg<PUBLISH_ADDRESS_SPEC>`"]
pub type PUBLISH_ADDRESS = crate::Reg<publish_address::PUBLISH_ADDRESS_SPEC>;
#[doc = "Publish configuration for event ADDRESS"]
pub mod publish_address;
#[doc = "PUBLISH_PAYLOAD (rw) register accessor: an alias for `Reg<PUBLISH_PAYLOAD_SPEC>`"]
pub type PUBLISH_PAYLOAD = crate::Reg<publish_payload::PUBLISH_PAYLOAD_SPEC>;
#[doc = "Publish configuration for event PAYLOAD"]
pub mod publish_payload;
#[doc = "PUBLISH_END (rw) register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_DISABLED (rw) register accessor: an alias for `Reg<PUBLISH_DISABLED_SPEC>`"]
pub type PUBLISH_DISABLED = crate::Reg<publish_disabled::PUBLISH_DISABLED_SPEC>;
#[doc = "Publish configuration for event DISABLED"]
pub mod publish_disabled;
#[doc = "PUBLISH_DEVMATCH (rw) register accessor: an alias for `Reg<PUBLISH_DEVMATCH_SPEC>`"]
pub type PUBLISH_DEVMATCH = crate::Reg<publish_devmatch::PUBLISH_DEVMATCH_SPEC>;
#[doc = "Publish configuration for event DEVMATCH"]
pub mod publish_devmatch;
#[doc = "PUBLISH_DEVMISS (rw) register accessor: an alias for `Reg<PUBLISH_DEVMISS_SPEC>`"]
pub type PUBLISH_DEVMISS = crate::Reg<publish_devmiss::PUBLISH_DEVMISS_SPEC>;
#[doc = "Publish configuration for event DEVMISS"]
pub mod publish_devmiss;
#[doc = "PUBLISH_RSSIEND (rw) register accessor: an alias for `Reg<PUBLISH_RSSIEND_SPEC>`"]
pub type PUBLISH_RSSIEND = crate::Reg<publish_rssiend::PUBLISH_RSSIEND_SPEC>;
#[doc = "Publish configuration for event RSSIEND"]
pub mod publish_rssiend;
#[doc = "PUBLISH_BCMATCH (rw) register accessor: an alias for `Reg<PUBLISH_BCMATCH_SPEC>`"]
pub type PUBLISH_BCMATCH = crate::Reg<publish_bcmatch::PUBLISH_BCMATCH_SPEC>;
#[doc = "Publish configuration for event BCMATCH"]
pub mod publish_bcmatch;
#[doc = "PUBLISH_CRCOK (rw) register accessor: an alias for `Reg<PUBLISH_CRCOK_SPEC>`"]
pub type PUBLISH_CRCOK = crate::Reg<publish_crcok::PUBLISH_CRCOK_SPEC>;
#[doc = "Publish configuration for event CRCOK"]
pub mod publish_crcok;
#[doc = "PUBLISH_CRCERROR (rw) register accessor: an alias for `Reg<PUBLISH_CRCERROR_SPEC>`"]
pub type PUBLISH_CRCERROR = crate::Reg<publish_crcerror::PUBLISH_CRCERROR_SPEC>;
#[doc = "Publish configuration for event CRCERROR"]
pub mod publish_crcerror;
#[doc = "PUBLISH_FRAMESTART (rw) register accessor: an alias for `Reg<PUBLISH_FRAMESTART_SPEC>`"]
pub type PUBLISH_FRAMESTART = crate::Reg<publish_framestart::PUBLISH_FRAMESTART_SPEC>;
#[doc = "Publish configuration for event FRAMESTART"]
pub mod publish_framestart;
#[doc = "PUBLISH_EDEND (rw) register accessor: an alias for `Reg<PUBLISH_EDEND_SPEC>`"]
pub type PUBLISH_EDEND = crate::Reg<publish_edend::PUBLISH_EDEND_SPEC>;
#[doc = "Publish configuration for event EDEND"]
pub mod publish_edend;
#[doc = "PUBLISH_EDSTOPPED (rw) register accessor: an alias for `Reg<PUBLISH_EDSTOPPED_SPEC>`"]
pub type PUBLISH_EDSTOPPED = crate::Reg<publish_edstopped::PUBLISH_EDSTOPPED_SPEC>;
#[doc = "Publish configuration for event EDSTOPPED"]
pub mod publish_edstopped;
#[doc = "PUBLISH_CCAIDLE (rw) register accessor: an alias for `Reg<PUBLISH_CCAIDLE_SPEC>`"]
pub type PUBLISH_CCAIDLE = crate::Reg<publish_ccaidle::PUBLISH_CCAIDLE_SPEC>;
#[doc = "Publish configuration for event CCAIDLE"]
pub mod publish_ccaidle;
#[doc = "PUBLISH_CCABUSY (rw) register accessor: an alias for `Reg<PUBLISH_CCABUSY_SPEC>`"]
pub type PUBLISH_CCABUSY = crate::Reg<publish_ccabusy::PUBLISH_CCABUSY_SPEC>;
#[doc = "Publish configuration for event CCABUSY"]
pub mod publish_ccabusy;
#[doc = "PUBLISH_CCASTOPPED (rw) register accessor: an alias for `Reg<PUBLISH_CCASTOPPED_SPEC>`"]
pub type PUBLISH_CCASTOPPED = crate::Reg<publish_ccastopped::PUBLISH_CCASTOPPED_SPEC>;
#[doc = "Publish configuration for event CCASTOPPED"]
pub mod publish_ccastopped;
#[doc = "PUBLISH_RATEBOOST (rw) register accessor: an alias for `Reg<PUBLISH_RATEBOOST_SPEC>`"]
pub type PUBLISH_RATEBOOST = crate::Reg<publish_rateboost::PUBLISH_RATEBOOST_SPEC>;
#[doc = "Publish configuration for event RATEBOOST"]
pub mod publish_rateboost;
#[doc = "PUBLISH_TXREADY (rw) register accessor: an alias for `Reg<PUBLISH_TXREADY_SPEC>`"]
pub type PUBLISH_TXREADY = crate::Reg<publish_txready::PUBLISH_TXREADY_SPEC>;
#[doc = "Publish configuration for event TXREADY"]
pub mod publish_txready;
#[doc = "PUBLISH_RXREADY (rw) register accessor: an alias for `Reg<PUBLISH_RXREADY_SPEC>`"]
pub type PUBLISH_RXREADY = crate::Reg<publish_rxready::PUBLISH_RXREADY_SPEC>;
#[doc = "Publish configuration for event RXREADY"]
pub mod publish_rxready;
#[doc = "PUBLISH_MHRMATCH (rw) register accessor: an alias for `Reg<PUBLISH_MHRMATCH_SPEC>`"]
pub type PUBLISH_MHRMATCH = crate::Reg<publish_mhrmatch::PUBLISH_MHRMATCH_SPEC>;
#[doc = "Publish configuration for event MHRMATCH"]
pub mod publish_mhrmatch;
#[doc = "PUBLISH_SYNC (rw) register accessor: an alias for `Reg<PUBLISH_SYNC_SPEC>`"]
pub type PUBLISH_SYNC = crate::Reg<publish_sync::PUBLISH_SYNC_SPEC>;
#[doc = "Publish configuration for event SYNC"]
pub mod publish_sync;
#[doc = "PUBLISH_PHYEND (rw) register accessor: an alias for `Reg<PUBLISH_PHYEND_SPEC>`"]
pub type PUBLISH_PHYEND = crate::Reg<publish_phyend::PUBLISH_PHYEND_SPEC>;
#[doc = "Publish configuration for event PHYEND"]
pub mod publish_phyend;
#[doc = "PUBLISH_CTEPRESENT (rw) register accessor: an alias for `Reg<PUBLISH_CTEPRESENT_SPEC>`"]
pub type PUBLISH_CTEPRESENT = crate::Reg<publish_ctepresent::PUBLISH_CTEPRESENT_SPEC>;
#[doc = "Publish configuration for event CTEPRESENT"]
pub mod publish_ctepresent;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CRCSTATUS (r) register accessor: an alias for `Reg<CRCSTATUS_SPEC>`"]
pub type CRCSTATUS = crate::Reg<crcstatus::CRCSTATUS_SPEC>;
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "RXMATCH (r) register accessor: an alias for `Reg<RXMATCH_SPEC>`"]
pub type RXMATCH = crate::Reg<rxmatch::RXMATCH_SPEC>;
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "RXCRC (r) register accessor: an alias for `Reg<RXCRC_SPEC>`"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "DAI (r) register accessor: an alias for `Reg<DAI_SPEC>`"]
pub type DAI = crate::Reg<dai::DAI_SPEC>;
#[doc = "Device address match index"]
pub mod dai;
#[doc = "PDUSTAT (r) register accessor: an alias for `Reg<PDUSTAT_SPEC>`"]
pub type PDUSTAT = crate::Reg<pdustat::PDUSTAT_SPEC>;
#[doc = "Payload status"]
pub mod pdustat;
#[doc = "CTESTATUS (r) register accessor: an alias for `Reg<CTESTATUS_SPEC>`"]
pub type CTESTATUS = crate::Reg<ctestatus::CTESTATUS_SPEC>;
#[doc = "CTEInfo parsed from received packet"]
pub mod ctestatus;
#[doc = "DFESTATUS (r) register accessor: an alias for `Reg<DFESTATUS_SPEC>`"]
pub type DFESTATUS = crate::Reg<dfestatus::DFESTATUS_SPEC>;
#[doc = "DFE status information"]
pub mod dfestatus;
#[doc = "PACKETPTR (rw) register accessor: an alias for `Reg<PACKETPTR_SPEC>`"]
pub type PACKETPTR = crate::Reg<packetptr::PACKETPTR_SPEC>;
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "Frequency"]
pub mod frequency;
#[doc = "TXPOWER (rw) register accessor: an alias for `Reg<TXPOWER_SPEC>`"]
pub type TXPOWER = crate::Reg<txpower::TXPOWER_SPEC>;
#[doc = "Output power"]
pub mod txpower;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "PCNF0 (rw) register accessor: an alias for `Reg<PCNF0_SPEC>`"]
pub type PCNF0 = crate::Reg<pcnf0::PCNF0_SPEC>;
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "PCNF1 (rw) register accessor: an alias for `Reg<PCNF1_SPEC>`"]
pub type PCNF1 = crate::Reg<pcnf1::PCNF1_SPEC>;
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "BASE0 (rw) register accessor: an alias for `Reg<BASE0_SPEC>`"]
pub type BASE0 = crate::Reg<base0::BASE0_SPEC>;
#[doc = "Base address 0"]
pub mod base0;
#[doc = "BASE1 (rw) register accessor: an alias for `Reg<BASE1_SPEC>`"]
pub type BASE1 = crate::Reg<base1::BASE1_SPEC>;
#[doc = "Base address 1"]
pub mod base1;
#[doc = "PREFIX0 (rw) register accessor: an alias for `Reg<PREFIX0_SPEC>`"]
pub type PREFIX0 = crate::Reg<prefix0::PREFIX0_SPEC>;
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "PREFIX1 (rw) register accessor: an alias for `Reg<PREFIX1_SPEC>`"]
pub type PREFIX1 = crate::Reg<prefix1::PREFIX1_SPEC>;
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "TXADDRESS (rw) register accessor: an alias for `Reg<TXADDRESS_SPEC>`"]
pub type TXADDRESS = crate::Reg<txaddress::TXADDRESS_SPEC>;
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "RXADDRESSES (rw) register accessor: an alias for `Reg<RXADDRESSES_SPEC>`"]
pub type RXADDRESSES = crate::Reg<rxaddresses::RXADDRESSES_SPEC>;
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRCCNF (rw) register accessor: an alias for `Reg<CRCCNF_SPEC>`"]
pub type CRCCNF = crate::Reg<crccnf::CRCCNF_SPEC>;
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRCPOLY (rw) register accessor: an alias for `Reg<CRCPOLY_SPEC>`"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRCINIT (rw) register accessor: an alias for `Reg<CRCINIT_SPEC>`"]
pub type CRCINIT = crate::Reg<crcinit::CRCINIT_SPEC>;
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "TIFS (rw) register accessor: an alias for `Reg<TIFS_SPEC>`"]
pub type TIFS = crate::Reg<tifs::TIFS_SPEC>;
#[doc = "Interframe spacing in us"]
pub mod tifs;
#[doc = "RSSISAMPLE (r) register accessor: an alias for `Reg<RSSISAMPLE_SPEC>`"]
pub type RSSISAMPLE = crate::Reg<rssisample::RSSISAMPLE_SPEC>;
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Current radio state"]
pub mod state;
#[doc = "DATAWHITEIV (rw) register accessor: an alias for `Reg<DATAWHITEIV_SPEC>`"]
pub type DATAWHITEIV = crate::Reg<datawhiteiv::DATAWHITEIV_SPEC>;
#[doc = "Data whitening initial value"]
pub mod datawhiteiv;
#[doc = "BCC (rw) register accessor: an alias for `Reg<BCC_SPEC>`"]
pub type BCC = crate::Reg<bcc::BCC_SPEC>;
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "DAB (rw) register accessor: an alias for `Reg<DAB_SPEC>`"]
pub type DAB = crate::Reg<dab::DAB_SPEC>;
#[doc = "Description collection: Device address base segment n"]
pub mod dab;
#[doc = "DAP (rw) register accessor: an alias for `Reg<DAP_SPEC>`"]
pub type DAP = crate::Reg<dap::DAP_SPEC>;
#[doc = "Description collection: Device address prefix n"]
pub mod dap;
#[doc = "DACNF (rw) register accessor: an alias for `Reg<DACNF_SPEC>`"]
pub type DACNF = crate::Reg<dacnf::DACNF_SPEC>;
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "MHRMATCHCONF (rw) register accessor: an alias for `Reg<MHRMATCHCONF_SPEC>`"]
pub type MHRMATCHCONF = crate::Reg<mhrmatchconf::MHRMATCHCONF_SPEC>;
#[doc = "Search pattern configuration"]
pub mod mhrmatchconf;
#[doc = "MHRMATCHMAS (rw) register accessor: an alias for `Reg<MHRMATCHMAS_SPEC>`"]
pub type MHRMATCHMAS = crate::Reg<mhrmatchmas::MHRMATCHMAS_SPEC>;
#[doc = "Pattern mask"]
pub mod mhrmatchmas;
#[doc = "MODECNF0 (rw) register accessor: an alias for `Reg<MODECNF0_SPEC>`"]
pub type MODECNF0 = crate::Reg<modecnf0::MODECNF0_SPEC>;
#[doc = "Radio mode configuration register 0"]
pub mod modecnf0;
#[doc = "SFD (rw) register accessor: an alias for `Reg<SFD_SPEC>`"]
pub type SFD = crate::Reg<sfd::SFD_SPEC>;
#[doc = "IEEE 802.15.4 start of frame delimiter"]
pub mod sfd;
#[doc = "EDCNT (rw) register accessor: an alias for `Reg<EDCNT_SPEC>`"]
pub type EDCNT = crate::Reg<edcnt::EDCNT_SPEC>;
#[doc = "IEEE 802.15.4 energy detect loop count"]
pub mod edcnt;
#[doc = "EDSAMPLE (r) register accessor: an alias for `Reg<EDSAMPLE_SPEC>`"]
pub type EDSAMPLE = crate::Reg<edsample::EDSAMPLE_SPEC>;
#[doc = "IEEE 802.15.4 energy detect level"]
pub mod edsample;
#[doc = "CCACTRL (rw) register accessor: an alias for `Reg<CCACTRL_SPEC>`"]
pub type CCACTRL = crate::Reg<ccactrl::CCACTRL_SPEC>;
#[doc = "IEEE 802.15.4 clear channel assessment control"]
pub mod ccactrl;
#[doc = "DFEMODE (rw) register accessor: an alias for `Reg<DFEMODE_SPEC>`"]
pub type DFEMODE = crate::Reg<dfemode::DFEMODE_SPEC>;
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
pub mod dfemode;
#[doc = "CTEINLINECONF (rw) register accessor: an alias for `Reg<CTEINLINECONF_SPEC>`"]
pub type CTEINLINECONF = crate::Reg<cteinlineconf::CTEINLINECONF_SPEC>;
#[doc = "Configuration for CTE inline mode"]
pub mod cteinlineconf;
#[doc = "DFECTRL1 (rw) register accessor: an alias for `Reg<DFECTRL1_SPEC>`"]
pub type DFECTRL1 = crate::Reg<dfectrl1::DFECTRL1_SPEC>;
#[doc = "Various configuration for Direction finding"]
pub mod dfectrl1;
#[doc = "DFECTRL2 (rw) register accessor: an alias for `Reg<DFECTRL2_SPEC>`"]
pub type DFECTRL2 = crate::Reg<dfectrl2::DFECTRL2_SPEC>;
#[doc = "Start offset for Direction finding"]
pub mod dfectrl2;
#[doc = "SWITCHPATTERN (rw) register accessor: an alias for `Reg<SWITCHPATTERN_SPEC>`"]
pub type SWITCHPATTERN = crate::Reg<switchpattern::SWITCHPATTERN_SPEC>;
#[doc = "GPIO patterns to be used for each antenna"]
pub mod switchpattern;
#[doc = "CLEARPATTERN (rw) register accessor: an alias for `Reg<CLEARPATTERN_SPEC>`"]
pub type CLEARPATTERN = crate::Reg<clearpattern::CLEARPATTERN_SPEC>;
#[doc = "Clear the GPIO pattern array for antenna control"]
pub mod clearpattern;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "DFE packet EasyDMA channel"]
pub use dfepacket::DFEPACKET;
#[doc = r"Cluster"]
#[doc = "DFE packet EasyDMA channel"]
pub mod dfepacket;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control"]
pub mod power;
