#[doc = "Register `TASKS_STOP` writer"]
pub type W = crate::W<TasksStopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop the comparator.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStopSpec;
impl crate::RegisterSpec for TasksStopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stop::W`](W) writer structure"]
impl crate::Writable for TasksStopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOP to value 0"]
impl crate::Resettable for TasksStopSpec {}
