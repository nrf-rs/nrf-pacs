#[doc = "Register `EVENTS_TIMEOUT` reader"]
pub type R = crate::R<EventsTimeoutSpec>;
#[doc = "Register `EVENTS_TIMEOUT` writer"]
pub type W = crate::W<EventsTimeoutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTimeoutSpec;
impl crate::RegisterSpec for EventsTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_timeout::R`](R) reader structure"]
impl crate::Readable for EventsTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`events_timeout::W`](W) writer structure"]
impl crate::Writable for EventsTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TIMEOUT to value 0"]
impl crate::Resettable for EventsTimeoutSpec {}
