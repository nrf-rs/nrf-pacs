#[doc = "Register `TASKS_CAPTURE[%s]` writer"]
pub type W = crate::W<TasksCaptureSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCaptureSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Capture Timer value to CC\\[n\\] registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCaptureSpec;
impl crate::RegisterSpec for TasksCaptureSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_capture::W`](W) writer structure"]
impl crate::Writable for TasksCaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CAPTURE[%s] to value 0"]
impl crate::Resettable for TasksCaptureSpec {}
