#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    clockstart: Clockstart,
    clockstop: Clockstop,
    _reserved2: [u8; 0x04f4],
    enable: Enable,
    psel: Psel,
    traceportspeed: Traceportspeed,
}
impl RegisterBlock {
    #[doc = "0x04 - Start all trace and debug clocks."]
    #[inline(always)]
    pub const fn clockstart(&self) -> &Clockstart {
        &self.clockstart
    }
    #[doc = "0x08 - Stop all trace and debug clocks."]
    #[inline(always)]
    pub const fn clockstop(&self) -> &Clockstop {
        &self.clockstop
    }
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504..0x518 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x518 - Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
    #[inline(always)]
    pub const fn traceportspeed(&self) -> &Traceportspeed {
        &self.traceportspeed
    }
}
#[doc = "CLOCKSTART (w) register accessor: Start all trace and debug clocks.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockstart`] module"]
#[doc(alias = "CLOCKSTART")]
pub type Clockstart = crate::Reg<clockstart::ClockstartSpec>;
#[doc = "Start all trace and debug clocks."]
pub mod clockstart;
#[doc = "CLOCKSTOP (w) register accessor: Stop all trace and debug clocks.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockstop`] module"]
#[doc(alias = "CLOCKSTOP")]
pub type Clockstop = crate::Reg<clockstop::ClockstopSpec>;
#[doc = "Stop all trace and debug clocks."]
pub mod clockstop;
#[doc = "ENABLE (rw) register accessor: Enable debug domain and aquire selected GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TRACEPORTSPEED (rw) register accessor: Clocking options for the Trace Port debug interface Reset behavior is the same as debug components\n\nYou can [`read`](crate::Reg::read) this register and get [`traceportspeed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceportspeed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceportspeed`] module"]
#[doc(alias = "TRACEPORTSPEED")]
pub type Traceportspeed = crate::Reg<traceportspeed::TraceportspeedSpec>;
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
pub mod traceportspeed;
