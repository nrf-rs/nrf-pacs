#[doc = "Register `TASKS_RSSISTART` writer"]
pub type W = crate::W<TasksRssistartSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksRssistartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start the RSSI and take one sample of the receive signal strength.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRssistartSpec;
impl crate::RegisterSpec for TasksRssistartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rssistart::W`](W) writer structure"]
impl crate::Writable for TasksRssistartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RSSISTART to value 0"]
impl crate::Resettable for TasksRssistartSpec {}
