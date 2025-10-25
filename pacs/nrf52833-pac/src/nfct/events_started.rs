#[doc = "Register `EVENTS_STARTED` reader"]
pub type R = crate::R<EventsStartedSpec>;
#[doc = "Register `EVENTS_STARTED` writer"]
pub type W = crate::W<EventsStartedSpec>;
#[doc = "EasyDMA is ready to receive or send frames.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsStarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsStarted> for bool {
    #[inline(always)]
    fn from(variant: EventsStarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_STARTED` reader - EasyDMA is ready to receive or send frames."]
pub type EventsStartedR = crate::BitReader<EventsStarted>;
impl EventsStartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsStarted {
        match self.bits {
            false => EventsStarted::NotGenerated,
            true => EventsStarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsStarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsStarted::Generated
    }
}
#[doc = "Field `EVENTS_STARTED` writer - EasyDMA is ready to receive or send frames."]
pub type EventsStartedW<'a, REG> = crate::BitWriter<'a, REG, EventsStarted>;
impl<'a, REG> EventsStartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsStarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsStarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - EasyDMA is ready to receive or send frames."]
    #[inline(always)]
    pub fn events_started(&self) -> EventsStartedR {
        EventsStartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EasyDMA is ready to receive or send frames."]
    #[inline(always)]
    pub fn events_started(&mut self) -> EventsStartedW<'_, EventsStartedSpec> {
        EventsStartedW::new(self, 0)
    }
}
#[doc = "EasyDMA is ready to receive or send frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsStartedSpec;
impl crate::RegisterSpec for EventsStartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_started::R`](R) reader structure"]
impl crate::Readable for EventsStartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_started::W`](W) writer structure"]
impl crate::Writable for EventsStartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_STARTED to value 0"]
impl crate::Resettable for EventsStartedSpec {}
