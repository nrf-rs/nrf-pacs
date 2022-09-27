#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04..0x24 - Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    pub tasks_startepin: [TASKS_STARTEPIN; 8],
    #[doc = "0x24 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    pub tasks_startisoin: TASKS_STARTISOIN,
    #[doc = "0x28..0x48 - Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    pub tasks_startepout: [TASKS_STARTEPOUT; 8],
    #[doc = "0x48 - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    pub tasks_startisoout: TASKS_STARTISOOUT,
    #[doc = "0x4c - Allows OUT data stage on control endpoint 0"]
    pub tasks_ep0rcvout: TASKS_EP0RCVOUT,
    #[doc = "0x50 - Allows status stage on control endpoint 0"]
    pub tasks_ep0status: TASKS_EP0STATUS,
    #[doc = "0x54 - Stalls data and status stage on control endpoint 0"]
    pub tasks_ep0stall: TASKS_EP0STALL,
    #[doc = "0x58 - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    pub tasks_dpdmdrive: TASKS_DPDMDRIVE,
    #[doc = "0x5c - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    pub tasks_dpdmnodrive: TASKS_DPDMNODRIVE,
    _reserved9: [u8; 0xa0],
    #[doc = "0x100 - Signals that a USB reset condition has been detected on USB lines"]
    pub events_usbreset: EVENTS_USBRESET,
    #[doc = "0x104 - Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x108..0x128 - Description collection\\[n\\]: The whole EPIN\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepin: [EVENTS_ENDEPIN; 8],
    #[doc = "0x128 - An acknowledged data transfer has taken place on the control endpoint"]
    pub events_ep0datadone: EVENTS_EP0DATADONE,
    #[doc = "0x12c - The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoin: EVENTS_ENDISOIN,
    #[doc = "0x130..0x150 - Description collection\\[n\\]: The whole EPOUT\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepout: [EVENTS_ENDEPOUT; 8],
    #[doc = "0x150 - The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoout: EVENTS_ENDISOOUT,
    #[doc = "0x154 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    pub events_sof: EVENTS_SOF,
    #[doc = "0x158 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    pub events_usbevent: EVENTS_USBEVENT,
    #[doc = "0x15c - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    pub events_ep0setup: EVENTS_EP0SETUP,
    #[doc = "0x160 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    pub events_epdata: EVENTS_EPDATA,
    _reserved20: [u8; 0x9c],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved21: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved24: [u8; 0xf4],
    #[doc = "0x400 - Details on what caused the USBEVENT event"]
    pub eventcause: EVENTCAUSE,
    _reserved25: [u8; 0x1c],
    #[doc = "0x420..0x464 - Unspecified"]
    pub halted: HALTED,
    _reserved26: [u8; 0x04],
    #[doc = "0x468 - Provides information on which endpoint's EasyDMA registers have been captured"]
    pub epstatus: EPSTATUS,
    #[doc = "0x46c - Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    pub epdatastatus: EPDATASTATUS,
    #[doc = "0x470 - Device USB address"]
    pub usbaddr: USBADDR,
    _reserved29: [u8; 0x0c],
    #[doc = "0x480 - SETUP data, byte 0, bmRequestType"]
    pub bmrequesttype: BMREQUESTTYPE,
    #[doc = "0x484 - SETUP data, byte 1, bRequest"]
    pub brequest: BREQUEST,
    #[doc = "0x488 - SETUP data, byte 2, LSB of wValue"]
    pub wvaluel: WVALUEL,
    #[doc = "0x48c - SETUP data, byte 3, MSB of wValue"]
    pub wvalueh: WVALUEH,
    #[doc = "0x490 - SETUP data, byte 4, LSB of wIndex"]
    pub windexl: WINDEXL,
    #[doc = "0x494 - SETUP data, byte 5, MSB of wIndex"]
    pub windexh: WINDEXH,
    #[doc = "0x498 - SETUP data, byte 6, LSB of wLength"]
    pub wlengthl: WLENGTHL,
    #[doc = "0x49c - SETUP data, byte 7, MSB of wLength"]
    pub wlengthh: WLENGTHH,
    #[doc = "0x4a0..0x4c4 - Unspecified"]
    pub size: SIZE,
    _reserved38: [u8; 0x3c],
    #[doc = "0x500 - Enable USB"]
    pub enable: ENABLE,
    #[doc = "0x504 - Control of the USB pull-up"]
    pub usbpullup: USBPULLUP,
    #[doc = "0x508 - State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    pub dpdmvalue: DPDMVALUE,
    #[doc = "0x50c - Data toggle control and status"]
    pub dtoggle: DTOGGLE,
    #[doc = "0x510 - Endpoint IN enable"]
    pub epinen: EPINEN,
    #[doc = "0x514 - Endpoint OUT enable"]
    pub epouten: EPOUTEN,
    #[doc = "0x518 - STALL endpoints"]
    pub epstall: EPSTALL,
    #[doc = "0x51c - Controls the split of ISO buffers"]
    pub isosplit: ISOSPLIT,
    #[doc = "0x520 - Returns the current value of the start of frame counter"]
    pub framecntr: FRAMECNTR,
    _reserved47: [u8; 0x08],
    #[doc = "0x52c - Controls USBD peripheral low power mode during USB suspend"]
    pub lowpower: LOWPOWER,
    #[doc = "0x530 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    pub isoinconfig: ISOINCONFIG,
    _reserved49: [u8; 0xcc],
    #[doc = "0x600..0x60c - Unspecified"]
    pub epin0: EPIN,
    _reserved50: [u8; 0x08],
    #[doc = "0x614..0x620 - Unspecified"]
    pub epin1: EPIN,
    _reserved51: [u8; 0x08],
    #[doc = "0x628..0x634 - Unspecified"]
    pub epin2: EPIN,
    _reserved52: [u8; 0x08],
    #[doc = "0x63c..0x648 - Unspecified"]
    pub epin3: EPIN,
    _reserved53: [u8; 0x08],
    #[doc = "0x650..0x65c - Unspecified"]
    pub epin4: EPIN,
    _reserved54: [u8; 0x08],
    #[doc = "0x664..0x670 - Unspecified"]
    pub epin5: EPIN,
    _reserved55: [u8; 0x08],
    #[doc = "0x678..0x684 - Unspecified"]
    pub epin6: EPIN,
    _reserved56: [u8; 0x08],
    #[doc = "0x68c..0x698 - Unspecified"]
    pub epin7: EPIN,
    _reserved57: [u8; 0x08],
    #[doc = "0x6a0..0x6ac - Unspecified"]
    pub isoin: ISOIN,
    _reserved58: [u8; 0x54],
    #[doc = "0x700..0x70c - Unspecified"]
    pub epout0: EPOUT,
    _reserved59: [u8; 0x08],
    #[doc = "0x714..0x720 - Unspecified"]
    pub epout1: EPOUT,
    _reserved60: [u8; 0x08],
    #[doc = "0x728..0x734 - Unspecified"]
    pub epout2: EPOUT,
    _reserved61: [u8; 0x08],
    #[doc = "0x73c..0x748 - Unspecified"]
    pub epout3: EPOUT,
    _reserved62: [u8; 0x08],
    #[doc = "0x750..0x75c - Unspecified"]
    pub epout4: EPOUT,
    _reserved63: [u8; 0x08],
    #[doc = "0x764..0x770 - Unspecified"]
    pub epout5: EPOUT,
    _reserved64: [u8; 0x08],
    #[doc = "0x778..0x784 - Unspecified"]
    pub epout6: EPOUT,
    _reserved65: [u8; 0x08],
    #[doc = "0x78c..0x798 - Unspecified"]
    pub epout7: EPOUT,
    _reserved66: [u8; 0x08],
    #[doc = "0x7a0..0x7ac - Unspecified"]
    pub isoout: ISOOUT,
}
#[doc = "TASKS_STARTEPIN (w) register accessor: an alias for `Reg<TASKS_STARTEPIN_SPEC>`"]
pub type TASKS_STARTEPIN = crate::Reg<tasks_startepin::TASKS_STARTEPIN_SPEC>;
#[doc = "Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub mod tasks_startepin;
#[doc = "TASKS_STARTISOIN (w) register accessor: an alias for `Reg<TASKS_STARTISOIN_SPEC>`"]
pub type TASKS_STARTISOIN = crate::Reg<tasks_startisoin::TASKS_STARTISOIN_SPEC>;
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub mod tasks_startisoin;
#[doc = "TASKS_STARTEPOUT (w) register accessor: an alias for `Reg<TASKS_STARTEPOUT_SPEC>`"]
pub type TASKS_STARTEPOUT = crate::Reg<tasks_startepout::TASKS_STARTEPOUT_SPEC>;
#[doc = "Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub mod tasks_startepout;
#[doc = "TASKS_STARTISOOUT (w) register accessor: an alias for `Reg<TASKS_STARTISOOUT_SPEC>`"]
pub type TASKS_STARTISOOUT = crate::Reg<tasks_startisoout::TASKS_STARTISOOUT_SPEC>;
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub mod tasks_startisoout;
#[doc = "TASKS_EP0RCVOUT (w) register accessor: an alias for `Reg<TASKS_EP0RCVOUT_SPEC>`"]
pub type TASKS_EP0RCVOUT = crate::Reg<tasks_ep0rcvout::TASKS_EP0RCVOUT_SPEC>;
#[doc = "Allows OUT data stage on control endpoint 0"]
pub mod tasks_ep0rcvout;
#[doc = "TASKS_EP0STATUS (w) register accessor: an alias for `Reg<TASKS_EP0STATUS_SPEC>`"]
pub type TASKS_EP0STATUS = crate::Reg<tasks_ep0status::TASKS_EP0STATUS_SPEC>;
#[doc = "Allows status stage on control endpoint 0"]
pub mod tasks_ep0status;
#[doc = "TASKS_EP0STALL (w) register accessor: an alias for `Reg<TASKS_EP0STALL_SPEC>`"]
pub type TASKS_EP0STALL = crate::Reg<tasks_ep0stall::TASKS_EP0STALL_SPEC>;
#[doc = "Stalls data and status stage on control endpoint 0"]
pub mod tasks_ep0stall;
#[doc = "TASKS_DPDMDRIVE (w) register accessor: an alias for `Reg<TASKS_DPDMDRIVE_SPEC>`"]
pub type TASKS_DPDMDRIVE = crate::Reg<tasks_dpdmdrive::TASKS_DPDMDRIVE_SPEC>;
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub mod tasks_dpdmdrive;
#[doc = "TASKS_DPDMNODRIVE (w) register accessor: an alias for `Reg<TASKS_DPDMNODRIVE_SPEC>`"]
pub type TASKS_DPDMNODRIVE = crate::Reg<tasks_dpdmnodrive::TASKS_DPDMNODRIVE_SPEC>;
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub mod tasks_dpdmnodrive;
#[doc = "EVENTS_USBRESET (rw) register accessor: an alias for `Reg<EVENTS_USBRESET_SPEC>`"]
pub type EVENTS_USBRESET = crate::Reg<events_usbreset::EVENTS_USBRESET_SPEC>;
#[doc = "Signals that a USB reset condition has been detected on USB lines"]
pub mod events_usbreset;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
pub mod events_started;
#[doc = "EVENTS_ENDEPIN (rw) register accessor: an alias for `Reg<EVENTS_ENDEPIN_SPEC>`"]
pub type EVENTS_ENDEPIN = crate::Reg<events_endepin::EVENTS_ENDEPIN_SPEC>;
#[doc = "Description collection\\[n\\]: The whole EPIN\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepin;
#[doc = "EVENTS_EP0DATADONE (rw) register accessor: an alias for `Reg<EVENTS_EP0DATADONE_SPEC>`"]
pub type EVENTS_EP0DATADONE = crate::Reg<events_ep0datadone::EVENTS_EP0DATADONE_SPEC>;
#[doc = "An acknowledged data transfer has taken place on the control endpoint"]
pub mod events_ep0datadone;
#[doc = "EVENTS_ENDISOIN (rw) register accessor: an alias for `Reg<EVENTS_ENDISOIN_SPEC>`"]
pub type EVENTS_ENDISOIN = crate::Reg<events_endisoin::EVENTS_ENDISOIN_SPEC>;
#[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoin;
#[doc = "EVENTS_ENDEPOUT (rw) register accessor: an alias for `Reg<EVENTS_ENDEPOUT_SPEC>`"]
pub type EVENTS_ENDEPOUT = crate::Reg<events_endepout::EVENTS_ENDEPOUT_SPEC>;
#[doc = "Description collection\\[n\\]: The whole EPOUT\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepout;
#[doc = "EVENTS_ENDISOOUT (rw) register accessor: an alias for `Reg<EVENTS_ENDISOOUT_SPEC>`"]
pub type EVENTS_ENDISOOUT = crate::Reg<events_endisoout::EVENTS_ENDISOOUT_SPEC>;
#[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoout;
#[doc = "EVENTS_SOF (rw) register accessor: an alias for `Reg<EVENTS_SOF_SPEC>`"]
pub type EVENTS_SOF = crate::Reg<events_sof::EVENTS_SOF_SPEC>;
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub mod events_sof;
#[doc = "EVENTS_USBEVENT (rw) register accessor: an alias for `Reg<EVENTS_USBEVENT_SPEC>`"]
pub type EVENTS_USBEVENT = crate::Reg<events_usbevent::EVENTS_USBEVENT_SPEC>;
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub mod events_usbevent;
#[doc = "EVENTS_EP0SETUP (rw) register accessor: an alias for `Reg<EVENTS_EP0SETUP_SPEC>`"]
pub type EVENTS_EP0SETUP = crate::Reg<events_ep0setup::EVENTS_EP0SETUP_SPEC>;
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub mod events_ep0setup;
#[doc = "EVENTS_EPDATA (rw) register accessor: an alias for `Reg<EVENTS_EPDATA_SPEC>`"]
pub type EVENTS_EPDATA = crate::Reg<events_epdata::EVENTS_EPDATA_SPEC>;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub mod events_epdata;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "EVENTCAUSE (rw) register accessor: an alias for `Reg<EVENTCAUSE_SPEC>`"]
pub type EVENTCAUSE = crate::Reg<eventcause::EVENTCAUSE_SPEC>;
#[doc = "Details on what caused the USBEVENT event"]
pub mod eventcause;
#[doc = "Unspecified"]
pub use halted::HALTED;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod halted;
#[doc = "EPSTATUS (rw) register accessor: an alias for `Reg<EPSTATUS_SPEC>`"]
pub type EPSTATUS = crate::Reg<epstatus::EPSTATUS_SPEC>;
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
pub mod epstatus;
#[doc = "EPDATASTATUS (rw) register accessor: an alias for `Reg<EPDATASTATUS_SPEC>`"]
pub type EPDATASTATUS = crate::Reg<epdatastatus::EPDATASTATUS_SPEC>;
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
pub mod epdatastatus;
#[doc = "USBADDR (r) register accessor: an alias for `Reg<USBADDR_SPEC>`"]
pub type USBADDR = crate::Reg<usbaddr::USBADDR_SPEC>;
#[doc = "Device USB address"]
pub mod usbaddr;
#[doc = "BMREQUESTTYPE (r) register accessor: an alias for `Reg<BMREQUESTTYPE_SPEC>`"]
pub type BMREQUESTTYPE = crate::Reg<bmrequesttype::BMREQUESTTYPE_SPEC>;
#[doc = "SETUP data, byte 0, bmRequestType"]
pub mod bmrequesttype;
#[doc = "BREQUEST (r) register accessor: an alias for `Reg<BREQUEST_SPEC>`"]
pub type BREQUEST = crate::Reg<brequest::BREQUEST_SPEC>;
#[doc = "SETUP data, byte 1, bRequest"]
pub mod brequest;
#[doc = "WVALUEL (r) register accessor: an alias for `Reg<WVALUEL_SPEC>`"]
pub type WVALUEL = crate::Reg<wvaluel::WVALUEL_SPEC>;
#[doc = "SETUP data, byte 2, LSB of wValue"]
pub mod wvaluel;
#[doc = "WVALUEH (r) register accessor: an alias for `Reg<WVALUEH_SPEC>`"]
pub type WVALUEH = crate::Reg<wvalueh::WVALUEH_SPEC>;
#[doc = "SETUP data, byte 3, MSB of wValue"]
pub mod wvalueh;
#[doc = "WINDEXL (r) register accessor: an alias for `Reg<WINDEXL_SPEC>`"]
pub type WINDEXL = crate::Reg<windexl::WINDEXL_SPEC>;
#[doc = "SETUP data, byte 4, LSB of wIndex"]
pub mod windexl;
#[doc = "WINDEXH (r) register accessor: an alias for `Reg<WINDEXH_SPEC>`"]
pub type WINDEXH = crate::Reg<windexh::WINDEXH_SPEC>;
#[doc = "SETUP data, byte 5, MSB of wIndex"]
pub mod windexh;
#[doc = "WLENGTHL (r) register accessor: an alias for `Reg<WLENGTHL_SPEC>`"]
pub type WLENGTHL = crate::Reg<wlengthl::WLENGTHL_SPEC>;
#[doc = "SETUP data, byte 6, LSB of wLength"]
pub mod wlengthl;
#[doc = "WLENGTHH (r) register accessor: an alias for `Reg<WLENGTHH_SPEC>`"]
pub type WLENGTHH = crate::Reg<wlengthh::WLENGTHH_SPEC>;
#[doc = "SETUP data, byte 7, MSB of wLength"]
pub mod wlengthh;
#[doc = "Unspecified"]
pub use size::SIZE;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod size;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable USB"]
pub mod enable;
#[doc = "USBPULLUP (rw) register accessor: an alias for `Reg<USBPULLUP_SPEC>`"]
pub type USBPULLUP = crate::Reg<usbpullup::USBPULLUP_SPEC>;
#[doc = "Control of the USB pull-up"]
pub mod usbpullup;
#[doc = "DPDMVALUE (rw) register accessor: an alias for `Reg<DPDMVALUE_SPEC>`"]
pub type DPDMVALUE = crate::Reg<dpdmvalue::DPDMVALUE_SPEC>;
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
pub mod dpdmvalue;
#[doc = "DTOGGLE (rw) register accessor: an alias for `Reg<DTOGGLE_SPEC>`"]
pub type DTOGGLE = crate::Reg<dtoggle::DTOGGLE_SPEC>;
#[doc = "Data toggle control and status"]
pub mod dtoggle;
#[doc = "EPINEN (rw) register accessor: an alias for `Reg<EPINEN_SPEC>`"]
pub type EPINEN = crate::Reg<epinen::EPINEN_SPEC>;
#[doc = "Endpoint IN enable"]
pub mod epinen;
#[doc = "EPOUTEN (rw) register accessor: an alias for `Reg<EPOUTEN_SPEC>`"]
pub type EPOUTEN = crate::Reg<epouten::EPOUTEN_SPEC>;
#[doc = "Endpoint OUT enable"]
pub mod epouten;
#[doc = "EPSTALL (w) register accessor: an alias for `Reg<EPSTALL_SPEC>`"]
pub type EPSTALL = crate::Reg<epstall::EPSTALL_SPEC>;
#[doc = "STALL endpoints"]
pub mod epstall;
#[doc = "ISOSPLIT (rw) register accessor: an alias for `Reg<ISOSPLIT_SPEC>`"]
pub type ISOSPLIT = crate::Reg<isosplit::ISOSPLIT_SPEC>;
#[doc = "Controls the split of ISO buffers"]
pub mod isosplit;
#[doc = "FRAMECNTR (r) register accessor: an alias for `Reg<FRAMECNTR_SPEC>`"]
pub type FRAMECNTR = crate::Reg<framecntr::FRAMECNTR_SPEC>;
#[doc = "Returns the current value of the start of frame counter"]
pub mod framecntr;
#[doc = "LOWPOWER (rw) register accessor: an alias for `Reg<LOWPOWER_SPEC>`"]
pub type LOWPOWER = crate::Reg<lowpower::LOWPOWER_SPEC>;
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
pub mod lowpower;
#[doc = "ISOINCONFIG (rw) register accessor: an alias for `Reg<ISOINCONFIG_SPEC>`"]
pub type ISOINCONFIG = crate::Reg<isoinconfig::ISOINCONFIG_SPEC>;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub mod isoinconfig;
#[doc = "Unspecified"]
pub use epin::EPIN;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod epin;
#[doc = "Unspecified"]
pub use isoin::ISOIN;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod isoin;
#[doc = "Unspecified"]
pub use epout::EPOUT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod epout;
#[doc = "Unspecified"]
pub use isoout::ISOOUT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod isoout;
