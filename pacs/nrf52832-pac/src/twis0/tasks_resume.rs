#[doc = "Register `TASKS_RESUME` writer"]
pub type W = crate::W<TasksResumeSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksResumeSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Resume TWI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_resume::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksResumeSpec;
impl crate::RegisterSpec for TasksResumeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_resume::W`](W) writer structure"]
impl crate::Writable for TasksResumeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RESUME to value 0"]
impl crate::Resettable for TasksResumeSpec {}
