#[doc = "Register `TASKS_SEQSTART[%s]` writer"]
pub type W = crate::W<TasksSeqstartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksSeqstartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Loads the first PWM value on all enabled channels from sequence 0, and starts playing that sequence at the rate defined in SEQ\\[0\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_seqstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSeqstartSpec;
impl crate::RegisterSpec for TasksSeqstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_seqstart::W`](W) writer structure"]
impl crate::Writable for TasksSeqstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SEQSTART[%s] to value 0"]
impl crate::Resettable for TasksSeqstartSpec {}
