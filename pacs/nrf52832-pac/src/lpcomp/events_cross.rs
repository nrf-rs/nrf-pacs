#[doc = "Register `EVENTS_CROSS` reader"]
pub type R = crate::R<EventsCrossSpec>;
#[doc = "Register `EVENTS_CROSS` writer"]
pub type W = crate::W<EventsCrossSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Downward or upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cross::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cross::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrossSpec;
impl crate::RegisterSpec for EventsCrossSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_cross::R`](R) reader structure"]
impl crate::Readable for EventsCrossSpec {}
#[doc = "`write(|w| ..)` method takes [`events_cross::W`](W) writer structure"]
impl crate::Writable for EventsCrossSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CROSS to value 0"]
impl crate::Resettable for EventsCrossSpec {}
