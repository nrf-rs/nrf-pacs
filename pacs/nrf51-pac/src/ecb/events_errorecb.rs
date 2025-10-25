#[doc = "Register `EVENTS_ERRORECB` reader"]
pub type R = crate::R<EventsErrorecbSpec>;
#[doc = "Register `EVENTS_ERRORECB` writer"]
pub type W = crate::W<EventsErrorecbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_errorecb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_errorecb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsErrorecbSpec;
impl crate::RegisterSpec for EventsErrorecbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_errorecb::R`](R) reader structure"]
impl crate::Readable for EventsErrorecbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_errorecb::W`](W) writer structure"]
impl crate::Writable for EventsErrorecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ERRORECB to value 0"]
impl crate::Resettable for EventsErrorecbSpec {}
