#[doc = "Register `EVENTS_RXFRAMEEND` reader"]
pub type R = crate::R<EventsRxframeendSpec>;
#[doc = "Register `EVENTS_RXFRAMEEND` writer"]
pub type W = crate::W<EventsRxframeendSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxframeend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxframeend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxframeendSpec;
impl crate::RegisterSpec for EventsRxframeendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxframeend::R`](R) reader structure"]
impl crate::Readable for EventsRxframeendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxframeend::W`](W) writer structure"]
impl crate::Writable for EventsRxframeendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXFRAMEEND to value 0"]
impl crate::Resettable for EventsRxframeendSpec {}
