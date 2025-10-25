#[doc = "Register `TASKS_ENABLERXDATA` writer"]
pub type W = crate::W<TasksEnablerxdataSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksEnablerxdataSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Initializes the EasyDMA for receive.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_enablerxdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEnablerxdataSpec;
impl crate::RegisterSpec for TasksEnablerxdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_enablerxdata::W`](W) writer structure"]
impl crate::Writable for TasksEnablerxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ENABLERXDATA to value 0"]
impl crate::Resettable for TasksEnablerxdataSpec {}
