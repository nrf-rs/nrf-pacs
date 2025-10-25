#[doc = "Register `EVENTS_USBEVENT` reader"]
pub type R = crate::R<EventsUsbeventSpec>;
#[doc = "Register `EVENTS_USBEVENT` writer"]
pub type W = crate::W<EventsUsbeventSpec>;
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUsbevent {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUsbevent> for bool {
    #[inline(always)]
    fn from(variant: EventsUsbevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_USBEVENT` reader - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub type EventsUsbeventR = crate::BitReader<EventsUsbevent>;
impl EventsUsbeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUsbevent {
        match self.bits {
            false => EventsUsbevent::NotGenerated,
            true => EventsUsbevent::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUsbevent::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUsbevent::Generated
    }
}
#[doc = "Field `EVENTS_USBEVENT` writer - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub type EventsUsbeventW<'a, REG> = crate::BitWriter<'a, REG, EventsUsbevent>;
impl<'a, REG> EventsUsbeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbevent::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbevent::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    #[inline(always)]
    pub fn events_usbevent(&self) -> EventsUsbeventR {
        EventsUsbeventR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    #[inline(always)]
    pub fn events_usbevent(&mut self) -> EventsUsbeventW<'_, EventsUsbeventSpec> {
        EventsUsbeventW::new(self, 0)
    }
}
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbevent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbevent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUsbeventSpec;
impl crate::RegisterSpec for EventsUsbeventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_usbevent::R`](R) reader structure"]
impl crate::Readable for EventsUsbeventSpec {}
#[doc = "`write(|w| ..)` method takes [`events_usbevent::W`](W) writer structure"]
impl crate::Writable for EventsUsbeventSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_USBEVENT to value 0"]
impl crate::Resettable for EventsUsbeventSpec {}
