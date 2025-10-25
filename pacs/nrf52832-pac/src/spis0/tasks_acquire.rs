#[doc = "Register `TASKS_ACQUIRE` writer"]
pub type W = crate::W<TasksAcquireSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksAcquireSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Acquire SPI semaphore\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_acquire::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksAcquireSpec;
impl crate::RegisterSpec for TasksAcquireSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_acquire::W`](W) writer structure"]
impl crate::Writable for TasksAcquireSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ACQUIRE to value 0"]
impl crate::Resettable for TasksAcquireSpec {}
