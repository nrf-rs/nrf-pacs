#[doc = "Register `EVENTS_RESULTDONE` reader"]
pub type R = crate::R<EventsResultdoneSpec>;
#[doc = "Register `EVENTS_RESULTDONE` writer"]
pub type W = crate::W<EventsResultdoneSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "A result is ready to get transferred to RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resultdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resultdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsResultdoneSpec;
impl crate::RegisterSpec for EventsResultdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_resultdone::R`](R) reader structure"]
impl crate::Readable for EventsResultdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_resultdone::W`](W) writer structure"]
impl crate::Writable for EventsResultdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RESULTDONE to value 0"]
impl crate::Resettable for EventsResultdoneSpec {}
