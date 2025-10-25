#[doc = "Register `EVENTS_PORT` reader"]
pub type R = crate::R<EventsPortSpec>;
#[doc = "Register `EVENTS_PORT` writer"]
pub type W = crate::W<EventsPortSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Event generated from multiple pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPortSpec;
impl crate::RegisterSpec for EventsPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_port::R`](R) reader structure"]
impl crate::Readable for EventsPortSpec {}
#[doc = "`write(|w| ..)` method takes [`events_port::W`](W) writer structure"]
impl crate::Writable for EventsPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PORT to value 0"]
impl crate::Resettable for EventsPortSpec {}
