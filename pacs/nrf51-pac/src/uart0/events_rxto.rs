#[doc = "Register `EVENTS_RXTO` reader"]
pub type R = crate::R<EventsRxtoSpec>;
#[doc = "Register `EVENTS_RXTO` writer"]
pub type W = crate::W<EventsRxtoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receiver timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxtoSpec;
impl crate::RegisterSpec for EventsRxtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxto::R`](R) reader structure"]
impl crate::Readable for EventsRxtoSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxto::W`](W) writer structure"]
impl crate::Writable for EventsRxtoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXTO to value 0"]
impl crate::Resettable for EventsRxtoSpec {}
