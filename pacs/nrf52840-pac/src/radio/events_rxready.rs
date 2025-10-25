#[doc = "Register `EVENTS_RXREADY` reader"]
pub type R = crate::R<EventsRxreadySpec>;
#[doc = "Register `EVENTS_RXREADY` writer"]
pub type W = crate::W<EventsRxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started RX path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxready> for bool {
    #[inline(always)]
    fn from(variant: EventsRxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXREADY` reader - RADIO has ramped up and is ready to be started RX path"]
pub type EventsRxreadyR = crate::BitReader<EventsRxready>;
impl EventsRxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxready {
        match self.bits {
            false => EventsRxready::NotGenerated,
            true => EventsRxready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxready::Generated
    }
}
#[doc = "Field `EVENTS_RXREADY` writer - RADIO has ramped up and is ready to be started RX path"]
pub type EventsRxreadyW<'a, REG> = crate::BitWriter<'a, REG, EventsRxready>;
impl<'a, REG> EventsRxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub fn events_rxready(&self) -> EventsRxreadyR {
        EventsRxreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub fn events_rxready(&mut self) -> EventsRxreadyW<'_, EventsRxreadySpec> {
        EventsRxreadyW::new(self, 0)
    }
}
#[doc = "RADIO has ramped up and is ready to be started RX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxreadySpec;
impl crate::RegisterSpec for EventsRxreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxready::R`](R) reader structure"]
impl crate::Readable for EventsRxreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxready::W`](W) writer structure"]
impl crate::Writable for EventsRxreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXREADY to value 0"]
impl crate::Resettable for EventsRxreadySpec {}
