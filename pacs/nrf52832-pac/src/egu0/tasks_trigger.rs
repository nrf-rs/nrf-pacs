#[doc = "Register `TASKS_TRIGGER[%s]` writer"]
pub type W = crate::W<TasksTriggerSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksTriggerSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Trigger 0 for triggering the corresponding TRIGGERED\\[0\\] event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTriggerSpec;
impl crate::RegisterSpec for TasksTriggerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_trigger::W`](W) writer structure"]
impl crate::Writable for TasksTriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_TRIGGER[%s] to value 0"]
impl crate::Resettable for TasksTriggerSpec {}
