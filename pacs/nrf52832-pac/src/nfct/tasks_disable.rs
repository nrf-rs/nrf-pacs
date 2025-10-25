#[doc = "Register `TASKS_DISABLE` writer"]
pub type W = crate::W<TasksDisableSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksDisableSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Disable NFC peripheral\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksDisableSpec;
impl crate::RegisterSpec for TasksDisableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_disable::W`](W) writer structure"]
impl crate::Writable for TasksDisableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_DISABLE to value 0"]
impl crate::Resettable for TasksDisableSpec {}
