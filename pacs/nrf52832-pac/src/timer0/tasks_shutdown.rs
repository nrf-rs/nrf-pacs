#[doc = "Register `TASKS_SHUTDOWN` writer"]
pub type W = crate::W<TasksShutdownSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksShutdownSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Deprecated register - Shut down timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_shutdown::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksShutdownSpec;
impl crate::RegisterSpec for TasksShutdownSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_shutdown::W`](W) writer structure"]
impl crate::Writable for TasksShutdownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SHUTDOWN to value 0"]
impl crate::Resettable for TasksShutdownSpec {}
