#[doc = "Register `EVENTS_IN[%s]` reader"]
pub type R = crate::R<EventsInSpec>;
#[doc = "Register `EVENTS_IN[%s]` writer"]
pub type W = crate::W<EventsInSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`events_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsInSpec;
impl crate::RegisterSpec for EventsInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_in::R`](R) reader structure"]
impl crate::Readable for EventsInSpec {}
#[doc = "`write(|w| ..)` method takes [`events_in::W`](W) writer structure"]
impl crate::Writable for EventsInSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_IN[%s] to value 0"]
impl crate::Resettable for EventsInSpec {}
