#[doc = "Register `EVENTS_RXDRDY` reader"]
pub type R = crate::R<EventsRxdrdySpec>;
#[doc = "Register `EVENTS_RXDRDY` writer"]
pub type W = crate::W<EventsRxdrdySpec>;
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxdrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxdrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsRxdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXDRDY` reader - Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub type EventsRxdrdyR = crate::BitReader<EventsRxdrdy>;
impl EventsRxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxdrdy {
        match self.bits {
            false => EventsRxdrdy::NotGenerated,
            true => EventsRxdrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxdrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxdrdy::Generated
    }
}
#[doc = "Field `EVENTS_RXDRDY` writer - Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub type EventsRxdrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsRxdrdy>;
impl<'a, REG> EventsRxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxdrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxdrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    #[inline(always)]
    pub fn events_rxdrdy(&self) -> EventsRxdrdyR {
        EventsRxdrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    #[inline(always)]
    pub fn events_rxdrdy(&mut self) -> EventsRxdrdyW<'_, EventsRxdrdySpec> {
        EventsRxdrdyW::new(self, 0)
    }
}
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxdrdySpec;
impl crate::RegisterSpec for EventsRxdrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxdrdy::R`](R) reader structure"]
impl crate::Readable for EventsRxdrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxdrdy::W`](W) writer structure"]
impl crate::Writable for EventsRxdrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXDRDY to value 0"]
impl crate::Resettable for EventsRxdrdySpec {}
