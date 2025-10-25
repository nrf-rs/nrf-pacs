#[doc = "Register `EVENTS_RESOLVED` reader"]
pub type R = crate::R<EventsResolvedSpec>;
#[doc = "Register `EVENTS_RESOLVED` writer"]
pub type W = crate::W<EventsResolvedSpec>;
#[doc = "Address resolved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsResolved {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsResolved> for bool {
    #[inline(always)]
    fn from(variant: EventsResolved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RESOLVED` reader - Address resolved"]
pub type EventsResolvedR = crate::BitReader<EventsResolved>;
impl EventsResolvedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsResolved {
        match self.bits {
            false => EventsResolved::NotGenerated,
            true => EventsResolved::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsResolved::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsResolved::Generated
    }
}
#[doc = "Field `EVENTS_RESOLVED` writer - Address resolved"]
pub type EventsResolvedW<'a, REG> = crate::BitWriter<'a, REG, EventsResolved>;
impl<'a, REG> EventsResolvedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsResolved::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsResolved::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Address resolved"]
    #[inline(always)]
    pub fn events_resolved(&self) -> EventsResolvedR {
        EventsResolvedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address resolved"]
    #[inline(always)]
    pub fn events_resolved(&mut self) -> EventsResolvedW<'_, EventsResolvedSpec> {
        EventsResolvedW::new(self, 0)
    }
}
#[doc = "Address resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resolved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resolved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsResolvedSpec;
impl crate::RegisterSpec for EventsResolvedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_resolved::R`](R) reader structure"]
impl crate::Readable for EventsResolvedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_resolved::W`](W) writer structure"]
impl crate::Writable for EventsResolvedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RESOLVED to value 0"]
impl crate::Resettable for EventsResolvedSpec {}
