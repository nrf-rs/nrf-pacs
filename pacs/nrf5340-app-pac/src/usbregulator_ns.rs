#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Voltage supply detected on VBUS"]
    pub events_usbdetected: crate::Reg<events_usbdetected::EVENTS_USBDETECTED_SPEC>,
    #[doc = "0x104 - Voltage supply removed from VBUS"]
    pub events_usbremoved: crate::Reg<events_usbremoved::EVENTS_USBREMOVED_SPEC>,
    #[doc = "0x108 - USB 3.3 V supply ready"]
    pub events_usbpwrrdy: crate::Reg<events_usbpwrrdy::EVENTS_USBPWRRDY_SPEC>,
    _reserved3: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event USBDETECTED"]
    pub publish_usbdetected: crate::Reg<publish_usbdetected::PUBLISH_USBDETECTED_SPEC>,
    #[doc = "0x184 - Publish configuration for event USBREMOVED"]
    pub publish_usbremoved: crate::Reg<publish_usbremoved::PUBLISH_USBREMOVED_SPEC>,
    #[doc = "0x188 - Publish configuration for event USBPWRRDY"]
    pub publish_usbpwrrdy: crate::Reg<publish_usbpwrrdy::PUBLISH_USBPWRRDY_SPEC>,
    _reserved6: [u8; 0x0174],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0xf4],
    #[doc = "0x400 - USB supply status"]
    pub usbregstatus: crate::Reg<usbregstatus::USBREGSTATUS_SPEC>,
}
#[doc = "EVENTS_USBDETECTED register accessor: an alias for `Reg<EVENTS_USBDETECTED_SPEC>`"]
pub type EVENTS_USBDETECTED = crate::Reg<events_usbdetected::EVENTS_USBDETECTED_SPEC>;
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "EVENTS_USBREMOVED register accessor: an alias for `Reg<EVENTS_USBREMOVED_SPEC>`"]
pub type EVENTS_USBREMOVED = crate::Reg<events_usbremoved::EVENTS_USBREMOVED_SPEC>;
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "EVENTS_USBPWRRDY register accessor: an alias for `Reg<EVENTS_USBPWRRDY_SPEC>`"]
pub type EVENTS_USBPWRRDY = crate::Reg<events_usbpwrrdy::EVENTS_USBPWRRDY_SPEC>;
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
#[doc = "PUBLISH_USBDETECTED register accessor: an alias for `Reg<PUBLISH_USBDETECTED_SPEC>`"]
pub type PUBLISH_USBDETECTED = crate::Reg<publish_usbdetected::PUBLISH_USBDETECTED_SPEC>;
#[doc = "Publish configuration for event USBDETECTED"]
pub mod publish_usbdetected;
#[doc = "PUBLISH_USBREMOVED register accessor: an alias for `Reg<PUBLISH_USBREMOVED_SPEC>`"]
pub type PUBLISH_USBREMOVED = crate::Reg<publish_usbremoved::PUBLISH_USBREMOVED_SPEC>;
#[doc = "Publish configuration for event USBREMOVED"]
pub mod publish_usbremoved;
#[doc = "PUBLISH_USBPWRRDY register accessor: an alias for `Reg<PUBLISH_USBPWRRDY_SPEC>`"]
pub type PUBLISH_USBPWRRDY = crate::Reg<publish_usbpwrrdy::PUBLISH_USBPWRRDY_SPEC>;
#[doc = "Publish configuration for event USBPWRRDY"]
pub mod publish_usbpwrrdy;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "USBREGSTATUS register accessor: an alias for `Reg<USBREGSTATUS_SPEC>`"]
pub type USBREGSTATUS = crate::Reg<usbregstatus::USBREGSTATUS_SPEC>;
#[doc = "USB supply status"]
pub mod usbregstatus;
