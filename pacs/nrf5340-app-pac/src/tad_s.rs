#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Start all trace and debug clocks."]
    pub clockstart: CLOCKSTART,
    #[doc = "0x08 - Stop all trace and debug clocks."]
    pub clockstop: CLOCKSTOP,
    _reserved2: [u8; 0x04f4],
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    pub enable: ENABLE,
    #[doc = "0x504..0x518 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x518 - Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
    pub traceportspeed: TRACEPORTSPEED,
}
#[doc = "CLOCKSTART (w) register accessor: an alias for `Reg<CLOCKSTART_SPEC>`"]
pub type CLOCKSTART = crate::Reg<clockstart::CLOCKSTART_SPEC>;
#[doc = "Start all trace and debug clocks."]
pub mod clockstart;
#[doc = "CLOCKSTOP (w) register accessor: an alias for `Reg<CLOCKSTOP_SPEC>`"]
pub type CLOCKSTOP = crate::Reg<clockstop::CLOCKSTOP_SPEC>;
#[doc = "Stop all trace and debug clocks."]
pub mod clockstop;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TRACEPORTSPEED (rw) register accessor: an alias for `Reg<TRACEPORTSPEED_SPEC>`"]
pub type TRACEPORTSPEED = crate::Reg<traceportspeed::TRACEPORTSPEED_SPEC>;
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
pub mod traceportspeed;
