#[doc = "Register `EVENTS_HFCLKSTARTED` reader"]
pub type R = crate::R<EventsHfclkstartedSpec>;
#[doc = "Register `EVENTS_HFCLKSTARTED` writer"]
pub type W = crate::W<EventsHfclkstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HFCLK oscillator started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_hfclkstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_hfclkstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsHfclkstartedSpec;
impl crate::RegisterSpec for EventsHfclkstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_hfclkstarted::R`](R) reader structure"]
impl crate::Readable for EventsHfclkstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_hfclkstarted::W`](W) writer structure"]
impl crate::Writable for EventsHfclkstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_HFCLKSTARTED to value 0"]
impl crate::Resettable for EventsHfclkstartedSpec {}
