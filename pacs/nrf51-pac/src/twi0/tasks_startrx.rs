#[doc = "Register `TASKS_STARTRX` writer"]
pub type W = crate::W<TasksStartrxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStartrxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start 2-Wire master receive sequence.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startrx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartrxSpec;
impl crate::RegisterSpec for TasksStartrxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startrx::W`](W) writer structure"]
impl crate::Writable for TasksStartrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTRX to value 0"]
impl crate::Resettable for TasksStartrxSpec {}
