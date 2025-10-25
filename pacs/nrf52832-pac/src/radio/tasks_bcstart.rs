#[doc = "Register `TASKS_BCSTART` writer"]
pub type W = crate::W<TasksBcstartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksBcstartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksBcstartSpec;
impl crate::RegisterSpec for TasksBcstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_bcstart::W`](W) writer structure"]
impl crate::Writable for TasksBcstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_BCSTART to value 0"]
impl crate::Resettable for TasksBcstartSpec {}
