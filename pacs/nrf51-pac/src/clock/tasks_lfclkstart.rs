#[doc = "Register `TASKS_LFCLKSTART` writer"]
pub type W = crate::W<TasksLfclkstartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksLfclkstartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start LFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLfclkstartSpec;
impl crate::RegisterSpec for TasksLfclkstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lfclkstart::W`](W) writer structure"]
impl crate::Writable for TasksLfclkstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LFCLKSTART to value 0"]
impl crate::Resettable for TasksLfclkstartSpec {}
