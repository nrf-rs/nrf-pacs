#[doc = "Register `EVENTS_RXSTARTED` reader"]
pub type R = crate::R<EventsRxstartedSpec>;
#[doc = "Register `EVENTS_RXSTARTED` writer"]
pub type W = crate::W<EventsRxstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UART receiver has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxstartedSpec;
impl crate::RegisterSpec for EventsRxstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxstarted::R`](R) reader structure"]
impl crate::Readable for EventsRxstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxstarted::W`](W) writer structure"]
impl crate::Writable for EventsRxstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXSTARTED to value 0"]
impl crate::Resettable for EventsRxstartedSpec {}
