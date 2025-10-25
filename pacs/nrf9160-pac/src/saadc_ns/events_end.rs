#[doc = "Register `EVENTS_END` reader"]
pub type R = crate::R<EventsEndSpec>;
#[doc = "Register `EVENTS_END` writer"]
pub type W = crate::W<EventsEndSpec>;
#[doc = "The ADC has filled up the Result buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEnd {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEnd> for bool {
    #[inline(always)]
    fn from(variant: EventsEnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_END` reader - The ADC has filled up the Result buffer"]
pub type EventsEndR = crate::BitReader<EventsEnd>;
impl EventsEndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEnd {
        match self.bits {
            false => EventsEnd::NotGenerated,
            true => EventsEnd::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEnd::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEnd::Generated
    }
}
#[doc = "Field `EVENTS_END` writer - The ADC has filled up the Result buffer"]
pub type EventsEndW<'a, REG> = crate::BitWriter<'a, REG, EventsEnd>;
impl<'a, REG> EventsEndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEnd::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEnd::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub fn events_end(&self) -> EventsEndR {
        EventsEndR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub fn events_end(&mut self) -> EventsEndW<'_, EventsEndSpec> {
        EventsEndW::new(self, 0)
    }
}
#[doc = "The ADC has filled up the Result buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndSpec;
impl crate::RegisterSpec for EventsEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_end::R`](R) reader structure"]
impl crate::Readable for EventsEndSpec {}
#[doc = "`write(|w| ..)` method takes [`events_end::W`](W) writer structure"]
impl crate::Writable for EventsEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_END to value 0"]
impl crate::Resettable for EventsEndSpec {}
