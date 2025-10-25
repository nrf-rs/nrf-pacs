#[doc = "Register `TASKS_GOIDLE` writer"]
pub type W = crate::W<TasksGoidleSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksGoidleSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Force state machine to IDLE state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_goidle::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksGoidleSpec;
impl crate::RegisterSpec for TasksGoidleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_goidle::W`](W) writer structure"]
impl crate::Writable for TasksGoidleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_GOIDLE to value 0"]
impl crate::Resettable for TasksGoidleSpec {}
