#[doc = "Register `EVENTS_AUTOCOLRESSTARTED` reader"]
pub type R = crate::R<EventsAutocolresstartedSpec>;
#[doc = "Register `EVENTS_AUTOCOLRESSTARTED` writer"]
pub type W = crate::W<EventsAutocolresstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Auto collision resolution process has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_autocolresstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_autocolresstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAutocolresstartedSpec;
impl crate::RegisterSpec for EventsAutocolresstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_autocolresstarted::R`](R) reader structure"]
impl crate::Readable for EventsAutocolresstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_autocolresstarted::W`](W) writer structure"]
impl crate::Writable for EventsAutocolresstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_AUTOCOLRESSTARTED to value 0"]
impl crate::Resettable for EventsAutocolresstartedSpec {}
