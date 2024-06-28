#[doc = "Register `EVENTS_RXSTARTED` reader"]
pub type R = crate::R<EventsRxstartedSpec>;
#[doc = "Register `EVENTS_RXSTARTED` writer"]
pub type W = crate::W<EventsRxstartedSpec>;
#[doc = "Receive sequence started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsRxstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXSTARTED` reader - Receive sequence started"]
pub type EventsRxstartedR = crate::BitReader<EventsRxstarted>;
impl EventsRxstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxstarted {
        match self.bits {
            false => EventsRxstarted::NotGenerated,
            true => EventsRxstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxstarted::Generated
    }
}
#[doc = "Field `EVENTS_RXSTARTED` writer - Receive sequence started"]
pub type EventsRxstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsRxstarted>;
impl<'a, REG> EventsRxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Receive sequence started"]
    #[inline(always)]
    pub fn events_rxstarted(&self) -> EventsRxstartedR {
        EventsRxstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive sequence started"]
    #[inline(always)]
    #[must_use]
    pub fn events_rxstarted(&mut self) -> EventsRxstartedW<EventsRxstartedSpec> {
        EventsRxstartedW::new(self, 0)
    }
}
#[doc = "Receive sequence started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_rxstarted::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_rxstarted::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxstartedSpec;
impl crate::RegisterSpec for EventsRxstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxstarted::R`](R) reader structure"]
impl crate::Readable for EventsRxstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxstarted::W`](W) writer structure"]
impl crate::Writable for EventsRxstartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RXSTARTED to value 0"]
impl crate::Resettable for EventsRxstartedSpec {
    const RESET_VALUE: u32 = 0;
}
