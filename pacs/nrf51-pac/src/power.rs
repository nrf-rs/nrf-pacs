#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    tasks_constlat: TasksConstlat,
    tasks_lowpwr: TasksLowpwr,
    _reserved2: [u8; 0x88],
    events_pofwarn: EventsPofwarn,
    _reserved3: [u8; 0x01f8],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0xf4],
    resetreas: Resetreas,
    _reserved6: [u8; 0x24],
    ramstatus: Ramstatus,
    _reserved7: [u8; 0xd4],
    systemoff: Systemoff,
    _reserved8: [u8; 0x0c],
    pofcon: Pofcon,
    _reserved9: [u8; 0x08],
    gpregret: Gpregret,
    _reserved10: [u8; 0x04],
    ramon: Ramon,
    _reserved11: [u8; 0x1c],
    reset: Reset,
    _reserved12: [u8; 0x0c],
    ramonb: Ramonb,
    _reserved13: [u8; 0x20],
    dcdcen: Dcdcen,
    _reserved14: [u8; 0x048c],
    dcdcforce: Dcdcforce,
}
impl RegisterBlock {
    #[doc = "0x78 - Enable constant latency mode."]
    #[inline(always)]
    pub const fn tasks_constlat(&self) -> &TasksConstlat {
        &self.tasks_constlat
    }
    #[doc = "0x7c - Enable low power mode (variable latency)."]
    #[inline(always)]
    pub const fn tasks_lowpwr(&self) -> &TasksLowpwr {
        &self.tasks_lowpwr
    }
    #[doc = "0x108 - Power failure warning."]
    #[inline(always)]
    pub const fn events_pofwarn(&self) -> &EventsPofwarn {
        &self.events_pofwarn
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
    #[doc = "0x400 - Reset reason."]
    #[inline(always)]
    pub const fn resetreas(&self) -> &Resetreas {
        &self.resetreas
    }
    #[doc = "0x428 - Ram status register."]
    #[inline(always)]
    pub const fn ramstatus(&self) -> &Ramstatus {
        &self.ramstatus
    }
    #[doc = "0x500 - System off register."]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x510 - Power failure configuration."]
    #[inline(always)]
    pub const fn pofcon(&self) -> &Pofcon {
        &self.pofcon
    }
    #[doc = "0x51c - General purpose retention register. This register is a retained register."]
    #[inline(always)]
    pub const fn gpregret(&self) -> &Gpregret {
        &self.gpregret
    }
    #[doc = "0x524 - Ram on/off."]
    #[inline(always)]
    pub const fn ramon(&self) -> &Ramon {
        &self.ramon
    }
    #[doc = "0x544 - Pin reset functionality configuration register. This register is a retained register."]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
    #[doc = "0x554 - Ram on/off."]
    #[inline(always)]
    pub const fn ramonb(&self) -> &Ramonb {
        &self.ramonb
    }
    #[doc = "0x578 - DCDC converter enable configuration register."]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
    #[doc = "0xa08 - DCDC power-up force register."]
    #[inline(always)]
    pub const fn dcdcforce(&self) -> &Dcdcforce {
        &self.dcdcforce
    }
}
#[doc = "TASKS_CONSTLAT (w) register accessor: Enable constant latency mode.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_constlat`] module"]
#[doc(alias = "TASKS_CONSTLAT")]
pub type TasksConstlat = crate::Reg<tasks_constlat::TasksConstlatSpec>;
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: Enable low power mode (variable latency).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lowpwr`] module"]
#[doc(alias = "TASKS_LOWPWR")]
pub type TasksLowpwr = crate::Reg<tasks_lowpwr::TasksLowpwrSpec>;
#[doc = "Enable low power mode (variable latency)."]
pub mod tasks_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: Power failure warning.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pofwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pofwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pofwarn`] module"]
#[doc(alias = "EVENTS_POFWARN")]
pub type EventsPofwarn = crate::Reg<events_pofwarn::EventsPofwarnSpec>;
#[doc = "Power failure warning."]
pub mod events_pofwarn;
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
#[doc = "RESETREAS (rw) register accessor: Reset reason.\n\nYou can [`read`](crate::Reg::read) this register and get [`resetreas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetreas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetreas`] module"]
#[doc(alias = "RESETREAS")]
pub type Resetreas = crate::Reg<resetreas::ResetreasSpec>;
#[doc = "Reset reason."]
pub mod resetreas;
#[doc = "RAMSTATUS (r) register accessor: Ram status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramstatus`] module"]
#[doc(alias = "RAMSTATUS")]
pub type Ramstatus = crate::Reg<ramstatus::RamstatusSpec>;
#[doc = "Ram status register."]
pub mod ramstatus;
#[doc = "SYSTEMOFF (w) register accessor: System off register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`] module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System off register."]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: Power failure configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofcon`] module"]
#[doc(alias = "POFCON")]
pub type Pofcon = crate::Reg<pofcon::PofconSpec>;
#[doc = "Power failure configuration."]
pub mod pofcon;
#[doc = "GPREGRET (rw) register accessor: General purpose retention register. This register is a retained register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpregret`] module"]
#[doc(alias = "GPREGRET")]
pub type Gpregret = crate::Reg<gpregret::GpregretSpec>;
#[doc = "General purpose retention register. This register is a retained register."]
pub mod gpregret;
#[doc = "RAMON (rw) register accessor: Ram on/off.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramon`] module"]
#[doc(alias = "RAMON")]
pub type Ramon = crate::Reg<ramon::RamonSpec>;
#[doc = "Ram on/off."]
pub mod ramon;
#[doc = "RESET (rw) register accessor: Pin reset functionality configuration register. This register is a retained register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`] module"]
#[doc(alias = "RESET")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "Pin reset functionality configuration register. This register is a retained register."]
pub mod reset;
#[doc = "RAMONB (rw) register accessor: Ram on/off.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramonb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramonb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramonb`] module"]
#[doc(alias = "RAMONB")]
pub type Ramonb = crate::Reg<ramonb::RamonbSpec>;
#[doc = "Ram on/off."]
pub mod ramonb;
#[doc = "DCDCEN (rw) register accessor: DCDC converter enable configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`] module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "DCDC converter enable configuration register."]
pub mod dcdcen;
#[doc = "DCDCFORCE (rw) register accessor: DCDC power-up force register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcforce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcforce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcforce`] module"]
#[doc(alias = "DCDCFORCE")]
pub type Dcdcforce = crate::Reg<dcdcforce::DcdcforceSpec>;
#[doc = "DCDC power-up force register."]
pub mod dcdcforce;
