#[doc = "Register `EVENTS_WRITE` reader"]
pub type R = crate::R<EventsWriteSpec>;
#[doc = "Register `EVENTS_WRITE` writer"]
pub type W = crate::W<EventsWriteSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Write command received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_write::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_write::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsWriteSpec;
impl crate::RegisterSpec for EventsWriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_write::R`](R) reader structure"]
impl crate::Readable for EventsWriteSpec {}
#[doc = "`write(|w| ..)` method takes [`events_write::W`](W) writer structure"]
impl crate::Writable for EventsWriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_WRITE to value 0"]
impl crate::Resettable for EventsWriteSpec {}
