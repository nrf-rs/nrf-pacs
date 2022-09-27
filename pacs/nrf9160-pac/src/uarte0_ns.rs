#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver"]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter"]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter"]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved4: [u8; 0x1c],
    #[doc = "0x2c - Flush RX FIFO into RX buffer"]
    pub tasks_flushrx: TASKS_FLUSHRX,
    _reserved5: [u8; 0x50],
    #[doc = "0x80 - Subscribe configuration for task STARTRX"]
    pub subscribe_startrx: SUBSCRIBE_STARTRX,
    #[doc = "0x84 - Subscribe configuration for task STOPRX"]
    pub subscribe_stoprx: SUBSCRIBE_STOPRX,
    #[doc = "0x88 - Subscribe configuration for task STARTTX"]
    pub subscribe_starttx: SUBSCRIBE_STARTTX,
    #[doc = "0x8c - Subscribe configuration for task STOPTX"]
    pub subscribe_stoptx: SUBSCRIBE_STOPTX,
    _reserved9: [u8; 0x1c],
    #[doc = "0xac - Subscribe configuration for task FLUSHRX"]
    pub subscribe_flushrx: SUBSCRIBE_FLUSHRX,
    _reserved10: [u8; 0x50],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved13: [u8; 0x04],
    #[doc = "0x110 - Receive buffer is filled up"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved14: [u8; 0x08],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    #[doc = "0x120 - Last TX byte transmitted"]
    pub events_endtx: EVENTS_ENDTX,
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved17: [u8; 0x1c],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved18: [u8; 0x04],
    #[doc = "0x14c - UART receiver has started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - UART transmitter has started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved20: [u8; 0x04],
    #[doc = "0x158 - Transmitter stopped"]
    pub events_txstopped: EVENTS_TXSTOPPED,
    _reserved21: [u8; 0x24],
    #[doc = "0x180 - Publish configuration for event CTS"]
    pub publish_cts: PUBLISH_CTS,
    #[doc = "0x184 - Publish configuration for event NCTS"]
    pub publish_ncts: PUBLISH_NCTS,
    #[doc = "0x188 - Publish configuration for event RXDRDY"]
    pub publish_rxdrdy: PUBLISH_RXDRDY,
    _reserved24: [u8; 0x04],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    _reserved25: [u8; 0x08],
    #[doc = "0x19c - Publish configuration for event TXDRDY"]
    pub publish_txdrdy: PUBLISH_TXDRDY,
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    pub publish_endtx: PUBLISH_ENDTX,
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved28: [u8; 0x1c],
    #[doc = "0x1c4 - Publish configuration for event RXTO"]
    pub publish_rxto: PUBLISH_RXTO,
    _reserved29: [u8; 0x04],
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    pub publish_rxstarted: PUBLISH_RXSTARTED,
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    pub publish_txstarted: PUBLISH_TXSTARTED,
    _reserved31: [u8; 0x04],
    #[doc = "0x1d8 - Publish configuration for event TXSTOPPED"]
    pub publish_txstopped: PUBLISH_TXSTOPPED,
    _reserved32: [u8; 0x24],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved33: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved36: [u8; 0x0174],
    #[doc = "0x480 - Error source Note : this register is read / write one to clear."]
    pub errorsrc: ERRORSRC,
    _reserved37: [u8; 0x7c],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved38: [u8; 0x04],
    #[doc = "0x508..0x518 - Unspecified"]
    pub psel: PSEL,
    _reserved39: [u8; 0x0c],
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    pub baudrate: BAUDRATE,
    _reserved40: [u8; 0x0c],
    #[doc = "0x534..0x540 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved41: [u8; 0x04],
    #[doc = "0x544..0x550 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved42: [u8; 0x1c],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = "TASKS_STARTRX (w) register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "TASKS_STOPRX (w) register accessor: an alias for `Reg<TASKS_STOPRX_SPEC>`"]
pub type TASKS_STOPRX = crate::Reg<tasks_stoprx::TASKS_STOPRX_SPEC>;
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "TASKS_STARTTX (w) register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "TASKS_STOPTX (w) register accessor: an alias for `Reg<TASKS_STOPTX_SPEC>`"]
pub type TASKS_STOPTX = crate::Reg<tasks_stoptx::TASKS_STOPTX_SPEC>;
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "TASKS_FLUSHRX (w) register accessor: an alias for `Reg<TASKS_FLUSHRX_SPEC>`"]
pub type TASKS_FLUSHRX = crate::Reg<tasks_flushrx::TASKS_FLUSHRX_SPEC>;
#[doc = "Flush RX FIFO into RX buffer"]
pub mod tasks_flushrx;
#[doc = "SUBSCRIBE_STARTRX (rw) register accessor: an alias for `Reg<SUBSCRIBE_STARTRX_SPEC>`"]
pub type SUBSCRIBE_STARTRX = crate::Reg<subscribe_startrx::SUBSCRIBE_STARTRX_SPEC>;
#[doc = "Subscribe configuration for task STARTRX"]
pub mod subscribe_startrx;
#[doc = "SUBSCRIBE_STOPRX (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOPRX_SPEC>`"]
pub type SUBSCRIBE_STOPRX = crate::Reg<subscribe_stoprx::SUBSCRIBE_STOPRX_SPEC>;
#[doc = "Subscribe configuration for task STOPRX"]
pub mod subscribe_stoprx;
#[doc = "SUBSCRIBE_STARTTX (rw) register accessor: an alias for `Reg<SUBSCRIBE_STARTTX_SPEC>`"]
pub type SUBSCRIBE_STARTTX = crate::Reg<subscribe_starttx::SUBSCRIBE_STARTTX_SPEC>;
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "SUBSCRIBE_STOPTX (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOPTX_SPEC>`"]
pub type SUBSCRIBE_STOPTX = crate::Reg<subscribe_stoptx::SUBSCRIBE_STOPTX_SPEC>;
#[doc = "Subscribe configuration for task STOPTX"]
pub mod subscribe_stoptx;
#[doc = "SUBSCRIBE_FLUSHRX (rw) register accessor: an alias for `Reg<SUBSCRIBE_FLUSHRX_SPEC>`"]
pub type SUBSCRIBE_FLUSHRX = crate::Reg<subscribe_flushrx::SUBSCRIBE_FLUSHRX_SPEC>;
#[doc = "Subscribe configuration for task FLUSHRX"]
pub mod subscribe_flushrx;
#[doc = "EVENTS_CTS (rw) register accessor: an alias for `Reg<EVENTS_CTS_SPEC>`"]
pub type EVENTS_CTS = crate::Reg<events_cts::EVENTS_CTS_SPEC>;
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "EVENTS_NCTS (rw) register accessor: an alias for `Reg<EVENTS_NCTS_SPEC>`"]
pub type EVENTS_NCTS = crate::Reg<events_ncts::EVENTS_NCTS_SPEC>;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "EVENTS_RXDRDY (rw) register accessor: an alias for `Reg<EVENTS_RXDRDY_SPEC>`"]
pub type EVENTS_RXDRDY = crate::Reg<events_rxdrdy::EVENTS_RXDRDY_SPEC>;
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub mod events_rxdrdy;
#[doc = "EVENTS_ENDRX (rw) register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "Receive buffer is filled up"]
pub mod events_endrx;
#[doc = "EVENTS_TXDRDY (rw) register accessor: an alias for `Reg<EVENTS_TXDRDY_SPEC>`"]
pub type EVENTS_TXDRDY = crate::Reg<events_txdrdy::EVENTS_TXDRDY_SPEC>;
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "EVENTS_ENDTX (rw) register accessor: an alias for `Reg<EVENTS_ENDTX_SPEC>`"]
pub type EVENTS_ENDTX = crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>;
#[doc = "Last TX byte transmitted"]
pub mod events_endtx;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Error detected"]
pub mod events_error;
#[doc = "EVENTS_RXTO (rw) register accessor: an alias for `Reg<EVENTS_RXTO_SPEC>`"]
pub type EVENTS_RXTO = crate::Reg<events_rxto::EVENTS_RXTO_SPEC>;
#[doc = "Receiver timeout"]
pub mod events_rxto;
#[doc = "EVENTS_RXSTARTED (rw) register accessor: an alias for `Reg<EVENTS_RXSTARTED_SPEC>`"]
pub type EVENTS_RXSTARTED = crate::Reg<events_rxstarted::EVENTS_RXSTARTED_SPEC>;
#[doc = "UART receiver has started"]
pub mod events_rxstarted;
#[doc = "EVENTS_TXSTARTED (rw) register accessor: an alias for `Reg<EVENTS_TXSTARTED_SPEC>`"]
pub type EVENTS_TXSTARTED = crate::Reg<events_txstarted::EVENTS_TXSTARTED_SPEC>;
#[doc = "UART transmitter has started"]
pub mod events_txstarted;
#[doc = "EVENTS_TXSTOPPED (rw) register accessor: an alias for `Reg<EVENTS_TXSTOPPED_SPEC>`"]
pub type EVENTS_TXSTOPPED = crate::Reg<events_txstopped::EVENTS_TXSTOPPED_SPEC>;
#[doc = "Transmitter stopped"]
pub mod events_txstopped;
#[doc = "PUBLISH_CTS (rw) register accessor: an alias for `Reg<PUBLISH_CTS_SPEC>`"]
pub type PUBLISH_CTS = crate::Reg<publish_cts::PUBLISH_CTS_SPEC>;
#[doc = "Publish configuration for event CTS"]
pub mod publish_cts;
#[doc = "PUBLISH_NCTS (rw) register accessor: an alias for `Reg<PUBLISH_NCTS_SPEC>`"]
pub type PUBLISH_NCTS = crate::Reg<publish_ncts::PUBLISH_NCTS_SPEC>;
#[doc = "Publish configuration for event NCTS"]
pub mod publish_ncts;
#[doc = "PUBLISH_RXDRDY (rw) register accessor: an alias for `Reg<PUBLISH_RXDRDY_SPEC>`"]
pub type PUBLISH_RXDRDY = crate::Reg<publish_rxdrdy::PUBLISH_RXDRDY_SPEC>;
#[doc = "Publish configuration for event RXDRDY"]
pub mod publish_rxdrdy;
#[doc = "PUBLISH_ENDRX (rw) register accessor: an alias for `Reg<PUBLISH_ENDRX_SPEC>`"]
pub type PUBLISH_ENDRX = crate::Reg<publish_endrx::PUBLISH_ENDRX_SPEC>;
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "PUBLISH_TXDRDY (rw) register accessor: an alias for `Reg<PUBLISH_TXDRDY_SPEC>`"]
pub type PUBLISH_TXDRDY = crate::Reg<publish_txdrdy::PUBLISH_TXDRDY_SPEC>;
#[doc = "Publish configuration for event TXDRDY"]
pub mod publish_txdrdy;
#[doc = "PUBLISH_ENDTX (rw) register accessor: an alias for `Reg<PUBLISH_ENDTX_SPEC>`"]
pub type PUBLISH_ENDTX = crate::Reg<publish_endtx::PUBLISH_ENDTX_SPEC>;
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "PUBLISH_ERROR (rw) register accessor: an alias for `Reg<PUBLISH_ERROR_SPEC>`"]
pub type PUBLISH_ERROR = crate::Reg<publish_error::PUBLISH_ERROR_SPEC>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_RXTO (rw) register accessor: an alias for `Reg<PUBLISH_RXTO_SPEC>`"]
pub type PUBLISH_RXTO = crate::Reg<publish_rxto::PUBLISH_RXTO_SPEC>;
#[doc = "Publish configuration for event RXTO"]
pub mod publish_rxto;
#[doc = "PUBLISH_RXSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_RXSTARTED_SPEC>`"]
pub type PUBLISH_RXSTARTED = crate::Reg<publish_rxstarted::PUBLISH_RXSTARTED_SPEC>;
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "PUBLISH_TXSTARTED (rw) register accessor: an alias for `Reg<PUBLISH_TXSTARTED_SPEC>`"]
pub type PUBLISH_TXSTARTED = crate::Reg<publish_txstarted::PUBLISH_TXSTARTED_SPEC>;
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "PUBLISH_TXSTOPPED (rw) register accessor: an alias for `Reg<PUBLISH_TXSTOPPED_SPEC>`"]
pub type PUBLISH_TXSTOPPED = crate::Reg<publish_txstopped::PUBLISH_TXSTOPPED_SPEC>;
#[doc = "Publish configuration for event TXSTOPPED"]
pub mod publish_txstopped;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ERRORSRC (rw) register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source Note : this register is read / write one to clear."]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "BAUDRATE (rw) register accessor: an alias for `Reg<BAUDRATE_SPEC>`"]
pub type BAUDRATE = crate::Reg<baudrate::BAUDRATE_SPEC>;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "RXD EasyDMA channel"]
pub use rxd::RXD;
#[doc = r"Cluster"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = "TXD EasyDMA channel"]
pub use txd::TXD;
#[doc = r"Cluster"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
