#[doc = "Register `TASKS_FLUSHRX` writer"]
pub type W = crate::W<TasksFlushrxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksFlushrxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Flush RX FIFO into RX buffer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_flushrx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksFlushrxSpec;
impl crate::RegisterSpec for TasksFlushrxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_flushrx::W`](W) writer structure"]
impl crate::Writable for TasksFlushrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_FLUSHRX to value 0"]
impl crate::Resettable for TasksFlushrxSpec {}
