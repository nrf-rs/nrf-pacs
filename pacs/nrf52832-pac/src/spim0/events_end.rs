#[doc = "Register `EVENTS_END` reader"]
pub type R = crate::R<EventsEndSpec>;
#[doc = "Register `EVENTS_END` writer"]
pub type W = crate::W<EventsEndSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "End of RXD buffer and TXD buffer reached\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndSpec;
impl crate::RegisterSpec for EventsEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_end::R`](R) reader structure"]
impl crate::Readable for EventsEndSpec {}
#[doc = "`write(|w| ..)` method takes [`events_end::W`](W) writer structure"]
impl crate::Writable for EventsEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_END to value 0"]
impl crate::Resettable for EventsEndSpec {}
