#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_usbdetected: EventsUsbdetected,
    events_usbremoved: EventsUsbremoved,
    events_usbpwrrdy: EventsUsbpwrrdy,
    _reserved3: [u8; 0x74],
    publish_usbdetected: PublishUsbdetected,
    publish_usbremoved: PublishUsbremoved,
    publish_usbpwrrdy: PublishUsbpwrrdy,
    _reserved6: [u8; 0x0174],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved9: [u8; 0xf4],
    usbregstatus: Usbregstatus,
}
impl RegisterBlock {
    #[doc = "0x100 - Voltage supply detected on VBUS"]
    #[inline(always)]
    pub const fn events_usbdetected(&self) -> &EventsUsbdetected {
        &self.events_usbdetected
    }
    #[doc = "0x104 - Voltage supply removed from VBUS"]
    #[inline(always)]
    pub const fn events_usbremoved(&self) -> &EventsUsbremoved {
        &self.events_usbremoved
    }
    #[doc = "0x108 - USB 3.3 V supply ready"]
    #[inline(always)]
    pub const fn events_usbpwrrdy(&self) -> &EventsUsbpwrrdy {
        &self.events_usbpwrrdy
    }
    #[doc = "0x180 - Publish configuration for event USBDETECTED"]
    #[inline(always)]
    pub const fn publish_usbdetected(&self) -> &PublishUsbdetected {
        &self.publish_usbdetected
    }
    #[doc = "0x184 - Publish configuration for event USBREMOVED"]
    #[inline(always)]
    pub const fn publish_usbremoved(&self) -> &PublishUsbremoved {
        &self.publish_usbremoved
    }
    #[doc = "0x188 - Publish configuration for event USBPWRRDY"]
    #[inline(always)]
    pub const fn publish_usbpwrrdy(&self) -> &PublishUsbpwrrdy {
        &self.publish_usbpwrrdy
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
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
    #[doc = "0x400 - USB supply status"]
    #[inline(always)]
    pub const fn usbregstatus(&self) -> &Usbregstatus {
        &self.usbregstatus
    }
}
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
#[doc = "PUBLISH_USBDETECTED (rw) register accessor: Publish configuration for event USBDETECTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_usbdetected::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_usbdetected::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_usbdetected`] module"]
#[doc(alias = "PUBLISH_USBDETECTED")]
pub type PublishUsbdetected = crate::Reg<publish_usbdetected::PublishUsbdetectedSpec>;
#[doc = "Publish configuration for event USBDETECTED"]
pub mod publish_usbdetected;
#[doc = "PUBLISH_USBREMOVED (rw) register accessor: Publish configuration for event USBREMOVED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_usbremoved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_usbremoved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_usbremoved`] module"]
#[doc(alias = "PUBLISH_USBREMOVED")]
pub type PublishUsbremoved = crate::Reg<publish_usbremoved::PublishUsbremovedSpec>;
#[doc = "Publish configuration for event USBREMOVED"]
pub mod publish_usbremoved;
#[doc = "PUBLISH_USBPWRRDY (rw) register accessor: Publish configuration for event USBPWRRDY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_usbpwrrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_usbpwrrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_usbpwrrdy`] module"]
#[doc(alias = "PUBLISH_USBPWRRDY")]
pub type PublishUsbpwrrdy = crate::Reg<publish_usbpwrrdy::PublishUsbpwrrdySpec>;
#[doc = "Publish configuration for event USBPWRRDY"]
pub mod publish_usbpwrrdy;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "USBREGSTATUS (r) register accessor: USB supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbregstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbregstatus`] module"]
#[doc(alias = "USBREGSTATUS")]
pub type Usbregstatus = crate::Reg<usbregstatus::UsbregstatusSpec>;
#[doc = "USB supply status"]
pub mod usbregstatus;
