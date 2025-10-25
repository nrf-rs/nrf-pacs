#[doc = "Register `TASKS_COUNT` writer"]
pub type W = crate::W<TasksCountSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCountSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Increment Timer (Counter mode only)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_count::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCountSpec;
impl crate::RegisterSpec for TasksCountSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_count::W`](W) writer structure"]
impl crate::Writable for TasksCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_COUNT to value 0"]
impl crate::Resettable for TasksCountSpec {}
