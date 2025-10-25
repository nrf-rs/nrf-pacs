#[doc = "Register `EVENTS_SEQEND[%s]` reader"]
pub type R = crate::R<EventsSeqendSpec>;
#[doc = "Register `EVENTS_SEQEND[%s]` writer"]
pub type W = crate::W<EventsSeqendSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Emitted at end of every sequence 0, when last value from RAM has been applied to wave counter\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSeqendSpec;
impl crate::RegisterSpec for EventsSeqendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_seqend::R`](R) reader structure"]
impl crate::Readable for EventsSeqendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_seqend::W`](W) writer structure"]
impl crate::Writable for EventsSeqendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SEQEND[%s] to value 0"]
impl crate::Resettable for EventsSeqendSpec {}
