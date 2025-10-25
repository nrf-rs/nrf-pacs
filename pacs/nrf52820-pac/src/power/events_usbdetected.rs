#[doc = "Register `EVENTS_USBDETECTED` reader"]
pub type R = crate::R<EventsUsbdetectedSpec>;
#[doc = "Register `EVENTS_USBDETECTED` writer"]
pub type W = crate::W<EventsUsbdetectedSpec>;
#[doc = "Voltage supply detected on VBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUsbdetected {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUsbdetected> for bool {
    #[inline(always)]
    fn from(variant: EventsUsbdetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_USBDETECTED` reader - Voltage supply detected on VBUS"]
pub type EventsUsbdetectedR = crate::BitReader<EventsUsbdetected>;
impl EventsUsbdetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUsbdetected {
        match self.bits {
            false => EventsUsbdetected::NotGenerated,
            true => EventsUsbdetected::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUsbdetected::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUsbdetected::Generated
    }
}
#[doc = "Field `EVENTS_USBDETECTED` writer - Voltage supply detected on VBUS"]
pub type EventsUsbdetectedW<'a, REG> = crate::BitWriter<'a, REG, EventsUsbdetected>;
impl<'a, REG> EventsUsbdetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbdetected::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbdetected::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage supply detected on VBUS"]
    #[inline(always)]
    pub fn events_usbdetected(&self) -> EventsUsbdetectedR {
        EventsUsbdetectedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage supply detected on VBUS"]
    #[inline(always)]
    pub fn events_usbdetected(&mut self) -> EventsUsbdetectedW<'_, EventsUsbdetectedSpec> {
        EventsUsbdetectedW::new(self, 0)
    }
}
#[doc = "Voltage supply detected on VBUS\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbdetected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbdetected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUsbdetectedSpec;
impl crate::RegisterSpec for EventsUsbdetectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_usbdetected::R`](R) reader structure"]
impl crate::Readable for EventsUsbdetectedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_usbdetected::W`](W) writer structure"]
impl crate::Writable for EventsUsbdetectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_USBDETECTED to value 0"]
impl crate::Resettable for EventsUsbdetectedSpec {}
