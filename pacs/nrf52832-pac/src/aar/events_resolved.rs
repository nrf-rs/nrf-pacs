#[doc = "Register `EVENTS_RESOLVED` reader"]
pub type R = crate::R<EventsResolvedSpec>;
#[doc = "Register `EVENTS_RESOLVED` writer"]
pub type W = crate::W<EventsResolvedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Address resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resolved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resolved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsResolvedSpec;
impl crate::RegisterSpec for EventsResolvedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_resolved::R`](R) reader structure"]
impl crate::Readable for EventsResolvedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_resolved::W`](W) writer structure"]
impl crate::Writable for EventsResolvedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RESOLVED to value 0"]
impl crate::Resettable for EventsResolvedSpec {}
