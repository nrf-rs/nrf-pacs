#[doc = "Register `TASKS_CONSTLAT` writer"]
pub type W = crate::W<TasksConstlatSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksConstlatSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enable constant latency mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksConstlatSpec;
impl crate::RegisterSpec for TasksConstlatSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_constlat::W`](W) writer structure"]
impl crate::Writable for TasksConstlatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CONSTLAT to value 0"]
impl crate::Resettable for TasksConstlatSpec {}
