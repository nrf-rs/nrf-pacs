#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    tasks_startepin: [TasksStartepin; 8],
    tasks_startisoin: TasksStartisoin,
    tasks_startepout: [TasksStartepout; 8],
    tasks_startisoout: TasksStartisoout,
    tasks_ep0rcvout: TasksEp0rcvout,
    tasks_ep0status: TasksEp0status,
    tasks_ep0stall: TasksEp0stall,
    tasks_dpdmdrive: TasksDpdmdrive,
    tasks_dpdmnodrive: TasksDpdmnodrive,
    _reserved9: [u8; 0x24],
    subscribe_startepin: [SubscribeStartepin; 8],
    subscribe_startisoin: SubscribeStartisoin,
    subscribe_startepout: [SubscribeStartepout; 8],
    subscribe_startisoout: SubscribeStartisoout,
    subscribe_ep0rcvout: SubscribeEp0rcvout,
    subscribe_ep0status: SubscribeEp0status,
    subscribe_ep0stall: SubscribeEp0stall,
    subscribe_dpdmdrive: SubscribeDpdmdrive,
    subscribe_dpdmnodrive: SubscribeDpdmnodrive,
    _reserved18: [u8; 0x20],
    events_usbreset: EventsUsbreset,
    events_started: EventsStarted,
    events_endepin: [EventsEndepin; 8],
    events_ep0datadone: EventsEp0datadone,
    events_endisoin: EventsEndisoin,
    events_endepout: [EventsEndepout; 8],
    events_endisoout: EventsEndisoout,
    events_sof: EventsSof,
    events_usbevent: EventsUsbevent,
    events_ep0setup: EventsEp0setup,
    events_epdata: EventsEpdata,
    _reserved29: [u8; 0x1c],
    publish_usbreset: PublishUsbreset,
    publish_started: PublishStarted,
    publish_endepin: [PublishEndepin; 8],
    publish_ep0datadone: PublishEp0datadone,
    publish_endisoin: PublishEndisoin,
    publish_endepout: [PublishEndepout; 8],
    publish_endisoout: PublishEndisoout,
    publish_sof: PublishSof,
    publish_usbevent: PublishUsbevent,
    publish_ep0setup: PublishEp0setup,
    publish_epdata: PublishEpdata,
    _reserved40: [u8; 0x1c],
    shorts: Shorts,
    _reserved41: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved44: [u8; 0xf4],
    eventcause: Eventcause,
    _reserved45: [u8; 0x1c],
    halted: Halted,
    _reserved46: [u8; 0x04],
    epstatus: Epstatus,
    epdatastatus: Epdatastatus,
    usbaddr: Usbaddr,
    _reserved49: [u8; 0x0c],
    bmrequesttype: Bmrequesttype,
    brequest: Brequest,
    wvaluel: Wvaluel,
    wvalueh: Wvalueh,
    windexl: Windexl,
    windexh: Windexh,
    wlengthl: Wlengthl,
    wlengthh: Wlengthh,
    size: Size,
    _reserved58: [u8; 0x3c],
    enable: Enable,
    usbpullup: Usbpullup,
    dpdmvalue: Dpdmvalue,
    dtoggle: Dtoggle,
    epinen: Epinen,
    epouten: Epouten,
    epstall: Epstall,
    isosplit: Isosplit,
    framecntr: Framecntr,
    _reserved67: [u8; 0x08],
    lowpower: Lowpower,
    isoinconfig: Isoinconfig,
    _reserved69: [u8; 0xcc],
    epin: (),
    _reserved70: [u8; 0xa0],
    isoin: Isoin,
    _reserved71: [u8; 0x54],
    epout: (),
    _reserved72: [u8; 0xa0],
    isoout: Isoout,
}
impl RegisterBlock {
    #[doc = "0x04..0x24 - Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    pub const fn tasks_startepin(&self, n: usize) -> &TasksStartepin {
        &self.tasks_startepin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x24 - Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepin_iter(&self) -> impl Iterator<Item = &TasksStartepin> {
        self.tasks_startepin.iter()
    }
    #[doc = "0x24 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    #[inline(always)]
    pub const fn tasks_startisoin(&self) -> &TasksStartisoin {
        &self.tasks_startisoin
    }
    #[doc = "0x28..0x48 - Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    #[inline(always)]
    pub const fn tasks_startepout(&self, n: usize) -> &TasksStartepout {
        &self.tasks_startepout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x48 - Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepout_iter(&self) -> impl Iterator<Item = &TasksStartepout> {
        self.tasks_startepout.iter()
    }
    #[doc = "0x48 - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    #[inline(always)]
    pub const fn tasks_startisoout(&self) -> &TasksStartisoout {
        &self.tasks_startisoout
    }
    #[doc = "0x4c - Allows OUT data stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0rcvout(&self) -> &TasksEp0rcvout {
        &self.tasks_ep0rcvout
    }
    #[doc = "0x50 - Allows status stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0status(&self) -> &TasksEp0status {
        &self.tasks_ep0status
    }
    #[doc = "0x54 - Stalls data and status stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0stall(&self) -> &TasksEp0stall {
        &self.tasks_ep0stall
    }
    #[doc = "0x58 - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    #[inline(always)]
    pub const fn tasks_dpdmdrive(&self) -> &TasksDpdmdrive {
        &self.tasks_dpdmdrive
    }
    #[doc = "0x5c - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    #[inline(always)]
    pub const fn tasks_dpdmnodrive(&self) -> &TasksDpdmnodrive {
        &self.tasks_dpdmnodrive
    }
    #[doc = "0x84..0xa4 - Description collection: Subscribe configuration for task STARTEPIN\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_startepin(&self, n: usize) -> &SubscribeStartepin {
        &self.subscribe_startepin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0xa4 - Description collection: Subscribe configuration for task STARTEPIN\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_startepin_iter(&self) -> impl Iterator<Item = &SubscribeStartepin> {
        self.subscribe_startepin.iter()
    }
    #[doc = "0xa4 - Subscribe configuration for task STARTISOIN"]
    #[inline(always)]
    pub const fn subscribe_startisoin(&self) -> &SubscribeStartisoin {
        &self.subscribe_startisoin
    }
    #[doc = "0xa8..0xc8 - Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_startepout(&self, n: usize) -> &SubscribeStartepout {
        &self.subscribe_startepout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa8..0xc8 - Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_startepout_iter(&self) -> impl Iterator<Item = &SubscribeStartepout> {
        self.subscribe_startepout.iter()
    }
    #[doc = "0xc8 - Subscribe configuration for task STARTISOOUT"]
    #[inline(always)]
    pub const fn subscribe_startisoout(&self) -> &SubscribeStartisoout {
        &self.subscribe_startisoout
    }
    #[doc = "0xcc - Subscribe configuration for task EP0RCVOUT"]
    #[inline(always)]
    pub const fn subscribe_ep0rcvout(&self) -> &SubscribeEp0rcvout {
        &self.subscribe_ep0rcvout
    }
    #[doc = "0xd0 - Subscribe configuration for task EP0STATUS"]
    #[inline(always)]
    pub const fn subscribe_ep0status(&self) -> &SubscribeEp0status {
        &self.subscribe_ep0status
    }
    #[doc = "0xd4 - Subscribe configuration for task EP0STALL"]
    #[inline(always)]
    pub const fn subscribe_ep0stall(&self) -> &SubscribeEp0stall {
        &self.subscribe_ep0stall
    }
    #[doc = "0xd8 - Subscribe configuration for task DPDMDRIVE"]
    #[inline(always)]
    pub const fn subscribe_dpdmdrive(&self) -> &SubscribeDpdmdrive {
        &self.subscribe_dpdmdrive
    }
    #[doc = "0xdc - Subscribe configuration for task DPDMNODRIVE"]
    #[inline(always)]
    pub const fn subscribe_dpdmnodrive(&self) -> &SubscribeDpdmnodrive {
        &self.subscribe_dpdmnodrive
    }
    #[doc = "0x100 - Signals that a USB reset condition has been detected on USB lines"]
    #[inline(always)]
    pub const fn events_usbreset(&self) -> &EventsUsbreset {
        &self.events_usbreset
    }
    #[doc = "0x104 - Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x108..0x128 - Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endepin(&self, n: usize) -> &EventsEndepin {
        &self.events_endepin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x128 - Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin_iter(&self) -> impl Iterator<Item = &EventsEndepin> {
        self.events_endepin.iter()
    }
    #[doc = "0x128 - An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub const fn events_ep0datadone(&self) -> &EventsEp0datadone {
        &self.events_ep0datadone
    }
    #[doc = "0x12c - The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endisoin(&self) -> &EventsEndisoin {
        &self.events_endisoin
    }
    #[doc = "0x130..0x150 - Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endepout(&self, n: usize) -> &EventsEndepout {
        &self.events_endepout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x150 - Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout_iter(&self) -> impl Iterator<Item = &EventsEndepout> {
        self.events_endepout.iter()
    }
    #[doc = "0x150 - The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endisoout(&self) -> &EventsEndisoout {
        &self.events_endisoout
    }
    #[doc = "0x154 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    #[inline(always)]
    pub const fn events_sof(&self) -> &EventsSof {
        &self.events_sof
    }
    #[doc = "0x158 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    #[inline(always)]
    pub const fn events_usbevent(&self) -> &EventsUsbevent {
        &self.events_usbevent
    }
    #[doc = "0x15c - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    #[inline(always)]
    pub const fn events_ep0setup(&self) -> &EventsEp0setup {
        &self.events_ep0setup
    }
    #[doc = "0x160 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub const fn events_epdata(&self) -> &EventsEpdata {
        &self.events_epdata
    }
    #[doc = "0x180 - Publish configuration for event USBRESET"]
    #[inline(always)]
    pub const fn publish_usbreset(&self) -> &PublishUsbreset {
        &self.publish_usbreset
    }
    #[doc = "0x184 - Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(&self) -> &PublishStarted {
        &self.publish_started
    }
    #[doc = "0x188..0x1a8 - Description collection: Publish configuration for event ENDEPIN\\[n\\]"]
    #[inline(always)]
    pub const fn publish_endepin(&self, n: usize) -> &PublishEndepin {
        &self.publish_endepin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x188..0x1a8 - Description collection: Publish configuration for event ENDEPIN\\[n\\]"]
    #[inline(always)]
    pub fn publish_endepin_iter(&self) -> impl Iterator<Item = &PublishEndepin> {
        self.publish_endepin.iter()
    }
    #[doc = "0x1a8 - Publish configuration for event EP0DATADONE"]
    #[inline(always)]
    pub const fn publish_ep0datadone(&self) -> &PublishEp0datadone {
        &self.publish_ep0datadone
    }
    #[doc = "0x1ac - Publish configuration for event ENDISOIN"]
    #[inline(always)]
    pub const fn publish_endisoin(&self) -> &PublishEndisoin {
        &self.publish_endisoin
    }
    #[doc = "0x1b0..0x1d0 - Description collection: Publish configuration for event ENDEPOUT\\[n\\]"]
    #[inline(always)]
    pub const fn publish_endepout(&self, n: usize) -> &PublishEndepout {
        &self.publish_endepout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1d0 - Description collection: Publish configuration for event ENDEPOUT\\[n\\]"]
    #[inline(always)]
    pub fn publish_endepout_iter(&self) -> impl Iterator<Item = &PublishEndepout> {
        self.publish_endepout.iter()
    }
    #[doc = "0x1d0 - Publish configuration for event ENDISOOUT"]
    #[inline(always)]
    pub const fn publish_endisoout(&self) -> &PublishEndisoout {
        &self.publish_endisoout
    }
    #[doc = "0x1d4 - Publish configuration for event SOF"]
    #[inline(always)]
    pub const fn publish_sof(&self) -> &PublishSof {
        &self.publish_sof
    }
    #[doc = "0x1d8 - Publish configuration for event USBEVENT"]
    #[inline(always)]
    pub const fn publish_usbevent(&self) -> &PublishUsbevent {
        &self.publish_usbevent
    }
    #[doc = "0x1dc - Publish configuration for event EP0SETUP"]
    #[inline(always)]
    pub const fn publish_ep0setup(&self) -> &PublishEp0setup {
        &self.publish_ep0setup
    }
    #[doc = "0x1e0 - Publish configuration for event EPDATA"]
    #[inline(always)]
    pub const fn publish_epdata(&self) -> &PublishEpdata {
        &self.publish_epdata
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x400 - Details on what caused the USBEVENT event"]
    #[inline(always)]
    pub const fn eventcause(&self) -> &Eventcause {
        &self.eventcause
    }
    #[doc = "0x420..0x464 - Unspecified"]
    #[inline(always)]
    pub const fn halted(&self) -> &Halted {
        &self.halted
    }
    #[doc = "0x468 - Provides information on which endpoint's EasyDMA registers have been captured"]
    #[inline(always)]
    pub const fn epstatus(&self) -> &Epstatus {
        &self.epstatus
    }
    #[doc = "0x46c - Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    #[inline(always)]
    pub const fn epdatastatus(&self) -> &Epdatastatus {
        &self.epdatastatus
    }
    #[doc = "0x470 - Device USB address"]
    #[inline(always)]
    pub const fn usbaddr(&self) -> &Usbaddr {
        &self.usbaddr
    }
    #[doc = "0x480 - SETUP data, byte 0, bmRequestType"]
    #[inline(always)]
    pub const fn bmrequesttype(&self) -> &Bmrequesttype {
        &self.bmrequesttype
    }
    #[doc = "0x484 - SETUP data, byte 1, bRequest"]
    #[inline(always)]
    pub const fn brequest(&self) -> &Brequest {
        &self.brequest
    }
    #[doc = "0x488 - SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub const fn wvaluel(&self) -> &Wvaluel {
        &self.wvaluel
    }
    #[doc = "0x48c - SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub const fn wvalueh(&self) -> &Wvalueh {
        &self.wvalueh
    }
    #[doc = "0x490 - SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub const fn windexl(&self) -> &Windexl {
        &self.windexl
    }
    #[doc = "0x494 - SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub const fn windexh(&self) -> &Windexh {
        &self.windexh
    }
    #[doc = "0x498 - SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub const fn wlengthl(&self) -> &Wlengthl {
        &self.wlengthl
    }
    #[doc = "0x49c - SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub const fn wlengthh(&self) -> &Wlengthh {
        &self.wlengthh
    }
    #[doc = "0x4a0..0x4c4 - Unspecified"]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
    #[doc = "0x500 - Enable USB"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Control of the USB pull-up"]
    #[inline(always)]
    pub const fn usbpullup(&self) -> &Usbpullup {
        &self.usbpullup
    }
    #[doc = "0x508 - State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    #[inline(always)]
    pub const fn dpdmvalue(&self) -> &Dpdmvalue {
        &self.dpdmvalue
    }
    #[doc = "0x50c - Data toggle control and status"]
    #[inline(always)]
    pub const fn dtoggle(&self) -> &Dtoggle {
        &self.dtoggle
    }
    #[doc = "0x510 - Endpoint IN enable"]
    #[inline(always)]
    pub const fn epinen(&self) -> &Epinen {
        &self.epinen
    }
    #[doc = "0x514 - Endpoint OUT enable"]
    #[inline(always)]
    pub const fn epouten(&self) -> &Epouten {
        &self.epouten
    }
    #[doc = "0x518 - STALL endpoints"]
    #[inline(always)]
    pub const fn epstall(&self) -> &Epstall {
        &self.epstall
    }
    #[doc = "0x51c - Controls the split of ISO buffers"]
    #[inline(always)]
    pub const fn isosplit(&self) -> &Isosplit {
        &self.isosplit
    }
    #[doc = "0x520 - Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub const fn framecntr(&self) -> &Framecntr {
        &self.framecntr
    }
    #[doc = "0x52c - Controls USBD peripheral low power mode during USB suspend"]
    #[inline(always)]
    pub const fn lowpower(&self) -> &Lowpower {
        &self.lowpower
    }
    #[doc = "0x530 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub const fn isoinconfig(&self) -> &Isoinconfig {
        &self.isoinconfig
    }
    #[doc = "0x600..0x660 - Unspecified"]
    #[inline(always)]
    pub const fn epin(&self, n: usize) -> &Epin {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x660 - Unspecified"]
    #[inline(always)]
    pub fn epin_iter(&self) -> impl Iterator<Item = &Epin> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x6a0..0x6ac - Unspecified"]
    #[inline(always)]
    pub const fn isoin(&self) -> &Isoin {
        &self.isoin
    }
    #[doc = "0x700..0x760 - Unspecified"]
    #[inline(always)]
    pub const fn epout(&self, n: usize) -> &Epout {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1792)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700..0x760 - Unspecified"]
    #[inline(always)]
    pub fn epout_iter(&self) -> impl Iterator<Item = &Epout> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1792)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x7a0..0x7ac - Unspecified"]
    #[inline(always)]
    pub const fn isoout(&self) -> &Isoout {
        &self.isoout
    }
}
#[doc = "TASKS_STARTEPIN (w) register accessor: Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startepin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startepin`] module"]
#[doc(alias = "TASKS_STARTEPIN")]
pub type TasksStartepin = crate::Reg<tasks_startepin::TasksStartepinSpec>;
#[doc = "Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub mod tasks_startepin;
#[doc = "TASKS_STARTISOIN (w) register accessor: Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startisoin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startisoin`] module"]
#[doc(alias = "TASKS_STARTISOIN")]
pub type TasksStartisoin = crate::Reg<tasks_startisoin::TasksStartisoinSpec>;
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub mod tasks_startisoin;
#[doc = "TASKS_STARTEPOUT (w) register accessor: Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startepout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startepout`] module"]
#[doc(alias = "TASKS_STARTEPOUT")]
pub type TasksStartepout = crate::Reg<tasks_startepout::TasksStartepoutSpec>;
#[doc = "Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub mod tasks_startepout;
#[doc = "TASKS_STARTISOOUT (w) register accessor: Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startisoout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startisoout`] module"]
#[doc(alias = "TASKS_STARTISOOUT")]
pub type TasksStartisoout = crate::Reg<tasks_startisoout::TasksStartisooutSpec>;
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub mod tasks_startisoout;
#[doc = "TASKS_EP0RCVOUT (w) register accessor: Allows OUT data stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0rcvout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ep0rcvout`] module"]
#[doc(alias = "TASKS_EP0RCVOUT")]
pub type TasksEp0rcvout = crate::Reg<tasks_ep0rcvout::TasksEp0rcvoutSpec>;
#[doc = "Allows OUT data stage on control endpoint 0"]
pub mod tasks_ep0rcvout;
#[doc = "TASKS_EP0STATUS (w) register accessor: Allows status stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ep0status`] module"]
#[doc(alias = "TASKS_EP0STATUS")]
pub type TasksEp0status = crate::Reg<tasks_ep0status::TasksEp0statusSpec>;
#[doc = "Allows status stage on control endpoint 0"]
pub mod tasks_ep0status;
#[doc = "TASKS_EP0STALL (w) register accessor: Stalls data and status stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0stall::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ep0stall`] module"]
#[doc(alias = "TASKS_EP0STALL")]
pub type TasksEp0stall = crate::Reg<tasks_ep0stall::TasksEp0stallSpec>;
#[doc = "Stalls data and status stage on control endpoint 0"]
pub mod tasks_ep0stall;
#[doc = "TASKS_DPDMDRIVE (w) register accessor: Forces D+ and D- lines into the state defined in the DPDMVALUE register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_dpdmdrive::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_dpdmdrive`] module"]
#[doc(alias = "TASKS_DPDMDRIVE")]
pub type TasksDpdmdrive = crate::Reg<tasks_dpdmdrive::TasksDpdmdriveSpec>;
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub mod tasks_dpdmdrive;
#[doc = "TASKS_DPDMNODRIVE (w) register accessor: Stops forcing D+ and D- lines into any state (USB engine takes control)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_dpdmnodrive::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_dpdmnodrive`] module"]
#[doc(alias = "TASKS_DPDMNODRIVE")]
pub type TasksDpdmnodrive = crate::Reg<tasks_dpdmnodrive::TasksDpdmnodriveSpec>;
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub mod tasks_dpdmnodrive;
#[doc = "SUBSCRIBE_STARTEPIN (rw) register accessor: Description collection: Subscribe configuration for task STARTEPIN\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_startepin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_startepin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startepin`] module"]
#[doc(alias = "SUBSCRIBE_STARTEPIN")]
pub type SubscribeStartepin = crate::Reg<subscribe_startepin::SubscribeStartepinSpec>;
#[doc = "Description collection: Subscribe configuration for task STARTEPIN\\[n\\]"]
pub mod subscribe_startepin;
#[doc = "SUBSCRIBE_STARTISOIN (rw) register accessor: Subscribe configuration for task STARTISOIN\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_startisoin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_startisoin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startisoin`] module"]
#[doc(alias = "SUBSCRIBE_STARTISOIN")]
pub type SubscribeStartisoin = crate::Reg<subscribe_startisoin::SubscribeStartisoinSpec>;
#[doc = "Subscribe configuration for task STARTISOIN"]
pub mod subscribe_startisoin;
#[doc = "SUBSCRIBE_STARTEPOUT (rw) register accessor: Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_startepout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_startepout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startepout`] module"]
#[doc(alias = "SUBSCRIBE_STARTEPOUT")]
pub type SubscribeStartepout = crate::Reg<subscribe_startepout::SubscribeStartepoutSpec>;
#[doc = "Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]"]
pub mod subscribe_startepout;
#[doc = "SUBSCRIBE_STARTISOOUT (rw) register accessor: Subscribe configuration for task STARTISOOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_startisoout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_startisoout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startisoout`] module"]
#[doc(alias = "SUBSCRIBE_STARTISOOUT")]
pub type SubscribeStartisoout = crate::Reg<subscribe_startisoout::SubscribeStartisooutSpec>;
#[doc = "Subscribe configuration for task STARTISOOUT"]
pub mod subscribe_startisoout;
#[doc = "SUBSCRIBE_EP0RCVOUT (rw) register accessor: Subscribe configuration for task EP0RCVOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_ep0rcvout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_ep0rcvout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_ep0rcvout`] module"]
#[doc(alias = "SUBSCRIBE_EP0RCVOUT")]
pub type SubscribeEp0rcvout = crate::Reg<subscribe_ep0rcvout::SubscribeEp0rcvoutSpec>;
#[doc = "Subscribe configuration for task EP0RCVOUT"]
pub mod subscribe_ep0rcvout;
#[doc = "SUBSCRIBE_EP0STATUS (rw) register accessor: Subscribe configuration for task EP0STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_ep0status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_ep0status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_ep0status`] module"]
#[doc(alias = "SUBSCRIBE_EP0STATUS")]
pub type SubscribeEp0status = crate::Reg<subscribe_ep0status::SubscribeEp0statusSpec>;
#[doc = "Subscribe configuration for task EP0STATUS"]
pub mod subscribe_ep0status;
#[doc = "SUBSCRIBE_EP0STALL (rw) register accessor: Subscribe configuration for task EP0STALL\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_ep0stall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_ep0stall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_ep0stall`] module"]
#[doc(alias = "SUBSCRIBE_EP0STALL")]
pub type SubscribeEp0stall = crate::Reg<subscribe_ep0stall::SubscribeEp0stallSpec>;
#[doc = "Subscribe configuration for task EP0STALL"]
pub mod subscribe_ep0stall;
#[doc = "SUBSCRIBE_DPDMDRIVE (rw) register accessor: Subscribe configuration for task DPDMDRIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_dpdmdrive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_dpdmdrive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_dpdmdrive`] module"]
#[doc(alias = "SUBSCRIBE_DPDMDRIVE")]
pub type SubscribeDpdmdrive = crate::Reg<subscribe_dpdmdrive::SubscribeDpdmdriveSpec>;
#[doc = "Subscribe configuration for task DPDMDRIVE"]
pub mod subscribe_dpdmdrive;
#[doc = "SUBSCRIBE_DPDMNODRIVE (rw) register accessor: Subscribe configuration for task DPDMNODRIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_dpdmnodrive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_dpdmnodrive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_dpdmnodrive`] module"]
#[doc(alias = "SUBSCRIBE_DPDMNODRIVE")]
pub type SubscribeDpdmnodrive = crate::Reg<subscribe_dpdmnodrive::SubscribeDpdmnodriveSpec>;
#[doc = "Subscribe configuration for task DPDMNODRIVE"]
pub mod subscribe_dpdmnodrive;
#[doc = "EVENTS_USBRESET (rw) register accessor: Signals that a USB reset condition has been detected on USB lines\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_usbreset`] module"]
#[doc(alias = "EVENTS_USBRESET")]
pub type EventsUsbreset = crate::Reg<events_usbreset::EventsUsbresetSpec>;
#[doc = "Signals that a USB reset condition has been detected on USB lines"]
pub mod events_usbreset;
#[doc = "EVENTS_STARTED (rw) register accessor: Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`] module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
pub mod events_started;
#[doc = "EVENTS_ENDEPIN (rw) register accessor: Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endepin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endepin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endepin`] module"]
#[doc(alias = "EVENTS_ENDEPIN")]
pub type EventsEndepin = crate::Reg<events_endepin::EventsEndepinSpec>;
#[doc = "Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub mod events_endepin;
#[doc = "EVENTS_EP0DATADONE (rw) register accessor: An acknowledged data transfer has taken place on the control endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ep0datadone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ep0datadone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ep0datadone`] module"]
#[doc(alias = "EVENTS_EP0DATADONE")]
pub type EventsEp0datadone = crate::Reg<events_ep0datadone::EventsEp0datadoneSpec>;
#[doc = "An acknowledged data transfer has taken place on the control endpoint"]
pub mod events_ep0datadone;
#[doc = "EVENTS_ENDISOIN (rw) register accessor: The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endisoin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endisoin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endisoin`] module"]
#[doc(alias = "EVENTS_ENDISOIN")]
pub type EventsEndisoin = crate::Reg<events_endisoin::EventsEndisoinSpec>;
#[doc = "The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
pub mod events_endisoin;
#[doc = "EVENTS_ENDEPOUT (rw) register accessor: Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endepout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endepout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endepout`] module"]
#[doc(alias = "EVENTS_ENDEPOUT")]
pub type EventsEndepout = crate::Reg<events_endepout::EventsEndepoutSpec>;
#[doc = "Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
pub mod events_endepout;
#[doc = "EVENTS_ENDISOOUT (rw) register accessor: The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endisoout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endisoout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endisoout`] module"]
#[doc(alias = "EVENTS_ENDISOOUT")]
pub type EventsEndisoout = crate::Reg<events_endisoout::EventsEndisooutSpec>;
#[doc = "The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
pub mod events_endisoout;
#[doc = "EVENTS_SOF (rw) register accessor: Signals that a SOF (start of frame) condition has been detected on USB lines\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sof`] module"]
#[doc(alias = "EVENTS_SOF")]
pub type EventsSof = crate::Reg<events_sof::EventsSofSpec>;
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub mod events_sof;
#[doc = "EVENTS_USBEVENT (rw) register accessor: An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbevent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbevent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_usbevent`] module"]
#[doc(alias = "EVENTS_USBEVENT")]
pub type EventsUsbevent = crate::Reg<events_usbevent::EventsUsbeventSpec>;
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub mod events_usbevent;
#[doc = "EVENTS_EP0SETUP (rw) register accessor: A valid SETUP token has been received (and acknowledged) on the control endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ep0setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ep0setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ep0setup`] module"]
#[doc(alias = "EVENTS_EP0SETUP")]
pub type EventsEp0setup = crate::Reg<events_ep0setup::EventsEp0setupSpec>;
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub mod events_ep0setup;
#[doc = "EVENTS_EPDATA (rw) register accessor: A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_epdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_epdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_epdata`] module"]
#[doc(alias = "EVENTS_EPDATA")]
pub type EventsEpdata = crate::Reg<events_epdata::EventsEpdataSpec>;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub mod events_epdata;
#[doc = "PUBLISH_USBRESET (rw) register accessor: Publish configuration for event USBRESET\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_usbreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_usbreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_usbreset`] module"]
#[doc(alias = "PUBLISH_USBRESET")]
pub type PublishUsbreset = crate::Reg<publish_usbreset::PublishUsbresetSpec>;
#[doc = "Publish configuration for event USBRESET"]
pub mod publish_usbreset;
#[doc = "PUBLISH_STARTED (rw) register accessor: Publish configuration for event STARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_started`] module"]
#[doc(alias = "PUBLISH_STARTED")]
pub type PublishStarted = crate::Reg<publish_started::PublishStartedSpec>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "PUBLISH_ENDEPIN (rw) register accessor: Description collection: Publish configuration for event ENDEPIN\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endepin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endepin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endepin`] module"]
#[doc(alias = "PUBLISH_ENDEPIN")]
pub type PublishEndepin = crate::Reg<publish_endepin::PublishEndepinSpec>;
#[doc = "Description collection: Publish configuration for event ENDEPIN\\[n\\]"]
pub mod publish_endepin;
#[doc = "PUBLISH_EP0DATADONE (rw) register accessor: Publish configuration for event EP0DATADONE\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ep0datadone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ep0datadone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ep0datadone`] module"]
#[doc(alias = "PUBLISH_EP0DATADONE")]
pub type PublishEp0datadone = crate::Reg<publish_ep0datadone::PublishEp0datadoneSpec>;
#[doc = "Publish configuration for event EP0DATADONE"]
pub mod publish_ep0datadone;
#[doc = "PUBLISH_ENDISOIN (rw) register accessor: Publish configuration for event ENDISOIN\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endisoin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endisoin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endisoin`] module"]
#[doc(alias = "PUBLISH_ENDISOIN")]
pub type PublishEndisoin = crate::Reg<publish_endisoin::PublishEndisoinSpec>;
#[doc = "Publish configuration for event ENDISOIN"]
pub mod publish_endisoin;
#[doc = "PUBLISH_ENDEPOUT (rw) register accessor: Description collection: Publish configuration for event ENDEPOUT\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endepout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endepout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endepout`] module"]
#[doc(alias = "PUBLISH_ENDEPOUT")]
pub type PublishEndepout = crate::Reg<publish_endepout::PublishEndepoutSpec>;
#[doc = "Description collection: Publish configuration for event ENDEPOUT\\[n\\]"]
pub mod publish_endepout;
#[doc = "PUBLISH_ENDISOOUT (rw) register accessor: Publish configuration for event ENDISOOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_endisoout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_endisoout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endisoout`] module"]
#[doc(alias = "PUBLISH_ENDISOOUT")]
pub type PublishEndisoout = crate::Reg<publish_endisoout::PublishEndisooutSpec>;
#[doc = "Publish configuration for event ENDISOOUT"]
pub mod publish_endisoout;
#[doc = "PUBLISH_SOF (rw) register accessor: Publish configuration for event SOF\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_sof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_sof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_sof`] module"]
#[doc(alias = "PUBLISH_SOF")]
pub type PublishSof = crate::Reg<publish_sof::PublishSofSpec>;
#[doc = "Publish configuration for event SOF"]
pub mod publish_sof;
#[doc = "PUBLISH_USBEVENT (rw) register accessor: Publish configuration for event USBEVENT\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_usbevent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_usbevent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_usbevent`] module"]
#[doc(alias = "PUBLISH_USBEVENT")]
pub type PublishUsbevent = crate::Reg<publish_usbevent::PublishUsbeventSpec>;
#[doc = "Publish configuration for event USBEVENT"]
pub mod publish_usbevent;
#[doc = "PUBLISH_EP0SETUP (rw) register accessor: Publish configuration for event EP0SETUP\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ep0setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ep0setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ep0setup`] module"]
#[doc(alias = "PUBLISH_EP0SETUP")]
pub type PublishEp0setup = crate::Reg<publish_ep0setup::PublishEp0setupSpec>;
#[doc = "Publish configuration for event EP0SETUP"]
pub mod publish_ep0setup;
#[doc = "PUBLISH_EPDATA (rw) register accessor: Publish configuration for event EPDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_epdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_epdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_epdata`] module"]
#[doc(alias = "PUBLISH_EPDATA")]
pub type PublishEpdata = crate::Reg<publish_epdata::PublishEpdataSpec>;
#[doc = "Publish configuration for event EPDATA"]
pub mod publish_epdata;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "EVENTCAUSE (rw) register accessor: Details on what caused the USBEVENT event\n\nYou can [`read`](crate::Reg::read) this register and get [`eventcause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventcause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventcause`] module"]
#[doc(alias = "EVENTCAUSE")]
pub type Eventcause = crate::Reg<eventcause::EventcauseSpec>;
#[doc = "Details on what caused the USBEVENT event"]
pub mod eventcause;
#[doc = "Unspecified"]
pub use self::halted::Halted;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod halted;
#[doc = "EPSTATUS (rw) register accessor: Provides information on which endpoint's EasyDMA registers have been captured\n\nYou can [`read`](crate::Reg::read) this register and get [`epstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatus`] module"]
#[doc(alias = "EPSTATUS")]
pub type Epstatus = crate::Reg<epstatus::EpstatusSpec>;
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
pub mod epstatus;
#[doc = "EPDATASTATUS (rw) register accessor: Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)\n\nYou can [`read`](crate::Reg::read) this register and get [`epdatastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epdatastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdatastatus`] module"]
#[doc(alias = "EPDATASTATUS")]
pub type Epdatastatus = crate::Reg<epdatastatus::EpdatastatusSpec>;
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
pub mod epdatastatus;
#[doc = "USBADDR (r) register accessor: Device USB address\n\nYou can [`read`](crate::Reg::read) this register and get [`usbaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbaddr`] module"]
#[doc(alias = "USBADDR")]
pub type Usbaddr = crate::Reg<usbaddr::UsbaddrSpec>;
#[doc = "Device USB address"]
pub mod usbaddr;
#[doc = "BMREQUESTTYPE (r) register accessor: SETUP data, byte 0, bmRequestType\n\nYou can [`read`](crate::Reg::read) this register and get [`bmrequesttype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmrequesttype`] module"]
#[doc(alias = "BMREQUESTTYPE")]
pub type Bmrequesttype = crate::Reg<bmrequesttype::BmrequesttypeSpec>;
#[doc = "SETUP data, byte 0, bmRequestType"]
pub mod bmrequesttype;
#[doc = "BREQUEST (r) register accessor: SETUP data, byte 1, bRequest\n\nYou can [`read`](crate::Reg::read) this register and get [`brequest::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brequest`] module"]
#[doc(alias = "BREQUEST")]
pub type Brequest = crate::Reg<brequest::BrequestSpec>;
#[doc = "SETUP data, byte 1, bRequest"]
pub mod brequest;
#[doc = "WVALUEL (r) register accessor: SETUP data, byte 2, LSB of wValue\n\nYou can [`read`](crate::Reg::read) this register and get [`wvaluel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wvaluel`] module"]
#[doc(alias = "WVALUEL")]
pub type Wvaluel = crate::Reg<wvaluel::WvaluelSpec>;
#[doc = "SETUP data, byte 2, LSB of wValue"]
pub mod wvaluel;
#[doc = "WVALUEH (r) register accessor: SETUP data, byte 3, MSB of wValue\n\nYou can [`read`](crate::Reg::read) this register and get [`wvalueh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wvalueh`] module"]
#[doc(alias = "WVALUEH")]
pub type Wvalueh = crate::Reg<wvalueh::WvaluehSpec>;
#[doc = "SETUP data, byte 3, MSB of wValue"]
pub mod wvalueh;
#[doc = "WINDEXL (r) register accessor: SETUP data, byte 4, LSB of wIndex\n\nYou can [`read`](crate::Reg::read) this register and get [`windexl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@windexl`] module"]
#[doc(alias = "WINDEXL")]
pub type Windexl = crate::Reg<windexl::WindexlSpec>;
#[doc = "SETUP data, byte 4, LSB of wIndex"]
pub mod windexl;
#[doc = "WINDEXH (r) register accessor: SETUP data, byte 5, MSB of wIndex\n\nYou can [`read`](crate::Reg::read) this register and get [`windexh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@windexh`] module"]
#[doc(alias = "WINDEXH")]
pub type Windexh = crate::Reg<windexh::WindexhSpec>;
#[doc = "SETUP data, byte 5, MSB of wIndex"]
pub mod windexh;
#[doc = "WLENGTHL (r) register accessor: SETUP data, byte 6, LSB of wLength\n\nYou can [`read`](crate::Reg::read) this register and get [`wlengthl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlengthl`] module"]
#[doc(alias = "WLENGTHL")]
pub type Wlengthl = crate::Reg<wlengthl::WlengthlSpec>;
#[doc = "SETUP data, byte 6, LSB of wLength"]
pub mod wlengthl;
#[doc = "WLENGTHH (r) register accessor: SETUP data, byte 7, MSB of wLength\n\nYou can [`read`](crate::Reg::read) this register and get [`wlengthh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlengthh`] module"]
#[doc(alias = "WLENGTHH")]
pub type Wlengthh = crate::Reg<wlengthh::WlengthhSpec>;
#[doc = "SETUP data, byte 7, MSB of wLength"]
pub mod wlengthh;
#[doc = "Unspecified"]
pub use self::size::Size;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod size;
#[doc = "ENABLE (rw) register accessor: Enable USB\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable USB"]
pub mod enable;
#[doc = "USBPULLUP (rw) register accessor: Control of the USB pull-up\n\nYou can [`read`](crate::Reg::read) this register and get [`usbpullup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbpullup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpullup`] module"]
#[doc(alias = "USBPULLUP")]
pub type Usbpullup = crate::Reg<usbpullup::UsbpullupSpec>;
#[doc = "Control of the USB pull-up"]
pub mod usbpullup;
#[doc = "DPDMVALUE (rw) register accessor: State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing).\n\nYou can [`read`](crate::Reg::read) this register and get [`dpdmvalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdmvalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpdmvalue`] module"]
#[doc(alias = "DPDMVALUE")]
pub type Dpdmvalue = crate::Reg<dpdmvalue::DpdmvalueSpec>;
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
pub mod dpdmvalue;
#[doc = "DTOGGLE (rw) register accessor: Data toggle control and status\n\nYou can [`read`](crate::Reg::read) this register and get [`dtoggle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtoggle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtoggle`] module"]
#[doc(alias = "DTOGGLE")]
pub type Dtoggle = crate::Reg<dtoggle::DtoggleSpec>;
#[doc = "Data toggle control and status"]
pub mod dtoggle;
#[doc = "EPINEN (rw) register accessor: Endpoint IN enable\n\nYou can [`read`](crate::Reg::read) this register and get [`epinen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinen`] module"]
#[doc(alias = "EPINEN")]
pub type Epinen = crate::Reg<epinen::EpinenSpec>;
#[doc = "Endpoint IN enable"]
pub mod epinen;
#[doc = "EPOUTEN (rw) register accessor: Endpoint OUT enable\n\nYou can [`read`](crate::Reg::read) this register and get [`epouten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epouten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epouten`] module"]
#[doc(alias = "EPOUTEN")]
pub type Epouten = crate::Reg<epouten::EpoutenSpec>;
#[doc = "Endpoint OUT enable"]
pub mod epouten;
#[doc = "EPSTALL (w) register accessor: STALL endpoints\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstall::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstall`] module"]
#[doc(alias = "EPSTALL")]
pub type Epstall = crate::Reg<epstall::EpstallSpec>;
#[doc = "STALL endpoints"]
pub mod epstall;
#[doc = "ISOSPLIT (rw) register accessor: Controls the split of ISO buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`isosplit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isosplit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isosplit`] module"]
#[doc(alias = "ISOSPLIT")]
pub type Isosplit = crate::Reg<isosplit::IsosplitSpec>;
#[doc = "Controls the split of ISO buffers"]
pub mod isosplit;
#[doc = "FRAMECNTR (r) register accessor: Returns the current value of the start of frame counter\n\nYou can [`read`](crate::Reg::read) this register and get [`framecntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framecntr`] module"]
#[doc(alias = "FRAMECNTR")]
pub type Framecntr = crate::Reg<framecntr::FramecntrSpec>;
#[doc = "Returns the current value of the start of frame counter"]
pub mod framecntr;
#[doc = "LOWPOWER (rw) register accessor: Controls USBD peripheral low power mode during USB suspend\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower`] module"]
#[doc(alias = "LOWPOWER")]
pub type Lowpower = crate::Reg<lowpower::LowpowerSpec>;
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
pub mod lowpower;
#[doc = "ISOINCONFIG (rw) register accessor: Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nYou can [`read`](crate::Reg::read) this register and get [`isoinconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoinconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoinconfig`] module"]
#[doc(alias = "ISOINCONFIG")]
pub type Isoinconfig = crate::Reg<isoinconfig::IsoinconfigSpec>;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub mod isoinconfig;
#[doc = "Unspecified"]
pub use self::epin::Epin;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod epin;
#[doc = "Unspecified"]
pub use self::isoin::Isoin;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod isoin;
#[doc = "Unspecified"]
pub use self::epout::Epout;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod epout;
#[doc = "Unspecified"]
pub use self::isoout::Isoout;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod isoout;
