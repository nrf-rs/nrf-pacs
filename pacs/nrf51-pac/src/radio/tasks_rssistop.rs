#[doc = "Register `TASKS_RSSISTOP` writer"]
pub type W = crate::W<TasksRssistopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksRssistopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop the RSSI measurement.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRssistopSpec;
impl crate::RegisterSpec for TasksRssistopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rssistop::W`](W) writer structure"]
impl crate::Writable for TasksRssistopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RSSISTOP to value 0"]
impl crate::Resettable for TasksRssistopSpec {}
