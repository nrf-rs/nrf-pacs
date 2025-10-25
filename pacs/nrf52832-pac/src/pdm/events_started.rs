#[doc = "Register `EVENTS_STARTED` reader"]
pub type R = crate::R<EventsStartedSpec>;
#[doc = "Register `EVENTS_STARTED` writer"]
pub type W = crate::W<EventsStartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PDM transfer has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsStartedSpec;
impl crate::RegisterSpec for EventsStartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_started::R`](R) reader structure"]
impl crate::Readable for EventsStartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_started::W`](W) writer structure"]
impl crate::Writable for EventsStartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_STARTED to value 0"]
impl crate::Resettable for EventsStartedSpec {}
