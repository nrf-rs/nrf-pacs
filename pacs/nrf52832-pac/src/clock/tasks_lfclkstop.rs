#[doc = "Register `TASKS_LFCLKSTOP` writer"]
pub type W = crate::W<TasksLfclkstopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksLfclkstopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop LFCLK source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLfclkstopSpec;
impl crate::RegisterSpec for TasksLfclkstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lfclkstop::W`](W) writer structure"]
impl crate::Writable for TasksLfclkstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LFCLKSTOP to value 0"]
impl crate::Resettable for TasksLfclkstopSpec {}
