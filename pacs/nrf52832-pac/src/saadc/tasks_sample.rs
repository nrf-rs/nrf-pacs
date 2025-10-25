#[doc = "Register `TASKS_SAMPLE` writer"]
pub type W = crate::W<TasksSampleSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksSampleSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sample::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSampleSpec;
impl crate::RegisterSpec for TasksSampleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_sample::W`](W) writer structure"]
impl crate::Writable for TasksSampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SAMPLE to value 0"]
impl crate::Resettable for TasksSampleSpec {}
