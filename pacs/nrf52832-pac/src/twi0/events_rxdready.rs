#[doc = "Register `EVENTS_RXDREADY` reader"]
pub type R = crate::R<EventsRxdreadySpec>;
#[doc = "Register `EVENTS_RXDREADY` writer"]
pub type W = crate::W<EventsRxdreadySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TWI RXD byte received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxdreadySpec;
impl crate::RegisterSpec for EventsRxdreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxdready::R`](R) reader structure"]
impl crate::Readable for EventsRxdreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxdready::W`](W) writer structure"]
impl crate::Writable for EventsRxdreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXDREADY to value 0"]
impl crate::Resettable for EventsRxdreadySpec {}
