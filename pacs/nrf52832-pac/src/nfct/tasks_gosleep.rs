#[doc = "Register `TASKS_GOSLEEP` writer"]
pub type W = crate::W<TasksGosleepSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksGosleepSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Force state machine to SLEEP_A state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_gosleep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksGosleepSpec;
impl crate::RegisterSpec for TasksGosleepSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_gosleep::W`](W) writer structure"]
impl crate::Writable for TasksGosleepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_GOSLEEP to value 0"]
impl crate::Resettable for TasksGosleepSpec {}
