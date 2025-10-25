#[doc = "Register `EVENTS_VALRDY` reader"]
pub type R = crate::R<EventsValrdySpec>;
#[doc = "Register `EVENTS_VALRDY` writer"]
pub type W = crate::W<EventsValrdySpec>;
#[doc = "Event being generated for every new random number written to the VALUE register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsValrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsValrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsValrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_VALRDY` reader - Event being generated for every new random number written to the VALUE register"]
pub type EventsValrdyR = crate::BitReader<EventsValrdy>;
impl EventsValrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsValrdy {
        match self.bits {
            false => EventsValrdy::NotGenerated,
            true => EventsValrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsValrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsValrdy::Generated
    }
}
#[doc = "Field `EVENTS_VALRDY` writer - Event being generated for every new random number written to the VALUE register"]
pub type EventsValrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsValrdy>;
impl<'a, REG> EventsValrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsValrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsValrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event being generated for every new random number written to the VALUE register"]
    #[inline(always)]
    pub fn events_valrdy(&self) -> EventsValrdyR {
        EventsValrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event being generated for every new random number written to the VALUE register"]
    #[inline(always)]
    pub fn events_valrdy(&mut self) -> EventsValrdyW<'_, EventsValrdySpec> {
        EventsValrdyW::new(self, 0)
    }
}
#[doc = "Event being generated for every new random number written to the VALUE register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_valrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_valrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsValrdySpec;
impl crate::RegisterSpec for EventsValrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_valrdy::R`](R) reader structure"]
impl crate::Readable for EventsValrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_valrdy::W`](W) writer structure"]
impl crate::Writable for EventsValrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_VALRDY to value 0"]
impl crate::Resettable for EventsValrdySpec {}
