#[doc = "Register `TASKS_TRIGOVRFLW` writer"]
pub type W = crate::W<TasksTrigovrflwSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksTrigovrflwSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Set COUNTER to 0xFFFFFFF0.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigovrflw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTrigovrflwSpec;
impl crate::RegisterSpec for TasksTrigovrflwSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_trigovrflw::W`](W) writer structure"]
impl crate::Writable for TasksTrigovrflwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_TRIGOVRFLW to value 0"]
impl crate::Resettable for TasksTrigovrflwSpec {}
