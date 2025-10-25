#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0xf8],
    events_datardy: EventsDatardy,
    _reserved3: [u8; 0x0200],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0x01fc],
    temp: Temp,
    _reserved6: [u8; 0x0af0],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Start temperature measurement."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop temperature measurement."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x100 - Temperature measurement complete, data ready event."]
    #[inline(always)]
    pub const fn events_datardy(&self) -> &EventsDatardy {
        &self.events_datardy
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
    #[doc = "0x508 - Die temperature in degC, 2's complement format, 0.25 degC pecision."]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_START (w) register accessor: Start temperature measurement.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start temperature measurement."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop temperature measurement.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop temperature measurement."]
pub mod tasks_stop;
#[doc = "EVENTS_DATARDY (rw) register accessor: Temperature measurement complete, data ready event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_datardy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_datardy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_datardy`] module"]
#[doc(alias = "EVENTS_DATARDY")]
pub type EventsDatardy = crate::Reg<events_datardy::EventsDatardySpec>;
#[doc = "Temperature measurement complete, data ready event."]
pub mod events_datardy;
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
#[doc = "TEMP (r) register accessor: Die temperature in degC, 2's complement format, 0.25 degC pecision.\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`] module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision."]
pub mod temp;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
