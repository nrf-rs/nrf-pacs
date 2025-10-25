#[doc = "Register `EVENTS_TXSTOPPED` reader"]
pub type R = crate::R<EventsTxstoppedSpec>;
#[doc = "Register `EVENTS_TXSTOPPED` writer"]
pub type W = crate::W<EventsTxstoppedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmitter stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txstopped::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txstopped::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxstoppedSpec;
impl crate::RegisterSpec for EventsTxstoppedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txstopped::R`](R) reader structure"]
impl crate::Readable for EventsTxstoppedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txstopped::W`](W) writer structure"]
impl crate::Writable for EventsTxstoppedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXSTOPPED to value 0"]
impl crate::Resettable for EventsTxstoppedSpec {}
