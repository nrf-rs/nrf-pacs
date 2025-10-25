#[doc = "Register `EVENTS_LASTRX` reader"]
pub type R = crate::R<EventsLastrxSpec>;
#[doc = "Register `EVENTS_LASTRX` writer"]
pub type W = crate::W<EventsLastrxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Byte boundary, starting to receive the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lastrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lastrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsLastrxSpec;
impl crate::RegisterSpec for EventsLastrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_lastrx::R`](R) reader structure"]
impl crate::Readable for EventsLastrxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_lastrx::W`](W) writer structure"]
impl crate::Writable for EventsLastrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_LASTRX to value 0"]
impl crate::Resettable for EventsLastrxSpec {}
