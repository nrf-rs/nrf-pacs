#[doc = "Register `EVENTS_NOTRESOLVED` reader"]
pub type R = crate::R<EventsNotresolvedSpec>;
#[doc = "Register `EVENTS_NOTRESOLVED` writer"]
pub type W = crate::W<EventsNotresolvedSpec>;
#[doc = "Address not resolved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsNotresolved {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsNotresolved> for bool {
    #[inline(always)]
    fn from(variant: EventsNotresolved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_NOTRESOLVED` reader - Address not resolved"]
pub type EventsNotresolvedR = crate::BitReader<EventsNotresolved>;
impl EventsNotresolvedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsNotresolved {
        match self.bits {
            false => EventsNotresolved::NotGenerated,
            true => EventsNotresolved::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsNotresolved::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsNotresolved::Generated
    }
}
#[doc = "Field `EVENTS_NOTRESOLVED` writer - Address not resolved"]
pub type EventsNotresolvedW<'a, REG> = crate::BitWriter<'a, REG, EventsNotresolved>;
impl<'a, REG> EventsNotresolvedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsNotresolved::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsNotresolved::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Address not resolved"]
    #[inline(always)]
    pub fn events_notresolved(&self) -> EventsNotresolvedR {
        EventsNotresolvedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address not resolved"]
    #[inline(always)]
    pub fn events_notresolved(&mut self) -> EventsNotresolvedW<'_, EventsNotresolvedSpec> {
        EventsNotresolvedW::new(self, 0)
    }
}
#[doc = "Address not resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_notresolved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_notresolved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsNotresolvedSpec;
impl crate::RegisterSpec for EventsNotresolvedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_notresolved::R`](R) reader structure"]
impl crate::Readable for EventsNotresolvedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_notresolved::W`](W) writer structure"]
impl crate::Writable for EventsNotresolvedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_NOTRESOLVED to value 0"]
impl crate::Resettable for EventsNotresolvedSpec {}
