#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops I2S transfer and MCK generator. Triggering this task will cause the event STOPPED to be generated."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 0x7c],
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
    pub events_rxptrupd: EVENTS_RXPTRUPD,
    #[doc = "0x108 - I2S transfer stopped."]
    pub events_stopped: EVENTS_STOPPED,
    _reserved6: [u8; 0x08],
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    pub events_txptrupd: EVENTS_TXPTRUPD,
    _reserved7: [u8; 0x04],
    #[doc = "0x11c - Frame start event, generated on the active edge of LRCK"]
    pub events_framestart: EVENTS_FRAMESTART,
    _reserved8: [u8; 0x64],
    #[doc = "0x184 - Publish configuration for event RXPTRUPD"]
    pub publish_rxptrupd: PUBLISH_RXPTRUPD,
    #[doc = "0x188 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    _reserved10: [u8; 0x08],
    #[doc = "0x194 - Publish configuration for event TXPTRUPD"]
    pub publish_txptrupd: PUBLISH_TXPTRUPD,
    _reserved11: [u8; 0x04],
    #[doc = "0x19c - Publish configuration for event FRAMESTART"]
    pub publish_framestart: PUBLISH_FRAMESTART,
    _reserved12: [u8; 0x0160],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved15: [u8; 0x01f4],
    #[doc = "0x500 - Enable I2S module"]
    pub enable: ENABLE,
    #[doc = "0x504..0x530 - Unspecified"]
    pub config: CONFIG,
    _reserved17: [u8; 0x08],
    #[doc = "0x538 - Unspecified"]
    pub rxd: RXD,
    _reserved18: [u8; 0x04],
    #[doc = "0x540 - Unspecified"]
    pub txd: TXD,
    _reserved19: [u8; 0x0c],
    #[doc = "0x550 - Unspecified"]
    pub rxtxd: RXTXD,
    _reserved20: [u8; 0x0c],
    #[doc = "0x560..0x574 - Unspecified"]
    pub psel: PSEL,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops I2S transfer and MCK generator. Triggering this task will cause the event STOPPED to be generated."]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_RXPTRUPD (rw) register accessor: an alias for `Reg<EVENTS_RXPTRUPD_SPEC>`"]
pub type EVENTS_RXPTRUPD = crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "EVENTS_TXPTRUPD (rw) register accessor: an alias for `Reg<EVENTS_TXPTRUPD_SPEC>`"]
pub type EVENTS_TXPTRUPD = crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
#[doc = "EVENTS_FRAMESTART (rw) register accessor: an alias for `Reg<EVENTS_FRAMESTART_SPEC>`"]
pub type EVENTS_FRAMESTART = crate::Reg<events_framestart::EVENTS_FRAMESTART_SPEC>;
#[doc = "Frame start event, generated on the active edge of LRCK"]
pub mod events_framestart;
#[doc = "PUBLISH_RXPTRUPD (rw) register accessor: an alias for `Reg<PUBLISH_RXPTRUPD_SPEC>`"]
pub type PUBLISH_RXPTRUPD = crate::Reg<publish_rxptrupd::PUBLISH_RXPTRUPD_SPEC>;
#[doc = "Publish configuration for event RXPTRUPD"]
pub mod publish_rxptrupd;
#[doc = "PUBLISH_STOPPED (rw) register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_TXPTRUPD (rw) register accessor: an alias for `Reg<PUBLISH_TXPTRUPD_SPEC>`"]
pub type PUBLISH_TXPTRUPD = crate::Reg<publish_txptrupd::PUBLISH_TXPTRUPD_SPEC>;
#[doc = "Publish configuration for event TXPTRUPD"]
pub mod publish_txptrupd;
#[doc = "PUBLISH_FRAMESTART (rw) register accessor: an alias for `Reg<PUBLISH_FRAMESTART_SPEC>`"]
pub type PUBLISH_FRAMESTART = crate::Reg<publish_framestart::PUBLISH_FRAMESTART_SPEC>;
#[doc = "Publish configuration for event FRAMESTART"]
pub mod publish_framestart;
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
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable I2S module"]
pub mod enable;
#[doc = "Unspecified"]
pub use config::CONFIG;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
#[doc = "Unspecified"]
pub use rxd::RXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Unspecified"]
pub use txd::TXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Unspecified"]
pub use rxtxd::RXTXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
