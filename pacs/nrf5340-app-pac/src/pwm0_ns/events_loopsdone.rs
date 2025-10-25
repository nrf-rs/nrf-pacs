#[doc = "Register `EVENTS_LOOPSDONE` reader"]
pub type R = crate::R<EventsLoopsdoneSpec>;
#[doc = "Register `EVENTS_LOOPSDONE` writer"]
pub type W = crate::W<EventsLoopsdoneSpec>;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsLoopsdone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsLoopsdone> for bool {
    #[inline(always)]
    fn from(variant: EventsLoopsdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_LOOPSDONE` reader - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub type EventsLoopsdoneR = crate::BitReader<EventsLoopsdone>;
impl EventsLoopsdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsLoopsdone {
        match self.bits {
            false => EventsLoopsdone::NotGenerated,
            true => EventsLoopsdone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsLoopsdone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsLoopsdone::Generated
    }
}
#[doc = "Field `EVENTS_LOOPSDONE` writer - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub type EventsLoopsdoneW<'a, REG> = crate::BitWriter<'a, REG, EventsLoopsdone>;
impl<'a, REG> EventsLoopsdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLoopsdone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsLoopsdone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    #[inline(always)]
    pub fn events_loopsdone(&self) -> EventsLoopsdoneR {
        EventsLoopsdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    #[inline(always)]
    pub fn events_loopsdone(&mut self) -> EventsLoopsdoneW<'_, EventsLoopsdoneSpec> {
        EventsLoopsdoneW::new(self, 0)
    }
}
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`events_loopsdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_loopsdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLoopsdoneSpec;
impl crate::RegisterSpec for EventsLoopsdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_loopsdone::R`](R) reader structure"]
impl crate::Readable for EventsLoopsdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_loopsdone::W`](W) writer structure"]
impl crate::Writable for EventsLoopsdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LOOPSDONE to value 0"]
impl crate::Resettable for EventsLoopsdoneSpec {}
