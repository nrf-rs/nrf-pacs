#[doc = "Register `EVENTS_USBREMOVED` reader"]
pub type R = crate::R<EventsUsbremovedSpec>;
#[doc = "Register `EVENTS_USBREMOVED` writer"]
pub type W = crate::W<EventsUsbremovedSpec>;
#[doc = "Voltage supply removed from VBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUsbremoved {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUsbremoved> for bool {
    #[inline(always)]
    fn from(variant: EventsUsbremoved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_USBREMOVED` reader - Voltage supply removed from VBUS"]
pub type EventsUsbremovedR = crate::BitReader<EventsUsbremoved>;
impl EventsUsbremovedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUsbremoved {
        match self.bits {
            false => EventsUsbremoved::NotGenerated,
            true => EventsUsbremoved::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUsbremoved::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUsbremoved::Generated
    }
}
#[doc = "Field `EVENTS_USBREMOVED` writer - Voltage supply removed from VBUS"]
pub type EventsUsbremovedW<'a, REG> = crate::BitWriter<'a, REG, EventsUsbremoved>;
impl<'a, REG> EventsUsbremovedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbremoved::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbremoved::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage supply removed from VBUS"]
    #[inline(always)]
    pub fn events_usbremoved(&self) -> EventsUsbremovedR {
        EventsUsbremovedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage supply removed from VBUS"]
    #[inline(always)]
    pub fn events_usbremoved(&mut self) -> EventsUsbremovedW<'_, EventsUsbremovedSpec> {
        EventsUsbremovedW::new(self, 0)
    }
}
#[doc = "Voltage supply removed from VBUS\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbremoved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbremoved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUsbremovedSpec;
impl crate::RegisterSpec for EventsUsbremovedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_usbremoved::R`](R) reader structure"]
impl crate::Readable for EventsUsbremovedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_usbremoved::W`](W) writer structure"]
impl crate::Writable for EventsUsbremovedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_USBREMOVED to value 0"]
impl crate::Resettable for EventsUsbremovedSpec {}
