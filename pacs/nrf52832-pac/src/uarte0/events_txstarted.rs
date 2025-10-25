#[doc = "Register `EVENTS_TXSTARTED` reader"]
pub type R = crate::R<EventsTxstartedSpec>;
#[doc = "Register `EVENTS_TXSTARTED` writer"]
pub type W = crate::W<EventsTxstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UART transmitter has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxstartedSpec;
impl crate::RegisterSpec for EventsTxstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txstarted::R`](R) reader structure"]
impl crate::Readable for EventsTxstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txstarted::W`](W) writer structure"]
impl crate::Writable for EventsTxstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXSTARTED to value 0"]
impl crate::Resettable for EventsTxstartedSpec {}
