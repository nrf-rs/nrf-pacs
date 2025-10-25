#[doc = "Register `TASKS_CTSTOP` writer"]
pub type W = crate::W<TasksCtstopSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCtstopSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Stop calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCtstopSpec;
impl crate::RegisterSpec for TasksCtstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ctstop::W`](W) writer structure"]
impl crate::Writable for TasksCtstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CTSTOP to value 0"]
impl crate::Resettable for TasksCtstopSpec {}
