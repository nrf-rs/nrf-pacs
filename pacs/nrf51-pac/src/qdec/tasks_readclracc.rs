#[doc = "Register `TASKS_READCLRACC` writer"]
pub type W = crate::W<TasksReadclraccSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksReadclraccSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readclracc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReadclraccSpec;
impl crate::RegisterSpec for TasksReadclraccSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_readclracc::W`](W) writer structure"]
impl crate::Writable for TasksReadclraccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_READCLRACC to value 0"]
impl crate::Resettable for TasksReadclraccSpec {}
