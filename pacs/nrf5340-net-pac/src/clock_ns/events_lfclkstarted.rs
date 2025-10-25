#[doc = "Register `EVENTS_LFCLKSTARTED` reader"]
pub type R = crate::R<EventsLfclkstartedSpec>;
#[doc = "Register `EVENTS_LFCLKSTARTED` writer"]
pub type W = crate::W<EventsLfclkstartedSpec>;
#[doc = "LFCLK source started\n\nValue on reset: 0"]
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
#[doc = "Field `EVENTS_LFCLKSTARTED` reader - LFCLK source started"]
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
#[doc = "Field `EVENTS_LFCLKSTARTED` writer - LFCLK source started"]
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
    #[doc = "Bit 0 - LFCLK source started"]
    #[inline(always)]
    pub fn events_lfclkstarted(&self) -> EventsLfclkstartedR {
        EventsLfclkstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFCLK source started"]
    #[inline(always)]
    pub fn events_lfclkstarted(&mut self) -> EventsLfclkstartedW<'_, EventsLfclkstartedSpec> {
        EventsLfclkstartedW::new(self, 0)
    }
}
#[doc = "LFCLK source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLfclkstartedSpec;
impl crate::RegisterSpec for EventsLfclkstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lfclkstarted::R`](R) reader structure"]
impl crate::Readable for EventsLfclkstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lfclkstarted::W`](W) writer structure"]
impl crate::Writable for EventsLfclkstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LFCLKSTARTED to value 0"]
impl crate::Resettable for EventsLfclkstartedSpec {}
