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
    events_usbdetected: EventsUsbdetected,
    events_usbremoved: EventsUsbremoved,
    events_usbpwrrdy: EventsUsbpwrrdy,
    _reserved8: [u8; 0x01dc],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved10: [u8; 0xf4],
    resetreas: Resetreas,
    _reserved11: [u8; 0x24],
    ramstatus: Ramstatus,
    _reserved12: [u8; 0x0c],
    usbregstatus: Usbregstatus,
    _reserved13: [u8; 0xc4],
    systemoff: Systemoff,
    _reserved14: [u8; 0x0c],
    pofcon: Pofcon,
    _reserved15: [u8; 0x08],
    gpregret: Gpregret,
    gpregret2: Gpregret2,
    _reserved17: [u8; 0x54],
    dcdcen: Dcdcen,
    _reserved18: [u8; 0x04],
    dcdcen0: Dcdcen0,
    _reserved19: [u8; 0xbc],
    mainregstatus: Mainregstatus,
    _reserved20: [u8; 0x02bc],
    ram: (),
}
impl RegisterBlock {
    #[doc = "0x78 - Enable Constant Latency mode"]
    #[inline(always)]
    pub const fn tasks_constlat(&self) -> &TasksConstlat {
        &self.tasks_constlat
    }
    #[doc = "0x7c - Enable Low-power mode (variable latency)"]
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
    #[doc = "0x11c - Voltage supply detected on VBUS"]
    #[inline(always)]
    pub const fn events_usbdetected(&self) -> &EventsUsbdetected {
        &self.events_usbdetected
    }
    #[doc = "0x120 - Voltage supply removed from VBUS"]
    #[inline(always)]
    pub const fn events_usbremoved(&self) -> &EventsUsbremoved {
        &self.events_usbremoved
    }
    #[doc = "0x124 - USB 3.3 V supply ready"]
    #[inline(always)]
    pub const fn events_usbpwrrdy(&self) -> &EventsUsbpwrrdy {
        &self.events_usbpwrrdy
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
    #[doc = "0x438 - USB supply status"]
    #[inline(always)]
    pub const fn usbregstatus(&self) -> &Usbregstatus {
        &self.usbregstatus
    }
    #[doc = "0x500 - System OFF register"]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x510 - Power-fail comparator configuration"]
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
    #[doc = "0x578 - Enable DC/DC converter for REG1 stage"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
    #[doc = "0x580 - Enable DC/DC converter for REG0 stage"]
    #[inline(always)]
    pub const fn dcdcen0(&self) -> &Dcdcen0 {
        &self.dcdcen0
    }
    #[doc = "0x640 - Main supply status"]
    #[inline(always)]
    pub const fn mainregstatus(&self) -> &Mainregstatus {
        &self.mainregstatus
    }
    #[doc = "0x900..0x96c - Unspecified"]
    #[inline(always)]
    pub const fn ram(&self, n: usize) -> &Ram {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2304)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x96c - Unspecified"]
    #[inline(always)]
    pub fn ram_iter(&self) -> impl Iterator<Item = &Ram> {
        (0..9).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2304)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "TASKS_CONSTLAT (w) register accessor: Enable Constant Latency mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_constlat`] module"]
#[doc(alias = "TASKS_CONSTLAT")]
pub type TasksConstlat = crate::Reg<tasks_constlat::TasksConstlatSpec>;
#[doc = "Enable Constant Latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: Enable Low-power mode (variable latency)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lowpwr`] module"]
#[doc(alias = "TASKS_LOWPWR")]
pub type TasksLowpwr = crate::Reg<tasks_lowpwr::TasksLowpwrSpec>;
#[doc = "Enable Low-power mode (variable latency)"]
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
#[doc = "EVENTS_USBDETECTED (rw) register accessor: Voltage supply detected on VBUS\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbdetected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbdetected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_usbdetected`] module"]
#[doc(alias = "EVENTS_USBDETECTED")]
pub type EventsUsbdetected = crate::Reg<events_usbdetected::EventsUsbdetectedSpec>;
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "EVENTS_USBREMOVED (rw) register accessor: Voltage supply removed from VBUS\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbremoved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbremoved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_usbremoved`] module"]
#[doc(alias = "EVENTS_USBREMOVED")]
pub type EventsUsbremoved = crate::Reg<events_usbremoved::EventsUsbremovedSpec>;
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "EVENTS_USBPWRRDY (rw) register accessor: USB 3.3 V supply ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbpwrrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbpwrrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_usbpwrrdy`] module"]
#[doc(alias = "EVENTS_USBPWRRDY")]
pub type EventsUsbpwrrdy = crate::Reg<events_usbpwrrdy::EventsUsbpwrrdySpec>;
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
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
#[doc = "USBREGSTATUS (r) register accessor: USB supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbregstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbregstatus`] module"]
#[doc(alias = "USBREGSTATUS")]
pub type Usbregstatus = crate::Reg<usbregstatus::UsbregstatusSpec>;
#[doc = "USB supply status"]
pub mod usbregstatus;
#[doc = "SYSTEMOFF (w) register accessor: System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`] module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: Power-fail comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofcon`] module"]
#[doc(alias = "POFCON")]
pub type Pofcon = crate::Reg<pofcon::PofconSpec>;
#[doc = "Power-fail comparator configuration"]
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
#[doc = "DCDCEN (rw) register accessor: Enable DC/DC converter for REG1 stage\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`] module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "Enable DC/DC converter for REG1 stage"]
pub mod dcdcen;
#[doc = "DCDCEN0 (rw) register accessor: Enable DC/DC converter for REG0 stage\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen0`] module"]
#[doc(alias = "DCDCEN0")]
pub type Dcdcen0 = crate::Reg<dcdcen0::Dcdcen0Spec>;
#[doc = "Enable DC/DC converter for REG0 stage"]
pub mod dcdcen0;
#[doc = "MAINREGSTATUS (r) register accessor: Main supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`mainregstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainregstatus`] module"]
#[doc(alias = "MAINREGSTATUS")]
pub type Mainregstatus = crate::Reg<mainregstatus::MainregstatusSpec>;
#[doc = "Main supply status"]
pub mod mainregstatus;
#[doc = "Unspecified"]
pub use self::ram::Ram;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ram;
