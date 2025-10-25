#[doc = "Register `EVENTS_ENDTX` reader"]
pub type R = crate::R<EventsEndtxSpec>;
#[doc = "Register `EVENTS_ENDTX` writer"]
pub type W = crate::W<EventsEndtxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Last TX byte transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endtx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endtx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndtxSpec;
impl crate::RegisterSpec for EventsEndtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endtx::R`](R) reader structure"]
impl crate::Readable for EventsEndtxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endtx::W`](W) writer structure"]
impl crate::Writable for EventsEndtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDTX to value 0"]
impl crate::Resettable for EventsEndtxSpec {}
