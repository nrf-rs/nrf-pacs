#[doc = "Register `EVENTS_TXFRAMESTART` reader"]
pub type R = crate::R<EventsTxframestartSpec>;
#[doc = "Register `EVENTS_TXFRAMESTART` writer"]
pub type W = crate::W<EventsTxframestartSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txframestart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txframestart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxframestartSpec;
impl crate::RegisterSpec for EventsTxframestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txframestart::R`](R) reader structure"]
impl crate::Readable for EventsTxframestartSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txframestart::W`](W) writer structure"]
impl crate::Writable for EventsTxframestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXFRAMESTART to value 0"]
impl crate::Resettable for EventsTxframestartSpec {}
