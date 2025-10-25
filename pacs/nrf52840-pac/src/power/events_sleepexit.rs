#[doc = "Register `EVENTS_SLEEPEXIT` reader"]
pub type R = crate::R<EventsSleepexitSpec>;
#[doc = "Register `EVENTS_SLEEPEXIT` writer"]
pub type W = crate::W<EventsSleepexitSpec>;
#[doc = "CPU exited WFI/WFE sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSleepexit {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSleepexit> for bool {
    #[inline(always)]
    fn from(variant: EventsSleepexit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SLEEPEXIT` reader - CPU exited WFI/WFE sleep"]
pub type EventsSleepexitR = crate::BitReader<EventsSleepexit>;
impl EventsSleepexitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSleepexit {
        match self.bits {
            false => EventsSleepexit::NotGenerated,
            true => EventsSleepexit::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSleepexit::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSleepexit::Generated
    }
}
#[doc = "Field `EVENTS_SLEEPEXIT` writer - CPU exited WFI/WFE sleep"]
pub type EventsSleepexitW<'a, REG> = crate::BitWriter<'a, REG, EventsSleepexit>;
impl<'a, REG> EventsSleepexitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSleepexit::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSleepexit::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepexit(&self) -> EventsSleepexitR {
        EventsSleepexitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepexit(&mut self) -> EventsSleepexitW<'_, EventsSleepexitSpec> {
        EventsSleepexitW::new(self, 0)
    }
}
#[doc = "CPU exited WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepexit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepexit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSleepexitSpec;
impl crate::RegisterSpec for EventsSleepexitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_sleepexit::R`](R) reader structure"]
impl crate::Readable for EventsSleepexitSpec {}
#[doc = "`write(|w| ..)` method takes [`events_sleepexit::W`](W) writer structure"]
impl crate::Writable for EventsSleepexitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SLEEPEXIT to value 0"]
impl crate::Resettable for EventsSleepexitSpec {}
