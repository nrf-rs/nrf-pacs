#[doc = "Register `TASKS_CALIBRATEOFFSET` writer"]
pub type W = crate::W<TasksCalibrateoffsetSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCalibrateoffsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Starts offset auto-calibration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_calibrateoffset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCalibrateoffsetSpec;
impl crate::RegisterSpec for TasksCalibrateoffsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_calibrateoffset::W`](W) writer structure"]
impl crate::Writable for TasksCalibrateoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CALIBRATEOFFSET to value 0"]
impl crate::Resettable for TasksCalibrateoffsetSpec {}
