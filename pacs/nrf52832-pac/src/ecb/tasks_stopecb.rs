#[doc = "Register `TASKS_STOPECB` writer"]
pub type W = crate::W<TasksStopecbSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStopecbSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Abort a possible executing ECB operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stopecb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStopecbSpec;
impl crate::RegisterSpec for TasksStopecbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stopecb::W`](W) writer structure"]
impl crate::Writable for TasksStopecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOPECB to value 0"]
impl crate::Resettable for TasksStopecbSpec {}
