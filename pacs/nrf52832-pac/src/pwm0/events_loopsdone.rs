#[doc = "Register `EVENTS_LOOPSDONE` reader"]
pub type R = crate::R<EventsLoopsdoneSpec>;
#[doc = "Register `EVENTS_LOOPSDONE` writer"]
pub type W = crate::W<EventsLoopsdoneSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`events_loopsdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_loopsdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLoopsdoneSpec;
impl crate::RegisterSpec for EventsLoopsdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_loopsdone::R`](R) reader structure"]
impl crate::Readable for EventsLoopsdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_loopsdone::W`](W) writer structure"]
impl crate::Writable for EventsLoopsdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LOOPSDONE to value 0"]
impl crate::Resettable for EventsLoopsdoneSpec {}
