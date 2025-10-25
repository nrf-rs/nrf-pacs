#[doc = "Register `TASKS_STOPTX` writer"]
pub type W = crate::W<TasksStoptxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStoptxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop UART transmitter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stoptx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStoptxSpec;
impl crate::RegisterSpec for TasksStoptxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stoptx::W`](W) writer structure"]
impl crate::Writable for TasksStoptxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOPTX to value 0"]
impl crate::Resettable for TasksStoptxSpec {}
