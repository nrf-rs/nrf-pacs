#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start all trace and debug clocks."]
    pub tasks_clockstart: TASKS_CLOCKSTART,
    #[doc = "0x04 - Stop all trace and debug clocks."]
    pub tasks_clockstop: TASKS_CLOCKSTOP,
    _reserved2: [u8; 0x04f8],
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    pub enable: ENABLE,
    #[doc = "0x504..0x518 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x518 - Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
    pub traceportspeed: TRACEPORTSPEED,
}
#[doc = "TASKS_CLOCKSTART (w) register accessor: an alias for `Reg<TASKS_CLOCKSTART_SPEC>`"]
pub type TASKS_CLOCKSTART = crate::Reg<tasks_clockstart::TASKS_CLOCKSTART_SPEC>;
#[doc = "Start all trace and debug clocks."]
pub mod tasks_clockstart;
#[doc = "TASKS_CLOCKSTOP (w) register accessor: an alias for `Reg<TASKS_CLOCKSTOP_SPEC>`"]
pub type TASKS_CLOCKSTOP = crate::Reg<tasks_clockstop::TASKS_CLOCKSTOP_SPEC>;
#[doc = "Stop all trace and debug clocks."]
pub mod tasks_clockstop;
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
