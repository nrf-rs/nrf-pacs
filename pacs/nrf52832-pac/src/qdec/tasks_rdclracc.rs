#[doc = "Register `TASKS_RDCLRACC` writer"]
pub type W = crate::W<TasksRdclraccSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksRdclraccSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Read and clear ACC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclracc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRdclraccSpec;
impl crate::RegisterSpec for TasksRdclraccSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rdclracc::W`](W) writer structure"]
impl crate::Writable for TasksRdclraccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RDCLRACC to value 0"]
impl crate::Resettable for TasksRdclraccSpec {}
