#[doc = "Register `EVENTS_RXDREADY` reader"]
pub type R = crate::R<EventsRxdreadySpec>;
#[doc = "Register `EVENTS_RXDREADY` writer"]
pub type W = crate::W<EventsRxdreadySpec>;
#[doc = "TWI RXD byte received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxdready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxdready> for bool {
    #[inline(always)]
    fn from(variant: EventsRxdready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXDREADY` reader - TWI RXD byte received"]
pub type EventsRxdreadyR = crate::BitReader<EventsRxdready>;
impl EventsRxdreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxdready {
        match self.bits {
            false => EventsRxdready::NotGenerated,
            true => EventsRxdready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxdready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxdready::Generated
    }
}
#[doc = "Field `EVENTS_RXDREADY` writer - TWI RXD byte received"]
pub type EventsRxdreadyW<'a, REG> = crate::BitWriter<'a, REG, EventsRxdready>;
impl<'a, REG> EventsRxdreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxdready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxdready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - TWI RXD byte received"]
    #[inline(always)]
    pub fn events_rxdready(&self) -> EventsRxdreadyR {
        EventsRxdreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI RXD byte received"]
    #[inline(always)]
    pub fn events_rxdready(&mut self) -> EventsRxdreadyW<'_, EventsRxdreadySpec> {
        EventsRxdreadyW::new(self, 0)
    }
}
#[doc = "TWI RXD byte received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxdreadySpec;
impl crate::RegisterSpec for EventsRxdreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxdready::R`](R) reader structure"]
impl crate::Readable for EventsRxdreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxdready::W`](W) writer structure"]
impl crate::Writable for EventsRxdreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXDREADY to value 0"]
impl crate::Resettable for EventsRxdreadySpec {}
