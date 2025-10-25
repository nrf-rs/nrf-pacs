#[doc = "Register `EVENTS_HFCLKAUDIOSTARTED` reader"]
pub type R = crate::R<EventsHfclkaudiostartedSpec>;
#[doc = "Register `EVENTS_HFCLKAUDIOSTARTED` writer"]
pub type W = crate::W<EventsHfclkaudiostartedSpec>;
#[doc = "HFCLKAUDIO source started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsHfclkaudiostarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsHfclkaudiostarted> for bool {
    #[inline(always)]
    fn from(variant: EventsHfclkaudiostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_HFCLKAUDIOSTARTED` reader - HFCLKAUDIO source started"]
pub type EventsHfclkaudiostartedR = crate::BitReader<EventsHfclkaudiostarted>;
impl EventsHfclkaudiostartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsHfclkaudiostarted {
        match self.bits {
            false => EventsHfclkaudiostarted::NotGenerated,
            true => EventsHfclkaudiostarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsHfclkaudiostarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsHfclkaudiostarted::Generated
    }
}
#[doc = "Field `EVENTS_HFCLKAUDIOSTARTED` writer - HFCLKAUDIO source started"]
pub type EventsHfclkaudiostartedW<'a, REG> = crate::BitWriter<'a, REG, EventsHfclkaudiostarted>;
impl<'a, REG> EventsHfclkaudiostartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclkaudiostarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclkaudiostarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFCLKAUDIO source started"]
    #[inline(always)]
    pub fn events_hfclkaudiostarted(&self) -> EventsHfclkaudiostartedR {
        EventsHfclkaudiostartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFCLKAUDIO source started"]
    #[inline(always)]
    pub fn events_hfclkaudiostarted(
        &mut self,
    ) -> EventsHfclkaudiostartedW<'_, EventsHfclkaudiostartedSpec> {
        EventsHfclkaudiostartedW::new(self, 0)
    }
}
#[doc = "HFCLKAUDIO source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkaudiostarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkaudiostarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsHfclkaudiostartedSpec;
impl crate::RegisterSpec for EventsHfclkaudiostartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_hfclkaudiostarted::R`](R) reader structure"]
impl crate::Readable for EventsHfclkaudiostartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_hfclkaudiostarted::W`](W) writer structure"]
impl crate::Writable for EventsHfclkaudiostartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_HFCLKAUDIOSTARTED to value 0"]
impl crate::Resettable for EventsHfclkaudiostartedSpec {}
