#[doc = "Register `EVENTS_ENDECB` reader"]
pub type R = crate::R<EventsEndecbSpec>;
#[doc = "Register `EVENTS_ENDECB` writer"]
pub type W = crate::W<EventsEndecbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ECB block encrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endecb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endecb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndecbSpec;
impl crate::RegisterSpec for EventsEndecbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endecb::R`](R) reader structure"]
impl crate::Readable for EventsEndecbSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endecb::W`](W) writer structure"]
impl crate::Writable for EventsEndecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDECB to value 0"]
impl crate::Resettable for EventsEndecbSpec {}
