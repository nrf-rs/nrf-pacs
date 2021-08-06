#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Start all trace and debug clocks."]
    pub clockstart: crate::Reg<clockstart::CLOCKSTART_SPEC>,
    #[doc = "0x08 - Stop all trace and debug clocks."]
    pub clockstop: crate::Reg<clockstop::CLOCKSTOP_SPEC>,
    _reserved2: [u8; 0x04f4],
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504..0x518 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x518 - Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
    pub traceportspeed: crate::Reg<traceportspeed::TRACEPORTSPEED_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin configuration for TRACECLK"]
    pub traceclk: crate::Reg<self::psel::traceclk::TRACECLK_SPEC>,
    #[doc = "0x04 - Pin configuration for TRACEDATA\\[0\\]"]
    pub tracedata0: crate::Reg<self::psel::tracedata0::TRACEDATA0_SPEC>,
    #[doc = "0x08 - Pin configuration for TRACEDATA\\[1\\]"]
    pub tracedata1: crate::Reg<self::psel::tracedata1::TRACEDATA1_SPEC>,
    #[doc = "0x0c - Pin configuration for TRACEDATA\\[2\\]"]
    pub tracedata2: crate::Reg<self::psel::tracedata2::TRACEDATA2_SPEC>,
    #[doc = "0x10 - Pin configuration for TRACEDATA\\[3\\]"]
    pub tracedata3: crate::Reg<self::psel::tracedata3::TRACEDATA3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "CLOCKSTART register accessor: an alias for `Reg<CLOCKSTART_SPEC>`"]
pub type CLOCKSTART = crate::Reg<clockstart::CLOCKSTART_SPEC>;
#[doc = "Start all trace and debug clocks."]
pub mod clockstart;
#[doc = "CLOCKSTOP register accessor: an alias for `Reg<CLOCKSTOP_SPEC>`"]
pub type CLOCKSTOP = crate::Reg<clockstop::CLOCKSTOP_SPEC>;
#[doc = "Stop all trace and debug clocks."]
pub mod clockstop;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "TRACEPORTSPEED register accessor: an alias for `Reg<TRACEPORTSPEED_SPEC>`"]
pub type TRACEPORTSPEED = crate::Reg<traceportspeed::TRACEPORTSPEED_SPEC>;
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
pub mod traceportspeed;
