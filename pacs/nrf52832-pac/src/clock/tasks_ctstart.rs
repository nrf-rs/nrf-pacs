#[doc = "Register `TASKS_CTSTART` writer"]
pub type W = crate::W<TasksCtstartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCtstartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCtstartSpec;
impl crate::RegisterSpec for TasksCtstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ctstart::W`](W) writer structure"]
impl crate::Writable for TasksCtstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CTSTART to value 0"]
impl crate::Resettable for TasksCtstartSpec {}
