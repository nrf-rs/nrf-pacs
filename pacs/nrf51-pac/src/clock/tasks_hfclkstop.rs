#[doc = "Register `TASKS_HFCLKSTOP` writer"]
pub type W = crate::W<TasksHfclkstopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksHfclkstopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop HFCLK clock source.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkstopSpec;
impl crate::RegisterSpec for TasksHfclkstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkstop::W`](W) writer structure"]
impl crate::Writable for TasksHfclkstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLKSTOP to value 0"]
impl crate::Resettable for TasksHfclkstopSpec {}
