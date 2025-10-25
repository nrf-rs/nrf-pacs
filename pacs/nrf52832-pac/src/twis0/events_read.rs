#[doc = "Register `EVENTS_READ` reader"]
pub type R = crate::R<EventsReadSpec>;
#[doc = "Register `EVENTS_READ` writer"]
pub type W = crate::W<EventsReadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read command received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_read::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_read::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReadSpec;
impl crate::RegisterSpec for EventsReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_read::R`](R) reader structure"]
impl crate::Readable for EventsReadSpec {}
#[doc = "`write(|w| ..)` method takes [`events_read::W`](W) writer structure"]
impl crate::Writable for EventsReadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_READ to value 0"]
impl crate::Resettable for EventsReadSpec {}
