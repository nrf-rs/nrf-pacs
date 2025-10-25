#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_out: [TasksOut; 4],
    _reserved1: [u8; 0xf0],
    events_in: [EventsIn; 4],
    _reserved2: [u8; 0x6c],
    events_port: EventsPort,
    _reserved3: [u8; 0x0184],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0x0204],
    config: [Config; 4],
    _reserved6: [u8; 0x0adc],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Tasks asssociated with GPIOTE channels."]
    #[inline(always)]
    pub const fn tasks_out(&self, n: usize) -> &TasksOut {
        &self.tasks_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Tasks asssociated with GPIOTE channels."]
    #[inline(always)]
    pub fn tasks_out_iter(&self) -> impl Iterator<Item = &TasksOut> {
        self.tasks_out.iter()
    }
    #[doc = "0x100..0x110 - Tasks asssociated with GPIOTE channels."]
    #[inline(always)]
    pub const fn events_in(&self, n: usize) -> &EventsIn {
        &self.events_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Tasks asssociated with GPIOTE channels."]
    #[inline(always)]
    pub fn events_in_iter(&self) -> impl Iterator<Item = &EventsIn> {
        self.events_in.iter()
    }
    #[doc = "0x17c - Event generated from multiple pins."]
    #[inline(always)]
    pub const fn events_port(&self) -> &EventsPort {
        &self.events_port
    }
    #[doc = "0x304 - Interrupt enable set register."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Interrupt enable clear register."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x510..0x520 - Channel configuration registers."]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x520 - Channel configuration registers."]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_OUT (w) register accessor: Tasks asssociated with GPIOTE channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_out::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_out`] module"]
#[doc(alias = "TASKS_OUT")]
pub type TasksOut = crate::Reg<tasks_out::TasksOutSpec>;
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod tasks_out;
#[doc = "EVENTS_IN (rw) register accessor: Tasks asssociated with GPIOTE channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_in`] module"]
#[doc(alias = "EVENTS_IN")]
pub type EventsIn = crate::Reg<events_in::EventsInSpec>;
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod events_in;
#[doc = "EVENTS_PORT (rw) register accessor: Event generated from multiple pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_port`] module"]
#[doc(alias = "EVENTS_PORT")]
pub type EventsPort = crate::Reg<events_port::EventsPortSpec>;
#[doc = "Event generated from multiple pins."]
pub mod events_port;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "CONFIG (rw) register accessor: Channel configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Channel configuration registers."]
pub mod config;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
