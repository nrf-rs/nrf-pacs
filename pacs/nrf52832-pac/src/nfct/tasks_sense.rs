#[doc = "Register `TASKS_SENSE` writer"]
pub type W = crate::W<TasksSenseSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksSenseSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enable NFC sense field mode, change state to sense mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sense::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSenseSpec;
impl crate::RegisterSpec for TasksSenseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_sense::W`](W) writer structure"]
impl crate::Writable for TasksSenseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SENSE to value 0"]
impl crate::Resettable for TasksSenseSpec {}
