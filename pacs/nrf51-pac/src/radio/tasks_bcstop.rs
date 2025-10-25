#[doc = "Register `TASKS_BCSTOP` writer"]
pub type W = crate::W<TasksBcstopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksBcstopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop the bit counter.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksBcstopSpec;
impl crate::RegisterSpec for TasksBcstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_bcstop::W`](W) writer structure"]
impl crate::Writable for TasksBcstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_BCSTOP to value 0"]
impl crate::Resettable for TasksBcstopSpec {}
