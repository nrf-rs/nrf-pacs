#[doc = "Register `TASKS_RELEASE` writer"]
pub type W = crate::W<TasksReleaseSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksReleaseSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReleaseSpec;
impl crate::RegisterSpec for TasksReleaseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_release::W`](W) writer structure"]
impl crate::Writable for TasksReleaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RELEASE to value 0"]
impl crate::Resettable for TasksReleaseSpec {}
