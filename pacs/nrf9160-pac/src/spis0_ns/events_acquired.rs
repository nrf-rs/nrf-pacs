#[doc = "Register `EVENTS_ACQUIRED` reader"]
pub type R = crate::R<EventsAcquiredSpec>;
#[doc = "Register `EVENTS_ACQUIRED` writer"]
pub type W = crate::W<EventsAcquiredSpec>;
#[doc = "Semaphore acquired\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsAcquired {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsAcquired> for bool {
    #[inline(always)]
    fn from(variant: EventsAcquired) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ACQUIRED` reader - Semaphore acquired"]
pub type EventsAcquiredR = crate::BitReader<EventsAcquired>;
impl EventsAcquiredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsAcquired {
        match self.bits {
            false => EventsAcquired::NotGenerated,
            true => EventsAcquired::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsAcquired::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsAcquired::Generated
    }
}
#[doc = "Field `EVENTS_ACQUIRED` writer - Semaphore acquired"]
pub type EventsAcquiredW<'a, REG> = crate::BitWriter<'a, REG, EventsAcquired>;
impl<'a, REG> EventsAcquiredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAcquired::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsAcquired::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Semaphore acquired"]
    #[inline(always)]
    pub fn events_acquired(&self) -> EventsAcquiredR {
        EventsAcquiredR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Semaphore acquired"]
    #[inline(always)]
    pub fn events_acquired(&mut self) -> EventsAcquiredW<'_, EventsAcquiredSpec> {
        EventsAcquiredW::new(self, 0)
    }
}
#[doc = "Semaphore acquired\n\nYou can [`read`](crate::Reg::read) this register and get [`events_acquired::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_acquired::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAcquiredSpec;
impl crate::RegisterSpec for EventsAcquiredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_acquired::R`](R) reader structure"]
impl crate::Readable for EventsAcquiredSpec {}
#[doc = "`write(|w| ..)` method takes [`events_acquired::W`](W) writer structure"]
impl crate::Writable for EventsAcquiredSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ACQUIRED to value 0"]
impl crate::Resettable for EventsAcquiredSpec {}
