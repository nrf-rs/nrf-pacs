#[doc = "Register `EVENTS_RXDRDY` reader"]
pub type R = crate::R<EventsRxdrdySpec>;
#[doc = "Register `EVENTS_RXDRDY` writer"]
pub type W = crate::W<EventsRxdrdySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data received in RXD\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxdrdySpec;
impl crate::RegisterSpec for EventsRxdrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxdrdy::R`](R) reader structure"]
impl crate::Readable for EventsRxdrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxdrdy::W`](W) writer structure"]
impl crate::Writable for EventsRxdrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXDRDY to value 0"]
impl crate::Resettable for EventsRxdrdySpec {}
