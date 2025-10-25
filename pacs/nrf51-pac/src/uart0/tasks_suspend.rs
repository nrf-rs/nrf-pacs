#[doc = "Register `TASKS_SUSPEND` writer"]
pub type W = crate::W<TasksSuspendSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksSuspendSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Suspend UART.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSuspendSpec;
impl crate::RegisterSpec for TasksSuspendSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_suspend::W`](W) writer structure"]
impl crate::Writable for TasksSuspendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SUSPEND to value 0"]
impl crate::Resettable for TasksSuspendSpec {}
