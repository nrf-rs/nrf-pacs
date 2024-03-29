#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable radio in TX mode."]
    pub tasks_txen: TASKS_TXEN,
    #[doc = "0x04 - Enable radio in RX mode."]
    pub tasks_rxen: TASKS_RXEN,
    #[doc = "0x08 - Start radio."]
    pub tasks_start: TASKS_START,
    #[doc = "0x0c - Stop radio."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x10 - Disable radio."]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x14 - Start the RSSI and take one sample of the receive signal strength."]
    pub tasks_rssistart: TASKS_RSSISTART,
    #[doc = "0x18 - Stop the RSSI measurement."]
    pub tasks_rssistop: TASKS_RSSISTOP,
    #[doc = "0x1c - Start the bit counter."]
    pub tasks_bcstart: TASKS_BCSTART,
    #[doc = "0x20 - Stop the bit counter."]
    pub tasks_bcstop: TASKS_BCSTOP,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100 - Ready event."]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Address event."]
    pub events_address: EVENTS_ADDRESS,
    #[doc = "0x108 - Payload event."]
    pub events_payload: EVENTS_PAYLOAD,
    #[doc = "0x10c - End event."]
    pub events_end: EVENTS_END,
    #[doc = "0x110 - Disable event."]
    pub events_disabled: EVENTS_DISABLED,
    #[doc = "0x114 - A device address match occurred on the last received packet."]
    pub events_devmatch: EVENTS_DEVMATCH,
    #[doc = "0x118 - No device address match occurred on the last received packet."]
    pub events_devmiss: EVENTS_DEVMISS,
    #[doc = "0x11c - Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
    pub events_rssiend: EVENTS_RSSIEND,
    _reserved17: [u8; 0x08],
    #[doc = "0x128 - Bit counter reached bit count value specified in BCC register."]
    pub events_bcmatch: EVENTS_BCMATCH,
    _reserved18: [u8; 0xd4],
    #[doc = "0x200 - Shortcuts for the radio."]
    pub shorts: SHORTS,
    _reserved19: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved21: [u8; 0xf4],
    #[doc = "0x400 - CRC status of received packet."]
    pub crcstatus: CRCSTATUS,
    _reserved22: [u8; 0x04],
    #[doc = "0x408 - Received address."]
    pub rxmatch: RXMATCH,
    #[doc = "0x40c - Received CRC."]
    pub rxcrc: RXCRC,
    #[doc = "0x410 - Device address match index."]
    pub dai: DAI,
    _reserved25: [u8; 0xf0],
    #[doc = "0x504 - Packet pointer. Decision point: START task."]
    pub packetptr: PACKETPTR,
    #[doc = "0x508 - Frequency."]
    pub frequency: FREQUENCY,
    #[doc = "0x50c - Output power."]
    pub txpower: TXPOWER,
    #[doc = "0x510 - Data rate and modulation."]
    pub mode: MODE,
    #[doc = "0x514 - Packet configuration 0."]
    pub pcnf0: PCNF0,
    #[doc = "0x518 - Packet configuration 1."]
    pub pcnf1: PCNF1,
    #[doc = "0x51c - Radio base address 0. Decision point: START task."]
    pub base0: BASE0,
    #[doc = "0x520 - Radio base address 1. Decision point: START task."]
    pub base1: BASE1,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0 to 3."]
    pub prefix0: PREFIX0,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4 to 7."]
    pub prefix1: PREFIX1,
    #[doc = "0x52c - Transmit address select."]
    pub txaddress: TXADDRESS,
    #[doc = "0x530 - Receive address select."]
    pub rxaddresses: RXADDRESSES,
    #[doc = "0x534 - CRC configuration."]
    pub crccnf: CRCCNF,
    #[doc = "0x538 - CRC polynomial."]
    pub crcpoly: CRCPOLY,
    #[doc = "0x53c - CRC initial value."]
    pub crcinit: CRCINIT,
    #[doc = "0x540 - Test features enable register."]
    pub test: TEST,
    #[doc = "0x544 - Inter Frame Spacing in microseconds."]
    pub tifs: TIFS,
    #[doc = "0x548 - RSSI sample."]
    pub rssisample: RSSISAMPLE,
    _reserved43: [u8; 0x04],
    #[doc = "0x550 - Current radio state."]
    pub state: STATE,
    #[doc = "0x554 - Data whitening initial value."]
    pub datawhiteiv: DATAWHITEIV,
    _reserved45: [u8; 0x08],
    #[doc = "0x560 - Bit counter compare."]
    pub bcc: BCC,
    _reserved46: [u8; 0x9c],
    #[doc = "0x600..0x620 - Device address base segment."]
    pub dab: [DAB; 8],
    #[doc = "0x620..0x640 - Device address prefix."]
    pub dap: [DAP; 8],
    #[doc = "0x640 - Device address match configuration."]
    pub dacnf: DACNF,
    _reserved49: [u8; 0xe0],
    #[doc = "0x724 - Trim value override register 0."]
    pub override0: OVERRIDE0,
    #[doc = "0x728 - Trim value override register 1."]
    pub override1: OVERRIDE1,
    #[doc = "0x72c - Trim value override register 2."]
    pub override2: OVERRIDE2,
    #[doc = "0x730 - Trim value override register 3."]
    pub override3: OVERRIDE3,
    #[doc = "0x734 - Trim value override register 4."]
    pub override4: OVERRIDE4,
    _reserved54: [u8; 0x08c4],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_TXEN (w) register accessor: an alias for `Reg<TASKS_TXEN_SPEC>`"]
pub type TASKS_TXEN = crate::Reg<tasks_txen::TASKS_TXEN_SPEC>;
#[doc = "Enable radio in TX mode."]
pub mod tasks_txen;
#[doc = "TASKS_RXEN (w) register accessor: an alias for `Reg<TASKS_RXEN_SPEC>`"]
pub type TASKS_RXEN = crate::Reg<tasks_rxen::TASKS_RXEN_SPEC>;
#[doc = "Enable radio in RX mode."]
pub mod tasks_rxen;
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start radio."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop radio."]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE (w) register accessor: an alias for `Reg<TASKS_DISABLE_SPEC>`"]
pub type TASKS_DISABLE = crate::Reg<tasks_disable::TASKS_DISABLE_SPEC>;
#[doc = "Disable radio."]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART (w) register accessor: an alias for `Reg<TASKS_RSSISTART_SPEC>`"]
pub type TASKS_RSSISTART = crate::Reg<tasks_rssistart::TASKS_RSSISTART_SPEC>;
#[doc = "Start the RSSI and take one sample of the receive signal strength."]
pub mod tasks_rssistart;
#[doc = "TASKS_RSSISTOP (w) register accessor: an alias for `Reg<TASKS_RSSISTOP_SPEC>`"]
pub type TASKS_RSSISTOP = crate::Reg<tasks_rssistop::TASKS_RSSISTOP_SPEC>;
#[doc = "Stop the RSSI measurement."]
pub mod tasks_rssistop;
#[doc = "TASKS_BCSTART (w) register accessor: an alias for `Reg<TASKS_BCSTART_SPEC>`"]
pub type TASKS_BCSTART = crate::Reg<tasks_bcstart::TASKS_BCSTART_SPEC>;
#[doc = "Start the bit counter."]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP (w) register accessor: an alias for `Reg<TASKS_BCSTOP_SPEC>`"]
pub type TASKS_BCSTOP = crate::Reg<tasks_bcstop::TASKS_BCSTOP_SPEC>;
#[doc = "Stop the bit counter."]
pub mod tasks_bcstop;
#[doc = "EVENTS_READY (rw) register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "Ready event."]
pub mod events_ready;
#[doc = "EVENTS_ADDRESS (rw) register accessor: an alias for `Reg<EVENTS_ADDRESS_SPEC>`"]
pub type EVENTS_ADDRESS = crate::Reg<events_address::EVENTS_ADDRESS_SPEC>;
#[doc = "Address event."]
pub mod events_address;
#[doc = "EVENTS_PAYLOAD (rw) register accessor: an alias for `Reg<EVENTS_PAYLOAD_SPEC>`"]
pub type EVENTS_PAYLOAD = crate::Reg<events_payload::EVENTS_PAYLOAD_SPEC>;
#[doc = "Payload event."]
pub mod events_payload;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "End event."]
pub mod events_end;
#[doc = "EVENTS_DISABLED (rw) register accessor: an alias for `Reg<EVENTS_DISABLED_SPEC>`"]
pub type EVENTS_DISABLED = crate::Reg<events_disabled::EVENTS_DISABLED_SPEC>;
#[doc = "Disable event."]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH (rw) register accessor: an alias for `Reg<EVENTS_DEVMATCH_SPEC>`"]
pub type EVENTS_DEVMATCH = crate::Reg<events_devmatch::EVENTS_DEVMATCH_SPEC>;
#[doc = "A device address match occurred on the last received packet."]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS (rw) register accessor: an alias for `Reg<EVENTS_DEVMISS_SPEC>`"]
pub type EVENTS_DEVMISS = crate::Reg<events_devmiss::EVENTS_DEVMISS_SPEC>;
#[doc = "No device address match occurred on the last received packet."]
pub mod events_devmiss;
#[doc = "EVENTS_RSSIEND (rw) register accessor: an alias for `Reg<EVENTS_RSSIEND_SPEC>`"]
pub type EVENTS_RSSIEND = crate::Reg<events_rssiend::EVENTS_RSSIEND_SPEC>;
#[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
pub mod events_rssiend;
#[doc = "EVENTS_BCMATCH (rw) register accessor: an alias for `Reg<EVENTS_BCMATCH_SPEC>`"]
pub type EVENTS_BCMATCH = crate::Reg<events_bcmatch::EVENTS_BCMATCH_SPEC>;
#[doc = "Bit counter reached bit count value specified in BCC register."]
pub mod events_bcmatch;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for the radio."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "CRCSTATUS (r) register accessor: an alias for `Reg<CRCSTATUS_SPEC>`"]
pub type CRCSTATUS = crate::Reg<crcstatus::CRCSTATUS_SPEC>;
#[doc = "CRC status of received packet."]
pub mod crcstatus;
#[doc = "RXMATCH (r) register accessor: an alias for `Reg<RXMATCH_SPEC>`"]
pub type RXMATCH = crate::Reg<rxmatch::RXMATCH_SPEC>;
#[doc = "Received address."]
pub mod rxmatch;
#[doc = "RXCRC (r) register accessor: an alias for `Reg<RXCRC_SPEC>`"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "Received CRC."]
pub mod rxcrc;
#[doc = "DAI (r) register accessor: an alias for `Reg<DAI_SPEC>`"]
pub type DAI = crate::Reg<dai::DAI_SPEC>;
#[doc = "Device address match index."]
pub mod dai;
#[doc = "PACKETPTR (rw) register accessor: an alias for `Reg<PACKETPTR_SPEC>`"]
pub type PACKETPTR = crate::Reg<packetptr::PACKETPTR_SPEC>;
#[doc = "Packet pointer. Decision point: START task."]
pub mod packetptr;
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "Frequency."]
pub mod frequency;
#[doc = "TXPOWER (rw) register accessor: an alias for `Reg<TXPOWER_SPEC>`"]
pub type TXPOWER = crate::Reg<txpower::TXPOWER_SPEC>;
#[doc = "Output power."]
pub mod txpower;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Data rate and modulation."]
pub mod mode;
#[doc = "PCNF0 (rw) register accessor: an alias for `Reg<PCNF0_SPEC>`"]
pub type PCNF0 = crate::Reg<pcnf0::PCNF0_SPEC>;
#[doc = "Packet configuration 0."]
pub mod pcnf0;
#[doc = "PCNF1 (rw) register accessor: an alias for `Reg<PCNF1_SPEC>`"]
pub type PCNF1 = crate::Reg<pcnf1::PCNF1_SPEC>;
#[doc = "Packet configuration 1."]
pub mod pcnf1;
#[doc = "BASE0 (rw) register accessor: an alias for `Reg<BASE0_SPEC>`"]
pub type BASE0 = crate::Reg<base0::BASE0_SPEC>;
#[doc = "Radio base address 0. Decision point: START task."]
pub mod base0;
#[doc = "BASE1 (rw) register accessor: an alias for `Reg<BASE1_SPEC>`"]
pub type BASE1 = crate::Reg<base1::BASE1_SPEC>;
#[doc = "Radio base address 1. Decision point: START task."]
pub mod base1;
#[doc = "PREFIX0 (rw) register accessor: an alias for `Reg<PREFIX0_SPEC>`"]
pub type PREFIX0 = crate::Reg<prefix0::PREFIX0_SPEC>;
#[doc = "Prefixes bytes for logical addresses 0 to 3."]
pub mod prefix0;
#[doc = "PREFIX1 (rw) register accessor: an alias for `Reg<PREFIX1_SPEC>`"]
pub type PREFIX1 = crate::Reg<prefix1::PREFIX1_SPEC>;
#[doc = "Prefixes bytes for logical addresses 4 to 7."]
pub mod prefix1;
#[doc = "TXADDRESS (rw) register accessor: an alias for `Reg<TXADDRESS_SPEC>`"]
pub type TXADDRESS = crate::Reg<txaddress::TXADDRESS_SPEC>;
#[doc = "Transmit address select."]
pub mod txaddress;
#[doc = "RXADDRESSES (rw) register accessor: an alias for `Reg<RXADDRESSES_SPEC>`"]
pub type RXADDRESSES = crate::Reg<rxaddresses::RXADDRESSES_SPEC>;
#[doc = "Receive address select."]
pub mod rxaddresses;
#[doc = "CRCCNF (rw) register accessor: an alias for `Reg<CRCCNF_SPEC>`"]
pub type CRCCNF = crate::Reg<crccnf::CRCCNF_SPEC>;
#[doc = "CRC configuration."]
pub mod crccnf;
#[doc = "CRCPOLY (rw) register accessor: an alias for `Reg<CRCPOLY_SPEC>`"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "CRC polynomial."]
pub mod crcpoly;
#[doc = "CRCINIT (rw) register accessor: an alias for `Reg<CRCINIT_SPEC>`"]
pub type CRCINIT = crate::Reg<crcinit::CRCINIT_SPEC>;
#[doc = "CRC initial value."]
pub mod crcinit;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test features enable register."]
pub mod test;
#[doc = "TIFS (rw) register accessor: an alias for `Reg<TIFS_SPEC>`"]
pub type TIFS = crate::Reg<tifs::TIFS_SPEC>;
#[doc = "Inter Frame Spacing in microseconds."]
pub mod tifs;
#[doc = "RSSISAMPLE (r) register accessor: an alias for `Reg<RSSISAMPLE_SPEC>`"]
pub type RSSISAMPLE = crate::Reg<rssisample::RSSISAMPLE_SPEC>;
#[doc = "RSSI sample."]
pub mod rssisample;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Current radio state."]
pub mod state;
#[doc = "DATAWHITEIV (rw) register accessor: an alias for `Reg<DATAWHITEIV_SPEC>`"]
pub type DATAWHITEIV = crate::Reg<datawhiteiv::DATAWHITEIV_SPEC>;
#[doc = "Data whitening initial value."]
pub mod datawhiteiv;
#[doc = "BCC (rw) register accessor: an alias for `Reg<BCC_SPEC>`"]
pub type BCC = crate::Reg<bcc::BCC_SPEC>;
#[doc = "Bit counter compare."]
pub mod bcc;
#[doc = "DAB (rw) register accessor: an alias for `Reg<DAB_SPEC>`"]
pub type DAB = crate::Reg<dab::DAB_SPEC>;
#[doc = "Device address base segment."]
pub mod dab;
#[doc = "DAP (rw) register accessor: an alias for `Reg<DAP_SPEC>`"]
pub type DAP = crate::Reg<dap::DAP_SPEC>;
#[doc = "Device address prefix."]
pub mod dap;
#[doc = "DACNF (rw) register accessor: an alias for `Reg<DACNF_SPEC>`"]
pub type DACNF = crate::Reg<dacnf::DACNF_SPEC>;
#[doc = "Device address match configuration."]
pub mod dacnf;
#[doc = "OVERRIDE0 (rw) register accessor: an alias for `Reg<OVERRIDE0_SPEC>`"]
pub type OVERRIDE0 = crate::Reg<override0::OVERRIDE0_SPEC>;
#[doc = "Trim value override register 0."]
pub mod override0;
#[doc = "OVERRIDE1 (rw) register accessor: an alias for `Reg<OVERRIDE1_SPEC>`"]
pub type OVERRIDE1 = crate::Reg<override1::OVERRIDE1_SPEC>;
#[doc = "Trim value override register 1."]
pub mod override1;
#[doc = "OVERRIDE2 (rw) register accessor: an alias for `Reg<OVERRIDE2_SPEC>`"]
pub type OVERRIDE2 = crate::Reg<override2::OVERRIDE2_SPEC>;
#[doc = "Trim value override register 2."]
pub mod override2;
#[doc = "OVERRIDE3 (rw) register accessor: an alias for `Reg<OVERRIDE3_SPEC>`"]
pub type OVERRIDE3 = crate::Reg<override3::OVERRIDE3_SPEC>;
#[doc = "Trim value override register 3."]
pub mod override3;
#[doc = "OVERRIDE4 (rw) register accessor: an alias for `Reg<OVERRIDE4_SPEC>`"]
pub type OVERRIDE4 = crate::Reg<override4::OVERRIDE4_SPEC>;
#[doc = "Trim value override register 4."]
pub mod override4;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
