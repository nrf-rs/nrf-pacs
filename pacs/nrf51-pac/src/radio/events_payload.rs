#[doc = "Register `EVENTS_PAYLOAD` reader"]
pub type R = crate::R<EventsPayloadSpec>;
#[doc = "Register `EVENTS_PAYLOAD` writer"]
pub type W = crate::W<EventsPayloadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Payload event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_payload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_payload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPayloadSpec;
impl crate::RegisterSpec for EventsPayloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_payload::R`](R) reader structure"]
impl crate::Readable for EventsPayloadSpec {}
#[doc = "`write(|w| ..)` method takes [`events_payload::W`](W) writer structure"]
impl crate::Writable for EventsPayloadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PAYLOAD to value 0"]
impl crate::Resettable for EventsPayloadSpec {}
