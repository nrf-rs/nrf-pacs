#[doc = "Register `EVENTS_RXFRAMESTART` reader"]
pub type R = crate::R<EventsRxframestartSpec>;
#[doc = "Register `EVENTS_RXFRAMESTART` writer"]
pub type W = crate::W<EventsRxframestartSpec>;
#[doc = "Marks the end of the first symbol of a received frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsRxframestart {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsRxframestart> for bool {
    #[inline(always)]
    fn from(variant: EventsRxframestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RXFRAMESTART` reader - Marks the end of the first symbol of a received frame"]
pub type EventsRxframestartR = crate::BitReader<EventsRxframestart>;
impl EventsRxframestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsRxframestart {
        match self.bits {
            false => EventsRxframestart::NotGenerated,
            true => EventsRxframestart::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsRxframestart::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsRxframestart::Generated
    }
}
#[doc = "Field `EVENTS_RXFRAMESTART` writer - Marks the end of the first symbol of a received frame"]
pub type EventsRxframestartW<'a, REG> = crate::BitWriter<'a, REG, EventsRxframestart>;
impl<'a, REG> EventsRxframestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxframestart::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsRxframestart::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Marks the end of the first symbol of a received frame"]
    #[inline(always)]
    pub fn events_rxframestart(&self) -> EventsRxframestartR {
        EventsRxframestartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the end of the first symbol of a received frame"]
    #[inline(always)]
    pub fn events_rxframestart(&mut self) -> EventsRxframestartW<'_, EventsRxframestartSpec> {
        EventsRxframestartW::new(self, 0)
    }
}
#[doc = "Marks the end of the first symbol of a received frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxframestart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxframestart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxframestartSpec;
impl crate::RegisterSpec for EventsRxframestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxframestart::R`](R) reader structure"]
impl crate::Readable for EventsRxframestartSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxframestart::W`](W) writer structure"]
impl crate::Writable for EventsRxframestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXFRAMESTART to value 0"]
impl crate::Resettable for EventsRxframestartSpec {}
