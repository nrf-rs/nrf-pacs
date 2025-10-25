#[doc = "Register `TASKS_CAL` writer"]
pub type W = crate::W<TasksCalSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCalSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start calibration of LFCLK RC oscillator.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCalSpec;
impl crate::RegisterSpec for TasksCalSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_cal::W`](W) writer structure"]
impl crate::Writable for TasksCalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CAL to value 0"]
impl crate::Resettable for TasksCalSpec {}
