#[doc = "Register `EVENTS_BCMATCH` reader"]
pub type R = crate::R<EventsBcmatchSpec>;
#[doc = "Register `EVENTS_BCMATCH` writer"]
pub type W = crate::W<EventsBcmatchSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bit counter reached bit count value specified in BCC register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bcmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bcmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsBcmatchSpec;
impl crate::RegisterSpec for EventsBcmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_bcmatch::R`](R) reader structure"]
impl crate::Readable for EventsBcmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_bcmatch::W`](W) writer structure"]
impl crate::Writable for EventsBcmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_BCMATCH to value 0"]
impl crate::Resettable for EventsBcmatchSpec {}
