#[doc = "Register `EVENTS_STOPPED` reader"]
pub type R = crate::R<EventsStoppedSpec>;
#[doc = "Register `EVENTS_STOPPED` writer"]
pub type W = crate::W<EventsStoppedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Two-wire stopped.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsStoppedSpec;
impl crate::RegisterSpec for EventsStoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_stopped::R`](R) reader structure"]
impl crate::Readable for EventsStoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_stopped::W`](W) writer structure"]
impl crate::Writable for EventsStoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_STOPPED to value 0"]
impl crate::Resettable for EventsStoppedSpec {}
