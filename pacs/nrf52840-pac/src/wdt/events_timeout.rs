#[doc = "Register `EVENTS_TIMEOUT` reader"]
pub type R = crate::R<EventsTimeoutSpec>;
#[doc = "Register `EVENTS_TIMEOUT` writer"]
pub type W = crate::W<EventsTimeoutSpec>;
#[doc = "Watchdog timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTimeout {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTimeout> for bool {
    #[inline(always)]
    fn from(variant: EventsTimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TIMEOUT` reader - Watchdog timeout"]
pub type EventsTimeoutR = crate::BitReader<EventsTimeout>;
impl EventsTimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTimeout {
        match self.bits {
            false => EventsTimeout::NotGenerated,
            true => EventsTimeout::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTimeout::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTimeout::Generated
    }
}
#[doc = "Field `EVENTS_TIMEOUT` writer - Watchdog timeout"]
pub type EventsTimeoutW<'a, REG> = crate::BitWriter<'a, REG, EventsTimeout>;
impl<'a, REG> EventsTimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTimeout::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTimeout::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timeout"]
    #[inline(always)]
    pub fn events_timeout(&self) -> EventsTimeoutR {
        EventsTimeoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timeout"]
    #[inline(always)]
    pub fn events_timeout(&mut self) -> EventsTimeoutW<'_, EventsTimeoutSpec> {
        EventsTimeoutW::new(self, 0)
    }
}
#[doc = "Watchdog timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTimeoutSpec;
impl crate::RegisterSpec for EventsTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_timeout::R`](R) reader structure"]
impl crate::Readable for EventsTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`events_timeout::W`](W) writer structure"]
impl crate::Writable for EventsTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TIMEOUT to value 0"]
impl crate::Resettable for EventsTimeoutSpec {}
