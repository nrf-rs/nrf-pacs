#[doc = "Register `EVENTS_LASTTX` reader"]
pub type R = crate::R<EventsLasttxSpec>;
#[doc = "Register `EVENTS_LASTTX` writer"]
pub type W = crate::W<EventsLasttxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Byte boundary, starting to transmit the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lasttx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lasttx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLasttxSpec;
impl crate::RegisterSpec for EventsLasttxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lasttx::R`](R) reader structure"]
impl crate::Readable for EventsLasttxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lasttx::W`](W) writer structure"]
impl crate::Writable for EventsLasttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LASTTX to value 0"]
impl crate::Resettable for EventsLasttxSpec {}
