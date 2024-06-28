#[doc = "Register `EVENTS_IN[%s]` reader"]
pub type R = crate::R<EventsInSpec>;
#[doc = "Register `EVENTS_IN[%s]` writer"]
pub type W = crate::W<EventsInSpec>;
#[doc = "Event generated from pin specified in CONFIG\\[n\\].PSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsIn {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsIn> for bool {
    #[inline(always)]
    fn from(variant: EventsIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_IN` reader - Event generated from pin specified in CONFIG\\[n\\].PSEL"]
pub type EventsInR = crate::BitReader<EventsIn>;
impl EventsInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsIn {
        match self.bits {
            false => EventsIn::NotGenerated,
            true => EventsIn::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsIn::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsIn::Generated
    }
}
#[doc = "Field `EVENTS_IN` writer - Event generated from pin specified in CONFIG\\[n\\].PSEL"]
pub type EventsInW<'a, REG> = crate::BitWriter<'a, REG, EventsIn>;
impl<'a, REG> EventsInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsIn::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsIn::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    pub fn events_in(&self) -> EventsInR {
        EventsInR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    #[must_use]
    pub fn events_in(&mut self) -> EventsInW<EventsInSpec> {
        EventsInW::new(self, 0)
    }
}
#[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsInSpec;
impl crate::RegisterSpec for EventsInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_in::R`](R) reader structure"]
impl crate::Readable for EventsInSpec {}
#[doc = "`write(|w| ..)` method takes [`events_in::W`](W) writer structure"]
impl crate::Writable for EventsInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_IN[%s]
to value 0"]
impl crate::Resettable for EventsInSpec {
    const RESET_VALUE: u32 = 0;
}
