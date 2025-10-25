#[doc = "Register `EVENTS_SEQSTARTED[%s]` reader"]
pub type R = crate::R<EventsSeqstartedSpec>;
#[doc = "Register `EVENTS_SEQSTARTED[%s]` writer"]
pub type W = crate::W<EventsSeqstartedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: First PWM period started on sequence 0\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSeqstartedSpec;
impl crate::RegisterSpec for EventsSeqstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_seqstarted::R`](R) reader structure"]
impl crate::Readable for EventsSeqstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_seqstarted::W`](W) writer structure"]
impl crate::Writable for EventsSeqstartedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SEQSTARTED[%s] to value 0"]
impl crate::Resettable for EventsSeqstartedSpec {}
