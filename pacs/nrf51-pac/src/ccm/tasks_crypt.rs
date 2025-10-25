#[doc = "Register `TASKS_CRYPT` writer"]
pub type W = crate::W<TasksCryptSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksCryptSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_crypt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCryptSpec;
impl crate::RegisterSpec for TasksCryptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_crypt::W`](W) writer structure"]
impl crate::Writable for TasksCryptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CRYPT to value 0"]
impl crate::Resettable for TasksCryptSpec {}
