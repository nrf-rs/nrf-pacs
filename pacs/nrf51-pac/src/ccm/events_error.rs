#[doc = "Register `EVENTS_ERROR` reader"]
pub type R = crate::R<EventsErrorSpec>;
#[doc = "Register `EVENTS_ERROR` writer"]
pub type W = crate::W<EventsErrorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Error happened.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsErrorSpec;
impl crate::RegisterSpec for EventsErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_error::R`](R) reader structure"]
impl crate::Readable for EventsErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_error::W`](W) writer structure"]
impl crate::Writable for EventsErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ERROR to value 0"]
impl crate::Resettable for EventsErrorSpec {}
