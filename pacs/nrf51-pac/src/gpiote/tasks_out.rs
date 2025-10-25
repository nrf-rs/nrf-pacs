#[doc = "Register `TASKS_OUT[%s]` writer"]
pub type W = crate::W<TasksOutSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksOutSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Tasks asssociated with GPIOTE channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_out::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksOutSpec;
impl crate::RegisterSpec for TasksOutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_out::W`](W) writer structure"]
impl crate::Writable for TasksOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_OUT[%s] to value 0"]
impl crate::Resettable for TasksOutSpec {}
