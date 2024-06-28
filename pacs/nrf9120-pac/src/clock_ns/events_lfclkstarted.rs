#[doc = "Register `EVENTS_LFCLKSTARTED` reader"]
pub type R = crate::R<EventsLfclkstartedSpec>;
#[doc = "Register `EVENTS_LFCLKSTARTED` writer"]
pub type W = crate::W<EventsLfclkstartedSpec>;
#[doc = "LFCLK started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsLfclkstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsLfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsLfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_LFCLKSTARTED` reader - LFCLK started"]
pub type EventsLfclkstartedR = crate::BitReader<EventsLfclkstarted>;
impl EventsLfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsLfclkstarted {
        match self.bits {
            false => EventsLfclkstarted::NotGenerated,
            true => EventsLfclkstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsLfclkstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsLfclkstarted::Generated
    }
}
#[doc = "Field `EVENTS_LFCLKSTARTED` writer - LFCLK started"]
pub type EventsLfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsLfclkstarted>;
impl<'a, REG> EventsLfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLfclkstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLfclkstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - LFCLK started"]
    #[inline(always)]
    pub fn events_lfclkstarted(&self) -> EventsLfclkstartedR {
        EventsLfclkstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFCLK started"]
    #[inline(always)]
    #[must_use]
    pub fn events_lfclkstarted(&mut self) -> EventsLfclkstartedW<EventsLfclkstartedSpec> {
        EventsLfclkstartedW::new(self, 0)
    }
}
#[doc = "LFCLK started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_lfclkstarted::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_lfclkstarted::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLfclkstartedSpec;
impl crate::RegisterSpec for EventsLfclkstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lfclkstarted::R`](R) reader structure"]
impl crate::Readable for EventsLfclkstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lfclkstarted::W`](W) writer structure"]
impl crate::Writable for EventsLfclkstartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_LFCLKSTARTED to value 0"]
impl crate::Resettable for EventsLfclkstartedSpec {
    const RESET_VALUE: u32 = 0;
}
