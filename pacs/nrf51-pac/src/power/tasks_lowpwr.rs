#[doc = "Register `TASKS_LOWPWR` writer"]
pub type W = crate::W<TasksLowpwrSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksLowpwrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enable low power mode (variable latency).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLowpwrSpec;
impl crate::RegisterSpec for TasksLowpwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lowpwr::W`](W) writer structure"]
impl crate::Writable for TasksLowpwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LOWPWR to value 0"]
impl crate::Resettable for TasksLowpwrSpec {}
