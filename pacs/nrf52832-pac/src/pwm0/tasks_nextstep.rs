#[doc = "Register `TASKS_NEXTSTEP` writer"]
pub type W = crate::W<TasksNextstepSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksNextstepSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start it was not running.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_nextstep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksNextstepSpec;
impl crate::RegisterSpec for TasksNextstepSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_nextstep::W`](W) writer structure"]
impl crate::Writable for TasksNextstepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_NEXTSTEP to value 0"]
impl crate::Resettable for TasksNextstepSpec {}
