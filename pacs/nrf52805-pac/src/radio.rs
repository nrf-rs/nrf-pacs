#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    pub tasks_txen: crate::Reg<tasks_txen::TASKS_TXEN_SPEC>,
    #[doc = "0x04 - Enable RADIO in RX mode"]
    pub tasks_rxen: crate::Reg<tasks_rxen::TASKS_RXEN_SPEC>,
    #[doc = "0x08 - Start RADIO"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x0c - Stop RADIO"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x10 - Disable RADIO"]
    pub tasks_disable: crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>,
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength"]
    pub tasks_rssistart: crate::Reg<tasks_rssistart::TASKS_RSSISTART_SPEC>,
    #[doc = "0x18 - Stop the RSSI measurement"]
    pub tasks_rssistop: crate::Reg<tasks_rssistop::TASKS_RSSISTOP_SPEC>,
    #[doc = "0x1c - Start the bit counter"]
    pub tasks_bcstart: crate::Reg<tasks_bcstart::TASKS_BCSTART_SPEC>,
    #[doc = "0x20 - Stop the bit counter"]
    pub tasks_bcstop: crate::Reg<tasks_bcstop::TASKS_BCSTOP_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100 - RADIO has ramped up and is ready to be started"]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    #[doc = "0x104 - Address sent or received"]
    pub events_address: crate::Reg<events_address::EVENTS_ADDRESS_SPEC>,
    #[doc = "0x108 - Packet payload sent or received"]
    pub events_payload: crate::Reg<events_payload::EVENTS_PAYLOAD_SPEC>,
    #[doc = "0x10c - Packet sent or received"]
    pub events_end: crate::Reg<events_end::EVENTS_END_SPEC>,
    #[doc = "0x110 - RADIO has been disabled"]
    pub events_disabled: crate::Reg<events_disabled::EVENTS_DISABLED_SPEC>,
    #[doc = "0x114 - A device address match occurred on the last received packet"]
    pub events_devmatch: crate::Reg<events_devmatch::EVENTS_DEVMATCH_SPEC>,
    #[doc = "0x118 - No device address match occurred on the last received packet"]
    pub events_devmiss: crate::Reg<events_devmiss::EVENTS_DEVMISS_SPEC>,
    #[doc = "0x11c - Sampling of receive signal strength complete"]
    pub events_rssiend: crate::Reg<events_rssiend::EVENTS_RSSIEND_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x128 - Bit counter reached bit count value"]
    pub events_bcmatch: crate::Reg<events_bcmatch::EVENTS_BCMATCH_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x130 - Packet received with CRC ok"]
    pub events_crcok: crate::Reg<events_crcok::EVENTS_CRCOK_SPEC>,
    #[doc = "0x134 - Packet received with CRC error"]
    pub events_crcerror: crate::Reg<events_crcerror::EVENTS_CRCERROR_SPEC>,
    _reserved20: [u8; 0x1c],
    #[doc = "0x154 - RADIO has ramped up and is ready to be started TX path"]
    pub events_txready: crate::Reg<events_txready::EVENTS_TXREADY_SPEC>,
    #[doc = "0x158 - RADIO has ramped up and is ready to be started RX path"]
    pub events_rxready: crate::Reg<events_rxready::EVENTS_RXREADY_SPEC>,
    _reserved22: [u8; 0x10],
    #[doc = "0x16c - Generated when last bit is sent on air, or received from air"]
    pub events_phyend: crate::Reg<events_phyend::EVENTS_PHYEND_SPEC>,
    _reserved23: [u8; 0x90],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved24: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved26: [u8; 0xf4],
    #[doc = "0x400 - CRC status"]
    pub crcstatus: crate::Reg<crcstatus::CRCSTATUS_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x408 - Received address"]
    pub rxmatch: crate::Reg<rxmatch::RXMATCH_SPEC>,
    #[doc = "0x40c - CRC field of previously received packet"]
    pub rxcrc: crate::Reg<rxcrc::RXCRC_SPEC>,
    #[doc = "0x410 - Device address match index"]
    pub dai: crate::Reg<dai::DAI_SPEC>,
    #[doc = "0x414 - Payload status"]
    pub pdustat: crate::Reg<pdustat::PDUSTAT_SPEC>,
    _reserved31: [u8; 0xec],
    #[doc = "0x504 - Packet pointer"]
    pub packetptr: crate::Reg<packetptr::PACKETPTR_SPEC>,
    #[doc = "0x508 - Frequency"]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    #[doc = "0x50c - Output power"]
    pub txpower: crate::Reg<txpower::TXPOWER_SPEC>,
    #[doc = "0x510 - Data rate and modulation"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x514 - Packet configuration register 0"]
    pub pcnf0: crate::Reg<pcnf0::PCNF0_SPEC>,
    #[doc = "0x518 - Packet configuration register 1"]
    pub pcnf1: crate::Reg<pcnf1::PCNF1_SPEC>,
    #[doc = "0x51c - Base address 0"]
    pub base0: crate::Reg<base0::BASE0_SPEC>,
    #[doc = "0x520 - Base address 1"]
    pub base1: crate::Reg<base1::BASE1_SPEC>,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0-3"]
    pub prefix0: crate::Reg<prefix0::PREFIX0_SPEC>,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4-7"]
    pub prefix1: crate::Reg<prefix1::PREFIX1_SPEC>,
    #[doc = "0x52c - Transmit address select"]
    pub txaddress: crate::Reg<txaddress::TXADDRESS_SPEC>,
    #[doc = "0x530 - Receive address select"]
    pub rxaddresses: crate::Reg<rxaddresses::RXADDRESSES_SPEC>,
    #[doc = "0x534 - CRC configuration"]
    pub crccnf: crate::Reg<crccnf::CRCCNF_SPEC>,
    #[doc = "0x538 - CRC polynomial"]
    pub crcpoly: crate::Reg<crcpoly::CRCPOLY_SPEC>,
    #[doc = "0x53c - CRC initial value"]
    pub crcinit: crate::Reg<crcinit::CRCINIT_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x544 - Interframe spacing in us"]
    pub tifs: crate::Reg<tifs::TIFS_SPEC>,
    #[doc = "0x548 - RSSI sample"]
    pub rssisample: crate::Reg<rssisample::RSSISAMPLE_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0x550 - Current radio state"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x554 - Data whitening initial value"]
    pub datawhiteiv: crate::Reg<datawhiteiv::DATAWHITEIV_SPEC>,
    _reserved50: [u8; 0x08],
    #[doc = "0x560 - Bit counter compare"]
    pub bcc: crate::Reg<bcc::BCC_SPEC>,
    _reserved51: [u8; 0x9c],
    #[doc = "0x600..0x620 - Description collection: Device address base segment n"]
    pub dab: [crate::Reg<dab::DAB_SPEC>; 8],
    #[doc = "0x620..0x640 - Description collection: Device address prefix n"]
    pub dap: [crate::Reg<dap::DAP_SPEC>; 8],
    #[doc = "0x640 - Device address match configuration"]
    pub dacnf: crate::Reg<dacnf::DACNF_SPEC>,
    _reserved54: [u8; 0x0c],
    #[doc = "0x650 - Radio mode configuration register 0"]
    pub modecnf0: crate::Reg<modecnf0::MODECNF0_SPEC>,
    _reserved55: [u8; 0x09a8],
    #[doc = "0xffc - Peripheral power control"]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_TXEN register accessor: an alias for `Reg<TASKS_TXEN_SPEC>`"]
pub type TASKS_TXEN = crate::Reg<tasks_txen::TASKS_TXEN_SPEC>;
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "TASKS_RXEN register accessor: an alias for `Reg<TASKS_RXEN_SPEC>`"]
pub type TASKS_RXEN = crate::Reg<tasks_rxen::TASKS_RXEN_SPEC>;
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE register accessor: an alias for `Reg<TASKS_DISABLE_SPEC>`"]
pub type TASKS_DISABLE = crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>;
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART register accessor: an alias for `Reg<TASKS_RSSISTART_SPEC>`"]
pub type TASKS_RSSISTART = crate::Reg<tasks_rssistart::TASKS_RSSISTART_SPEC>;
#[doc = "Start the RSSI and take one single sample of the receive signal strength"]
pub mod tasks_rssistart;
#[doc = "TASKS_RSSISTOP register accessor: an alias for `Reg<TASKS_RSSISTOP_SPEC>`"]
pub type TASKS_RSSISTOP = crate::Reg<tasks_rssistop::TASKS_RSSISTOP_SPEC>;
#[doc = "Stop the RSSI measurement"]
pub mod tasks_rssistop;
#[doc = "TASKS_BCSTART register accessor: an alias for `Reg<TASKS_BCSTART_SPEC>`"]
pub type TASKS_BCSTART = crate::Reg<tasks_bcstart::TASKS_BCSTART_SPEC>;
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP register accessor: an alias for `Reg<TASKS_BCSTOP_SPEC>`"]
pub type TASKS_BCSTOP = crate::Reg<tasks_bcstop::TASKS_BCSTOP_SPEC>;
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "EVENTS_ADDRESS register accessor: an alias for `Reg<EVENTS_ADDRESS_SPEC>`"]
pub type EVENTS_ADDRESS = crate::Reg<events_address::EVENTS_ADDRESS_SPEC>;
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "EVENTS_PAYLOAD register accessor: an alias for `Reg<EVENTS_PAYLOAD_SPEC>`"]
pub type EVENTS_PAYLOAD = crate::Reg<events_payload::EVENTS_PAYLOAD_SPEC>;
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "EVENTS_END register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Packet sent or received"]
pub mod events_end;
#[doc = "EVENTS_DISABLED register accessor: an alias for `Reg<EVENTS_DISABLED_SPEC>`"]
pub type EVENTS_DISABLED = crate::Reg<events_disabled::EVENTS_DISABLED_SPEC>;
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH register accessor: an alias for `Reg<EVENTS_DEVMATCH_SPEC>`"]
pub type EVENTS_DEVMATCH = crate::Reg<events_devmatch::EVENTS_DEVMATCH_SPEC>;
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS register accessor: an alias for `Reg<EVENTS_DEVMISS_SPEC>`"]
pub type EVENTS_DEVMISS = crate::Reg<events_devmiss::EVENTS_DEVMISS_SPEC>;
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "EVENTS_RSSIEND register accessor: an alias for `Reg<EVENTS_RSSIEND_SPEC>`"]
pub type EVENTS_RSSIEND = crate::Reg<events_rssiend::EVENTS_RSSIEND_SPEC>;
#[doc = "Sampling of receive signal strength complete"]
pub mod events_rssiend;
#[doc = "EVENTS_BCMATCH register accessor: an alias for `Reg<EVENTS_BCMATCH_SPEC>`"]
pub type EVENTS_BCMATCH = crate::Reg<events_bcmatch::EVENTS_BCMATCH_SPEC>;
#[doc = "Bit counter reached bit count value"]
pub mod events_bcmatch;
#[doc = "EVENTS_CRCOK register accessor: an alias for `Reg<EVENTS_CRCOK_SPEC>`"]
pub type EVENTS_CRCOK = crate::Reg<events_crcok::EVENTS_CRCOK_SPEC>;
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "EVENTS_CRCERROR register accessor: an alias for `Reg<EVENTS_CRCERROR_SPEC>`"]
pub type EVENTS_CRCERROR = crate::Reg<events_crcerror::EVENTS_CRCERROR_SPEC>;
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "EVENTS_TXREADY register accessor: an alias for `Reg<EVENTS_TXREADY_SPEC>`"]
pub type EVENTS_TXREADY = crate::Reg<events_txready::EVENTS_TXREADY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started TX path"]
pub mod events_txready;
#[doc = "EVENTS_RXREADY register accessor: an alias for `Reg<EVENTS_RXREADY_SPEC>`"]
pub type EVENTS_RXREADY = crate::Reg<events_rxready::EVENTS_RXREADY_SPEC>;
#[doc = "RADIO has ramped up and is ready to be started RX path"]
pub mod events_rxready;
#[doc = "EVENTS_PHYEND register accessor: an alias for `Reg<EVENTS_PHYEND_SPEC>`"]
pub type EVENTS_PHYEND = crate::Reg<events_phyend::EVENTS_PHYEND_SPEC>;
#[doc = "Generated when last bit is sent on air, or received from air"]
pub mod events_phyend;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CRCSTATUS register accessor: an alias for `Reg<CRCSTATUS_SPEC>`"]
pub type CRCSTATUS = crate::Reg<crcstatus::CRCSTATUS_SPEC>;
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "RXMATCH register accessor: an alias for `Reg<RXMATCH_SPEC>`"]
pub type RXMATCH = crate::Reg<rxmatch::RXMATCH_SPEC>;
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "RXCRC register accessor: an alias for `Reg<RXCRC_SPEC>`"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "DAI register accessor: an alias for `Reg<DAI_SPEC>`"]
pub type DAI = crate::Reg<dai::DAI_SPEC>;
#[doc = "Device address match index"]
pub mod dai;
#[doc = "PDUSTAT register accessor: an alias for `Reg<PDUSTAT_SPEC>`"]
pub type PDUSTAT = crate::Reg<pdustat::PDUSTAT_SPEC>;
#[doc = "Payload status"]
pub mod pdustat;
#[doc = "PACKETPTR register accessor: an alias for `Reg<PACKETPTR_SPEC>`"]
pub type PACKETPTR = crate::Reg<packetptr::PACKETPTR_SPEC>;
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "Frequency"]
pub mod frequency;
#[doc = "TXPOWER register accessor: an alias for `Reg<TXPOWER_SPEC>`"]
pub type TXPOWER = crate::Reg<txpower::TXPOWER_SPEC>;
#[doc = "Output power"]
pub mod txpower;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "PCNF0 register accessor: an alias for `Reg<PCNF0_SPEC>`"]
pub type PCNF0 = crate::Reg<pcnf0::PCNF0_SPEC>;
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "PCNF1 register accessor: an alias for `Reg<PCNF1_SPEC>`"]
pub type PCNF1 = crate::Reg<pcnf1::PCNF1_SPEC>;
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "BASE0 register accessor: an alias for `Reg<BASE0_SPEC>`"]
pub type BASE0 = crate::Reg<base0::BASE0_SPEC>;
#[doc = "Base address 0"]
pub mod base0;
#[doc = "BASE1 register accessor: an alias for `Reg<BASE1_SPEC>`"]
pub type BASE1 = crate::Reg<base1::BASE1_SPEC>;
#[doc = "Base address 1"]
pub mod base1;
#[doc = "PREFIX0 register accessor: an alias for `Reg<PREFIX0_SPEC>`"]
pub type PREFIX0 = crate::Reg<prefix0::PREFIX0_SPEC>;
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "PREFIX1 register accessor: an alias for `Reg<PREFIX1_SPEC>`"]
pub type PREFIX1 = crate::Reg<prefix1::PREFIX1_SPEC>;
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "TXADDRESS register accessor: an alias for `Reg<TXADDRESS_SPEC>`"]
pub type TXADDRESS = crate::Reg<txaddress::TXADDRESS_SPEC>;
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "RXADDRESSES register accessor: an alias for `Reg<RXADDRESSES_SPEC>`"]
pub type RXADDRESSES = crate::Reg<rxaddresses::RXADDRESSES_SPEC>;
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRCCNF register accessor: an alias for `Reg<CRCCNF_SPEC>`"]
pub type CRCCNF = crate::Reg<crccnf::CRCCNF_SPEC>;
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRCPOLY register accessor: an alias for `Reg<CRCPOLY_SPEC>`"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRCINIT register accessor: an alias for `Reg<CRCINIT_SPEC>`"]
pub type CRCINIT = crate::Reg<crcinit::CRCINIT_SPEC>;
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "TIFS register accessor: an alias for `Reg<TIFS_SPEC>`"]
pub type TIFS = crate::Reg<tifs::TIFS_SPEC>;
#[doc = "Interframe spacing in us"]
pub mod tifs;
#[doc = "RSSISAMPLE register accessor: an alias for `Reg<RSSISAMPLE_SPEC>`"]
pub type RSSISAMPLE = crate::Reg<rssisample::RSSISAMPLE_SPEC>;
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Current radio state"]
pub mod state;
#[doc = "DATAWHITEIV register accessor: an alias for `Reg<DATAWHITEIV_SPEC>`"]
pub type DATAWHITEIV = crate::Reg<datawhiteiv::DATAWHITEIV_SPEC>;
#[doc = "Data whitening initial value"]
pub mod datawhiteiv;
#[doc = "BCC register accessor: an alias for `Reg<BCC_SPEC>`"]
pub type BCC = crate::Reg<bcc::BCC_SPEC>;
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "DAB register accessor: an alias for `Reg<DAB_SPEC>`"]
pub type DAB = crate::Reg<dab::DAB_SPEC>;
#[doc = "Description collection: Device address base segment n"]
pub mod dab;
#[doc = "DAP register accessor: an alias for `Reg<DAP_SPEC>`"]
pub type DAP = crate::Reg<dap::DAP_SPEC>;
#[doc = "Description collection: Device address prefix n"]
pub mod dap;
#[doc = "DACNF register accessor: an alias for `Reg<DACNF_SPEC>`"]
pub type DACNF = crate::Reg<dacnf::DACNF_SPEC>;
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "MODECNF0 register accessor: an alias for `Reg<MODECNF0_SPEC>`"]
pub type MODECNF0 = crate::Reg<modecnf0::MODECNF0_SPEC>;
#[doc = "Radio mode configuration register 0"]
pub mod modecnf0;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control"]
pub mod power;
