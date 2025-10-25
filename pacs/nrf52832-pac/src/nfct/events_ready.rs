#[doc = "Register `EVENTS_READY` reader"]
pub type R = crate::R<EventsReadySpec>;
#[doc = "Register `EVENTS_READY` writer"]
pub type W = crate::W<EventsReadySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The NFC peripheral is ready to receive and send frames\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReadySpec;
impl crate::RegisterSpec for EventsReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ready::R`](R) reader structure"]
impl crate::Readable for EventsReadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_ready::W`](W) writer structure"]
impl crate::Writable for EventsReadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_READY to value 0"]
impl crate::Resettable for EventsReadySpec {}
