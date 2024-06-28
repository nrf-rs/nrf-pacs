#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_clockstart: TasksClockstart,
    tasks_clockstop: TasksClockstop,
    _reserved2: [u8; 0x04f8],
    enable: Enable,
    psel: Psel,
    traceportspeed: Traceportspeed,
}
impl RegisterBlock {
    #[doc = "0x00 - Start all trace and debug clocks."]
    #[inline(always)]
    pub const fn tasks_clockstart(&self) -> &TasksClockstart {
        &self.tasks_clockstart
    }
    #[doc = "0x04 - Stop all trace and debug clocks."]
    #[inline(always)]
    pub const fn tasks_clockstop(&self) -> &TasksClockstop {
        &self.tasks_clockstop
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
#[doc = "TASKS_CLOCKSTART (w) register accessor: Start all trace and debug clocks.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clockstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clockstart`]
module"]
#[doc(alias = "TASKS_CLOCKSTART")]
pub type TasksClockstart = crate::Reg<tasks_clockstart::TasksClockstartSpec>;
#[doc = "Start all trace and debug clocks."]
pub mod tasks_clockstart;
#[doc = "TASKS_CLOCKSTOP (w) register accessor: Stop all trace and debug clocks.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clockstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clockstop`]
module"]
#[doc(alias = "TASKS_CLOCKSTOP")]
pub type TasksClockstop = crate::Reg<tasks_clockstop::TasksClockstopSpec>;
#[doc = "Stop all trace and debug clocks."]
pub mod tasks_clockstop;
#[doc = "ENABLE (rw) register accessor: Enable debug domain and aquire selected GPIOs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TRACEPORTSPEED (rw) register accessor: Clocking options for the Trace Port debug interface Reset behavior is the same as debug components\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceportspeed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceportspeed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceportspeed`]
module"]
#[doc(alias = "TRACEPORTSPEED")]
pub type Traceportspeed = crate::Reg<traceportspeed::TraceportspeedSpec>;
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
pub mod traceportspeed;
