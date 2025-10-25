#[doc = "Register `EVENTS_RXTO` reader"]
pub type R = crate::R<EventsRxtoSpec>;
#[doc = "Register `EVENTS_RXTO` writer"]
pub type W = crate::W<EventsRxtoSpec>;
#[doc = "Receiver timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxto {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxto> for bool {
    #[inline(always)]
    fn from(variant: EventsRxto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXTO` reader - Receiver timeout"]
pub type EventsRxtoR = crate::BitReader<EventsRxto>;
impl EventsRxtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxto {
        match self.bits {
            false => EventsRxto::NotGenerated,
            true => EventsRxto::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxto::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxto::Generated
    }
}
#[doc = "Field `EVENTS_RXTO` writer - Receiver timeout"]
pub type EventsRxtoW<'a, REG> = crate::BitWriter<'a, REG, EventsRxto>;
impl<'a, REG> EventsRxtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxto::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxto::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver timeout"]
    #[inline(always)]
    pub fn events_rxto(&self) -> EventsRxtoR {
        EventsRxtoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver timeout"]
    #[inline(always)]
    pub fn events_rxto(&mut self) -> EventsRxtoW<'_, EventsRxtoSpec> {
        EventsRxtoW::new(self, 0)
    }
}
#[doc = "Receiver timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxtoSpec;
impl crate::RegisterSpec for EventsRxtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxto::R`](R) reader structure"]
impl crate::Readable for EventsRxtoSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxto::W`](W) writer structure"]
impl crate::Writable for EventsRxtoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXTO to value 0"]
impl crate::Resettable for EventsRxtoSpec {}
