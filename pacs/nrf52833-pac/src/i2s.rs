#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0xfc],
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    pub events_rxptrupd: crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>,
    #[doc = "0x108 - I2S transfer stopped."]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    pub events_txptrupd: crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>,
    _reserved5: [u8; 0x01e8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved8: [u8; 0x01f4],
    #[doc = "0x500 - Enable I2S module."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504..0x52c - Unspecified"]
    pub config: CONFIG,
    _reserved10: [u8; 0x0c],
    #[doc = "0x538 - Unspecified"]
    pub rxd: RXD,
    _reserved11: [u8; 0x04],
    #[doc = "0x540 - Unspecified"]
    pub txd: TXD,
    _reserved12: [u8; 0x0c],
    #[doc = "0x550 - Unspecified"]
    pub rxtxd: RXTXD,
    _reserved13: [u8; 0x0c],
    #[doc = "0x560..0x574 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - I2S mode."]
    pub mode: crate::Reg<self::config::mode::MODE_SPEC>,
    #[doc = "0x04 - Reception (RX) enable."]
    pub rxen: crate::Reg<self::config::rxen::RXEN_SPEC>,
    #[doc = "0x08 - Transmission (TX) enable."]
    pub txen: crate::Reg<self::config::txen::TXEN_SPEC>,
    #[doc = "0x0c - Master clock generator enable."]
    pub mcken: crate::Reg<self::config::mcken::MCKEN_SPEC>,
    #[doc = "0x10 - Master clock generator frequency."]
    pub mckfreq: crate::Reg<self::config::mckfreq::MCKFREQ_SPEC>,
    #[doc = "0x14 - MCK / LRCK ratio."]
    pub ratio: crate::Reg<self::config::ratio::RATIO_SPEC>,
    #[doc = "0x18 - Sample width."]
    pub swidth: crate::Reg<self::config::swidth::SWIDTH_SPEC>,
    #[doc = "0x1c - Alignment of sample within a frame."]
    pub align: crate::Reg<self::config::align::ALIGN_SPEC>,
    #[doc = "0x20 - Frame format."]
    pub format: crate::Reg<self::config::format::FORMAT_SPEC>,
    #[doc = "0x24 - Enable channels."]
    pub channels: crate::Reg<self::config::channels::CHANNELS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Receive buffer RAM start address."]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Transmit buffer RAM start address."]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXTXD {
    #[doc = "0x00 - Size of RXD and TXD buffers."]
    pub maxcnt: crate::Reg<self::rxtxd::maxcnt::MAXCNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for MCK signal."]
    pub mck: crate::Reg<self::psel::mck::MCK_SPEC>,
    #[doc = "0x04 - Pin select for SCK signal."]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x08 - Pin select for LRCK signal."]
    pub lrck: crate::Reg<self::psel::lrck::LRCK_SPEC>,
    #[doc = "0x0c - Pin select for SDIN signal."]
    pub sdin: crate::Reg<self::psel::sdin::SDIN_SPEC>,
    #[doc = "0x10 - Pin select for SDOUT signal."]
    pub sdout: crate::Reg<self::psel::sdout::SDOUT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated."]
pub mod tasks_stop;
#[doc = "EVENTS_RXPTRUPD register accessor: an alias for `Reg<EVENTS_RXPTRUPD_SPEC>`"]
pub type EVENTS_RXPTRUPD = crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "EVENTS_TXPTRUPD register accessor: an alias for `Reg<EVENTS_TXPTRUPD_SPEC>`"]
pub type EVENTS_TXPTRUPD = crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
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
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable I2S module."]
pub mod enable;
