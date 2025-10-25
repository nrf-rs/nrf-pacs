#[doc = "Register `EVENTS_DONE` reader"]
pub type R = crate::R<EventsDoneSpec>;
#[doc = "Register `EVENTS_DONE` writer"]
pub type W = crate::W<EventsDoneSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDoneSpec;
impl crate::RegisterSpec for EventsDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_done::R`](R) reader structure"]
impl crate::Readable for EventsDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_done::W`](W) writer structure"]
impl crate::Writable for EventsDoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DONE to value 0"]
impl crate::Resettable for EventsDoneSpec {}
