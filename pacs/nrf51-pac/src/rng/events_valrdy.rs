#[doc = "Register `EVENTS_VALRDY` reader"]
pub type R = crate::R<EventsValrdySpec>;
#[doc = "Register `EVENTS_VALRDY` writer"]
pub type W = crate::W<EventsValrdySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "New random number generated and written to VALUE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_valrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_valrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsValrdySpec;
impl crate::RegisterSpec for EventsValrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_valrdy::R`](R) reader structure"]
impl crate::Readable for EventsValrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_valrdy::W`](W) writer structure"]
impl crate::Writable for EventsValrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_VALRDY to value 0"]
impl crate::Resettable for EventsValrdySpec {}
