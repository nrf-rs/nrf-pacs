#[doc = "Register `TASKS_STARTECB` writer"]
pub type W = crate::W<TasksStartecbSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksStartecbSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startecb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartecbSpec;
impl crate::RegisterSpec for TasksStartecbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startecb::W`](W) writer structure"]
impl crate::Writable for TasksStartecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTECB to value 0"]
impl crate::Resettable for TasksStartecbSpec {}
