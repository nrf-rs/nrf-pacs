#[doc = "Register `EVENTS_ACQUIRED` reader"]
pub type R = crate::R<EventsAcquiredSpec>;
#[doc = "Register `EVENTS_ACQUIRED` writer"]
pub type W = crate::W<EventsAcquiredSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
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
