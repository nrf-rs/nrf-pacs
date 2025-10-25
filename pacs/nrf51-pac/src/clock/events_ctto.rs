#[doc = "Register `EVENTS_CTTO` reader"]
pub type R = crate::R<EventsCttoSpec>;
#[doc = "Register `EVENTS_CTTO` writer"]
pub type W = crate::W<EventsCttoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Calibration timer timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCttoSpec;
impl crate::RegisterSpec for EventsCttoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ctto::R`](R) reader structure"]
impl crate::Readable for EventsCttoSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ctto::W`](W) writer structure"]
impl crate::Writable for EventsCttoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTTO to value 0"]
impl crate::Resettable for EventsCttoSpec {}
