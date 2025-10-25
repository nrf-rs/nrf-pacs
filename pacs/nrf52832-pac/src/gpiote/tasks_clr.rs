#[doc = "Register `TASKS_CLR[%s]` writer"]
pub type W = crate::W<TasksClrSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClrSpec;
impl crate::RegisterSpec for TasksClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clr::W`](W) writer structure"]
impl crate::Writable for TasksClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CLR[%s] to value 0"]
impl crate::Resettable for TasksClrSpec {}
