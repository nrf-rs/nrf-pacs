#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    _reserved1: [u8; 0x04],
    tasks_stop: TasksStop,
    _reserved2: [u8; 0xf4],
    events_end: EventsEnd,
    events_resolved: EventsResolved,
    events_notresolved: EventsNotresolved,
    _reserved5: [u8; 0x01f8],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved7: [u8; 0xf4],
    status: Status,
    _reserved8: [u8; 0xfc],
    enable: Enable,
    nirk: Nirk,
    irkptr: Irkptr,
    _reserved11: [u8; 0x04],
    addrptr: Addrptr,
    scratchptr: Scratchptr,
    _reserved13: [u8; 0x0ae4],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x08 - Stop resolving addresses."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x100 - Address resolution procedure completed."]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x104 - Address resolved."]
    #[inline(always)]
    pub const fn events_resolved(&self) -> &EventsResolved {
        &self.events_resolved
    }
    #[doc = "0x108 - Address not resolved."]
    #[inline(always)]
    pub const fn events_notresolved(&self) -> &EventsNotresolved {
        &self.events_notresolved
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
    #[doc = "0x400 - Resolution status."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Enable AAR."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Number of Identity root Keys in the IRK data structure."]
    #[inline(always)]
    pub const fn nirk(&self) -> &Nirk {
        &self.nirk
    }
    #[doc = "0x508 - Pointer to the IRK data structure."]
    #[inline(always)]
    pub const fn irkptr(&self) -> &Irkptr {
        &self.irkptr
    }
    #[doc = "0x510 - Pointer to the resolvable address (6 bytes)."]
    #[inline(always)]
    pub const fn addrptr(&self) -> &Addrptr {
        &self.addrptr
    }
    #[doc = "0x514 - Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
    #[inline(always)]
    pub const fn scratchptr(&self) -> &Scratchptr {
        &self.scratchptr
    }
    #[doc = "0xffc - Peripheral power control."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_START (w) register accessor: Start resolving addresses based on IRKs specified in the IRK data structure.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop resolving addresses.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop resolving addresses."]
pub mod tasks_stop;
#[doc = "EVENTS_END (rw) register accessor: Address resolution procedure completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`] module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Address resolution procedure completed."]
pub mod events_end;
#[doc = "EVENTS_RESOLVED (rw) register accessor: Address resolved.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_resolved`] module"]
#[doc(alias = "EVENTS_RESOLVED")]
pub type EventsResolved = crate::Reg<events_resolved::EventsResolvedSpec>;
#[doc = "Address resolved."]
pub mod events_resolved;
#[doc = "EVENTS_NOTRESOLVED (rw) register accessor: Address not resolved.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_notresolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_notresolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_notresolved`] module"]
#[doc(alias = "EVENTS_NOTRESOLVED")]
pub type EventsNotresolved = crate::Reg<events_notresolved::EventsNotresolvedSpec>;
#[doc = "Address not resolved."]
pub mod events_notresolved;
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
#[doc = "STATUS (r) register accessor: Resolution status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Resolution status."]
pub mod status;
#[doc = "ENABLE (rw) register accessor: Enable AAR.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable AAR."]
pub mod enable;
#[doc = "NIRK (rw) register accessor: Number of Identity root Keys in the IRK data structure.\n\nYou can [`read`](crate::Reg::read) this register and get [`nirk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nirk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nirk`] module"]
#[doc(alias = "NIRK")]
pub type Nirk = crate::Reg<nirk::NirkSpec>;
#[doc = "Number of Identity root Keys in the IRK data structure."]
pub mod nirk;
#[doc = "IRKPTR (rw) register accessor: Pointer to the IRK data structure.\n\nYou can [`read`](crate::Reg::read) this register and get [`irkptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irkptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irkptr`] module"]
#[doc(alias = "IRKPTR")]
pub type Irkptr = crate::Reg<irkptr::IrkptrSpec>;
#[doc = "Pointer to the IRK data structure."]
pub mod irkptr;
#[doc = "ADDRPTR (rw) register accessor: Pointer to the resolvable address (6 bytes).\n\nYou can [`read`](crate::Reg::read) this register and get [`addrptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrptr`] module"]
#[doc(alias = "ADDRPTR")]
pub type Addrptr = crate::Reg<addrptr::AddrptrSpec>;
#[doc = "Pointer to the resolvable address (6 bytes)."]
pub mod addrptr;
#[doc = "SCRATCHPTR (rw) register accessor: Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`scratchptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratchptr`] module"]
#[doc(alias = "SCRATCHPTR")]
pub type Scratchptr = crate::Reg<scratchptr::ScratchptrSpec>;
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
pub mod scratchptr;
#[doc = "POWER (rw) register accessor: Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control."]
pub mod power;
