#[doc = "Register `TASKS_RXEN` writer"]
pub type W = crate::W<TasksRxenSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksRxenSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enable RADIO in RX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rxen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRxenSpec;
impl crate::RegisterSpec for TasksRxenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rxen::W`](W) writer structure"]
impl crate::Writable for TasksRxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RXEN to value 0"]
impl crate::Resettable for TasksRxenSpec {}
