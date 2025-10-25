#[doc = "Register `TASKS_RDCLRDBL` writer"]
pub type W = crate::W<TasksRdclrdblSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksRdclrdblSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Read and clear ACCDBL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclrdbl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRdclrdblSpec;
impl crate::RegisterSpec for TasksRdclrdblSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rdclrdbl::W`](W) writer structure"]
impl crate::Writable for TasksRdclrdblSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RDCLRDBL to value 0"]
impl crate::Resettable for TasksRdclrdblSpec {}
