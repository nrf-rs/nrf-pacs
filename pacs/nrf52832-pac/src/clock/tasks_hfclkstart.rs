#[doc = "Register `TASKS_HFCLKSTART` writer"]
pub type W = crate::W<TasksHfclkstartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksHfclkstartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start HFCLK crystal oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkstartSpec;
impl crate::RegisterSpec for TasksHfclkstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkstart::W`](W) writer structure"]
impl crate::Writable for TasksHfclkstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLKSTART to value 0"]
impl crate::Resettable for TasksHfclkstartSpec {}
