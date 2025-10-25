#[doc = "Register `EVENTS_USBPWRRDY` reader"]
pub type R = crate::R<EventsUsbpwrrdySpec>;
#[doc = "Register `EVENTS_USBPWRRDY` writer"]
pub type W = crate::W<EventsUsbpwrrdySpec>;
#[doc = "USB 3.3 V supply ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUsbpwrrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUsbpwrrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsUsbpwrrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_USBPWRRDY` reader - USB 3.3 V supply ready"]
pub type EventsUsbpwrrdyR = crate::BitReader<EventsUsbpwrrdy>;
impl EventsUsbpwrrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUsbpwrrdy {
        match self.bits {
            false => EventsUsbpwrrdy::NotGenerated,
            true => EventsUsbpwrrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUsbpwrrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUsbpwrrdy::Generated
    }
}
#[doc = "Field `EVENTS_USBPWRRDY` writer - USB 3.3 V supply ready"]
pub type EventsUsbpwrrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsUsbpwrrdy>;
impl<'a, REG> EventsUsbpwrrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbpwrrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbpwrrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - USB 3.3 V supply ready"]
    #[inline(always)]
    pub fn events_usbpwrrdy(&self) -> EventsUsbpwrrdyR {
        EventsUsbpwrrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB 3.3 V supply ready"]
    #[inline(always)]
    pub fn events_usbpwrrdy(&mut self) -> EventsUsbpwrrdyW<'_, EventsUsbpwrrdySpec> {
        EventsUsbpwrrdyW::new(self, 0)
    }
}
#[doc = "USB 3.3 V supply ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbpwrrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbpwrrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUsbpwrrdySpec;
impl crate::RegisterSpec for EventsUsbpwrrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_usbpwrrdy::R`](R) reader structure"]
impl crate::Readable for EventsUsbpwrrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_usbpwrrdy::W`](W) writer structure"]
impl crate::Writable for EventsUsbpwrrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_USBPWRRDY to value 0"]
impl crate::Resettable for EventsUsbpwrrdySpec {}
