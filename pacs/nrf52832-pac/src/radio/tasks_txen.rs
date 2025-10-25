#[doc = "Register `TASKS_TXEN` writer"]
pub type W = crate::W<TasksTxenSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksTxenSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enable RADIO in TX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_txen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTxenSpec;
impl crate::RegisterSpec for TasksTxenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_txen::W`](W) writer structure"]
impl crate::Writable for TasksTxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_TXEN to value 0"]
impl crate::Resettable for TasksTxenSpec {}
