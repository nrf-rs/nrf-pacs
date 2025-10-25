#[doc = "Register `TASKS_STARTTX` writer"]
pub type W = crate::W<TasksStarttxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStarttxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start UART transmitter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_starttx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStarttxSpec;
impl crate::RegisterSpec for TasksStarttxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_starttx::W`](W) writer structure"]
impl crate::Writable for TasksStarttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTTX to value 0"]
impl crate::Resettable for TasksStarttxSpec {}
