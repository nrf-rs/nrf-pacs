#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_out: [TasksOut; 8],
    _reserved1: [u8; 0x10],
    tasks_set: [TasksSet; 8],
    _reserved2: [u8; 0x10],
    tasks_clr: [TasksClr; 8],
    _reserved3: [u8; 0x80],
    events_in: [EventsIn; 8],
    _reserved4: [u8; 0x5c],
    events_port: EventsPort,
    _reserved5: [u8; 0x0184],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved7: [u8; 0x0204],
    config: [Config; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
    #[inline(always)]
    pub const fn tasks_out(&self, n: usize) -> &TasksOut {
        &self.tasks_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
    #[inline(always)]
    pub fn tasks_out_iter(&self) -> impl Iterator<Item = &TasksOut> {
        self.tasks_out.iter()
    }
    #[doc = "0x30..0x50 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub const fn tasks_set(&self, n: usize) -> &TasksSet {
        &self.tasks_set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x50 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub fn tasks_set_iter(&self) -> impl Iterator<Item = &TasksSet> {
        self.tasks_set.iter()
    }
    #[doc = "0x60..0x80 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub const fn tasks_clr(&self, n: usize) -> &TasksClr {
        &self.tasks_clr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub fn tasks_clr_iter(&self) -> impl Iterator<Item = &TasksClr> {
        self.tasks_clr.iter()
    }
    #[doc = "0x100..0x120 - Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
    #[inline(always)]
    pub const fn events_in(&self, n: usize) -> &EventsIn {
        &self.events_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
    #[inline(always)]
    pub fn events_in_iter(&self) -> impl Iterator<Item = &EventsIn> {
        self.events_in.iter()
    }
    #[doc = "0x17c - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub const fn events_port(&self) -> &EventsPort {
        &self.events_port
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x510..0x530 - Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x530 - Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
}
#[doc = "TASKS_OUT (w) register accessor: Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_out::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_out`] module"]
#[doc(alias = "TASKS_OUT")]
pub type TasksOut = crate::Reg<tasks_out::TasksOutSpec>;
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
pub mod tasks_out;
#[doc = "TASKS_SET (w) register accessor: Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_set`] module"]
#[doc(alias = "TASKS_SET")]
pub type TasksSet = crate::Reg<tasks_set::TasksSetSpec>;
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "TASKS_CLR (w) register accessor: Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clr`] module"]
#[doc(alias = "TASKS_CLR")]
pub type TasksClr = crate::Reg<tasks_clr::TasksClrSpec>;
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "EVENTS_IN (rw) register accessor: Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`events_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_in`] module"]
#[doc(alias = "EVENTS_IN")]
pub type EventsIn = crate::Reg<events_in::EventsInSpec>;
#[doc = "Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
pub mod events_in;
#[doc = "EVENTS_PORT (rw) register accessor: Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nYou can [`read`](crate::Reg::read) this register and get [`events_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_port`] module"]
#[doc(alias = "EVENTS_PORT")]
pub type EventsPort = crate::Reg<events_port::EventsPortSpec>;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub mod events_port;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CONFIG (rw) register accessor: Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
pub mod config;
