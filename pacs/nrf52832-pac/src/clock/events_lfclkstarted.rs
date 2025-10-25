#[doc = "Register `EVENTS_LFCLKSTARTED` reader"]
pub type R = crate::R<EventsLfclkstartedSpec>;
#[doc = "Register `EVENTS_LFCLKSTARTED` writer"]
pub type W = crate::W<EventsLfclkstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LFCLK started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLfclkstartedSpec;
impl crate::RegisterSpec for EventsLfclkstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lfclkstarted::R`](R) reader structure"]
impl crate::Readable for EventsLfclkstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lfclkstarted::W`](W) writer structure"]
impl crate::Writable for EventsLfclkstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LFCLKSTARTED to value 0"]
impl crate::Resettable for EventsLfclkstartedSpec {}
