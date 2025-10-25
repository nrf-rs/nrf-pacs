#[doc = "Register `TASKS_KSGEN` writer"]
pub type W = crate::W<TasksKsgenSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksKsgenSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ksgen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksKsgenSpec;
impl crate::RegisterSpec for TasksKsgenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ksgen::W`](W) writer structure"]
impl crate::Writable for TasksKsgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_KSGEN to value 0"]
impl crate::Resettable for TasksKsgenSpec {}
