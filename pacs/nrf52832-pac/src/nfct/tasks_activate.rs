#[doc = "Register `TASKS_ACTIVATE` writer"]
pub type W = crate::W<TasksActivateSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksActivateSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Activate NFC peripheral for incoming and outgoing frames, change state to activated\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_activate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksActivateSpec;
impl crate::RegisterSpec for TasksActivateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_activate::W`](W) writer structure"]
impl crate::Writable for TasksActivateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ACTIVATE to value 0"]
impl crate::Resettable for TasksActivateSpec {}
