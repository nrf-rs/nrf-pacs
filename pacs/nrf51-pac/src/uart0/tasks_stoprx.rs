#[doc = "Register `TASKS_STOPRX` writer"]
pub type W = crate::W<TasksStoprxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStoprxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop UART receiver.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stoprx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStoprxSpec;
impl crate::RegisterSpec for TasksStoprxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stoprx::W`](W) writer structure"]
impl crate::Writable for TasksStoprxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOPRX to value 0"]
impl crate::Resettable for TasksStoprxSpec {}
