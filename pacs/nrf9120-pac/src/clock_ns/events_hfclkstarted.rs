#[doc = "Register `EVENTS_HFCLKSTARTED` reader"]
pub type R = crate::R<EventsHfclkstartedSpec>;
#[doc = "Register `EVENTS_HFCLKSTARTED` writer"]
pub type W = crate::W<EventsHfclkstartedSpec>;
#[doc = "HFCLK oscillator started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsHfclkstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsHfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsHfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_HFCLKSTARTED` reader - HFCLK oscillator started"]
pub type EventsHfclkstartedR = crate::BitReader<EventsHfclkstarted>;
impl EventsHfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsHfclkstarted {
        match self.bits {
            false => EventsHfclkstarted::NotGenerated,
            true => EventsHfclkstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsHfclkstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsHfclkstarted::Generated
    }
}
#[doc = "Field `EVENTS_HFCLKSTARTED` writer - HFCLK oscillator started"]
pub type EventsHfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsHfclkstarted>;
impl<'a, REG> EventsHfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclkstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsHfclkstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - HFCLK oscillator started"]
    #[inline(always)]
    pub fn events_hfclkstarted(&self) -> EventsHfclkstartedR {
        EventsHfclkstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFCLK oscillator started"]
    #[inline(always)]
    #[must_use]
    pub fn events_hfclkstarted(&mut self) -> EventsHfclkstartedW<EventsHfclkstartedSpec> {
        EventsHfclkstartedW::new(self, 0)
    }
}
#[doc = "HFCLK oscillator started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_hfclkstarted::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_hfclkstarted::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsHfclkstartedSpec;
impl crate::RegisterSpec for EventsHfclkstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_hfclkstarted::R`](R) reader structure"]
impl crate::Readable for EventsHfclkstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_hfclkstarted::W`](W) writer structure"]
impl crate::Writable for EventsHfclkstartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_HFCLKSTARTED to value 0"]
impl crate::Resettable for EventsHfclkstartedSpec {
    const RESET_VALUE: u32 = 0;
}
