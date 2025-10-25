#[doc = "Register `EVENTS_NOTRESOLVED` reader"]
pub type R = crate::R<EventsNotresolvedSpec>;
#[doc = "Register `EVENTS_NOTRESOLVED` writer"]
pub type W = crate::W<EventsNotresolvedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Address not resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_notresolved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_notresolved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsNotresolvedSpec;
impl crate::RegisterSpec for EventsNotresolvedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_notresolved::R`](R) reader structure"]
impl crate::Readable for EventsNotresolvedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_notresolved::W`](W) writer structure"]
impl crate::Writable for EventsNotresolvedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_NOTRESOLVED to value 0"]
impl crate::Resettable for EventsNotresolvedSpec {}
