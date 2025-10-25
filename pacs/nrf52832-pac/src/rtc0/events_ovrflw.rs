#[doc = "Register `EVENTS_OVRFLW` reader"]
pub type R = crate::R<EventsOvrflwSpec>;
#[doc = "Register `EVENTS_OVRFLW` writer"]
pub type W = crate::W<EventsOvrflwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Event on COUNTER overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ovrflw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ovrflw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsOvrflwSpec;
impl crate::RegisterSpec for EventsOvrflwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ovrflw::R`](R) reader structure"]
impl crate::Readable for EventsOvrflwSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ovrflw::W`](W) writer structure"]
impl crate::Writable for EventsOvrflwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_OVRFLW to value 0"]
impl crate::Resettable for EventsOvrflwSpec {}
