#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    tasks_constlat: TasksConstlat,
    tasks_lowpwr: TasksLowpwr,
    _reserved2: [u8; 0x88],
    events_pofwarn: EventsPofwarn,
    _reserved3: [u8; 0x08],
    events_sleepenter: EventsSleepenter,
    events_sleepexit: EventsSleepexit,
    _reserved5: [u8; 0x01e8],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved7: [u8; 0xf4],
    resetreas: Resetreas,
    _reserved8: [u8; 0x24],
    ramstatus: Ramstatus,
    _reserved9: [u8; 0xd4],
    systemoff: Systemoff,
    _reserved10: [u8; 0x0c],
    pofcon: Pofcon,
    _reserved11: [u8; 0x08],
    gpregret: Gpregret,
    gpregret2: Gpregret2,
    ramon: Ramon,
    _reserved14: [u8; 0x2c],
    ramonb: Ramonb,
    _reserved15: [u8; 0x20],
    dcdcen: Dcdcen,
    _reserved16: [u8; 0x0384],
    ram: (),
}
impl RegisterBlock {
    #[doc = "0x78 - Enable constant latency mode"]
    #[inline(always)]
    pub const fn tasks_constlat(&self) -> &TasksConstlat {
        &self.tasks_constlat
    }
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    #[inline(always)]
    pub const fn tasks_lowpwr(&self) -> &TasksLowpwr {
        &self.tasks_lowpwr
    }
    #[doc = "0x108 - Power failure warning"]
    #[inline(always)]
    pub const fn events_pofwarn(&self) -> &EventsPofwarn {
        &self.events_pofwarn
    }
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepenter(&self) -> &EventsSleepenter {
        &self.events_sleepenter
    }
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepexit(&self) -> &EventsSleepexit {
        &self.events_sleepexit
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
    #[doc = "0x400 - Reset reason"]
    #[inline(always)]
    pub const fn resetreas(&self) -> &Resetreas {
        &self.resetreas
    }
    #[doc = "0x428 - Deprecated register - RAM status register"]
    #[inline(always)]
    pub const fn ramstatus(&self) -> &Ramstatus {
        &self.ramstatus
    }
    #[doc = "0x500 - System OFF register"]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x510 - Power failure comparator configuration"]
    #[inline(always)]
    pub const fn pofcon(&self) -> &Pofcon {
        &self.pofcon
    }
    #[doc = "0x51c - General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(&self) -> &Gpregret {
        &self.gpregret
    }
    #[doc = "0x520 - General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret2(&self) -> &Gpregret2 {
        &self.gpregret2
    }
    #[doc = "0x524 - Deprecated register - RAM on/off register (this register is retained)"]
    #[inline(always)]
    pub const fn ramon(&self) -> &Ramon {
        &self.ramon
    }
    #[doc = "0x554 - Deprecated register - RAM on/off register (this register is retained)"]
    #[inline(always)]
    pub const fn ramonb(&self) -> &Ramonb {
        &self.ramonb
    }
    #[doc = "0x578 - DC/DC enable register"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
    #[doc = "0x900..0x960 - Unspecified"]
    #[inline(always)]
    pub const fn ram(&self, n: usize) -> &Ram {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2304)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x960 - Unspecified"]
    #[inline(always)]
    pub fn ram_iter(&self) -> impl Iterator<Item = &Ram> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2304)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "TASKS_CONSTLAT (w) register accessor: Enable constant latency mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_constlat`] module"]
#[doc(alias = "TASKS_CONSTLAT")]
pub type TasksConstlat = crate::Reg<tasks_constlat::TasksConstlatSpec>;
#[doc = "Enable constant latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: Enable low power mode (variable latency)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lowpwr`] module"]
#[doc(alias = "TASKS_LOWPWR")]
pub type TasksLowpwr = crate::Reg<tasks_lowpwr::TasksLowpwrSpec>;
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: Power failure warning\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pofwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pofwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pofwarn`] module"]
#[doc(alias = "EVENTS_POFWARN")]
pub type EventsPofwarn = crate::Reg<events_pofwarn::EventsPofwarnSpec>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER (rw) register accessor: CPU entered WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepenter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepenter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sleepenter`] module"]
#[doc(alias = "EVENTS_SLEEPENTER")]
pub type EventsSleepenter = crate::Reg<events_sleepenter::EventsSleepenterSpec>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT (rw) register accessor: CPU exited WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepexit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepexit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sleepexit`] module"]
#[doc(alias = "EVENTS_SLEEPEXIT")]
pub type EventsSleepexit = crate::Reg<events_sleepexit::EventsSleepexitSpec>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
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
#[doc = "RESETREAS (rw) register accessor: Reset reason\n\nYou can [`read`](crate::Reg::read) this register and get [`resetreas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetreas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetreas`] module"]
#[doc(alias = "RESETREAS")]
pub type Resetreas = crate::Reg<resetreas::ResetreasSpec>;
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "RAMSTATUS (r) register accessor: Deprecated register - RAM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ramstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramstatus`] module"]
#[doc(alias = "RAMSTATUS")]
pub type Ramstatus = crate::Reg<ramstatus::RamstatusSpec>;
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "SYSTEMOFF (w) register accessor: System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`] module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: Power failure comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofcon`] module"]
#[doc(alias = "POFCON")]
pub type Pofcon = crate::Reg<pofcon::PofconSpec>;
#[doc = "Power failure comparator configuration"]
pub mod pofcon;
#[doc = "GPREGRET (rw) register accessor: General purpose retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpregret`] module"]
#[doc(alias = "GPREGRET")]
pub type Gpregret = crate::Reg<gpregret::GpregretSpec>;
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "GPREGRET2 (rw) register accessor: General purpose retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpregret2`] module"]
#[doc(alias = "GPREGRET2")]
pub type Gpregret2 = crate::Reg<gpregret2::Gpregret2Spec>;
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "RAMON (rw) register accessor: Deprecated register - RAM on/off register (this register is retained)\n\nYou can [`read`](crate::Reg::read) this register and get [`ramon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramon`] module"]
#[doc(alias = "RAMON")]
pub type Ramon = crate::Reg<ramon::RamonSpec>;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramon;
#[doc = "RAMONB (rw) register accessor: Deprecated register - RAM on/off register (this register is retained)\n\nYou can [`read`](crate::Reg::read) this register and get [`ramonb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramonb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramonb`] module"]
#[doc(alias = "RAMONB")]
pub type Ramonb = crate::Reg<ramonb::RamonbSpec>;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramonb;
#[doc = "DCDCEN (rw) register accessor: DC/DC enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`] module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "DC/DC enable register"]
pub mod dcdcen;
#[doc = "Unspecified"]
pub use self::ram::Ram;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ram;
