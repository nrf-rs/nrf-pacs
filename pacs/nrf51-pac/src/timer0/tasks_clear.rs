#[doc = "Register `TASKS_CLEAR` writer"]
pub type W = crate::W<TasksClearSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksClearSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Clear timer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClearSpec;
impl crate::RegisterSpec for TasksClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clear::W`](W) writer structure"]
impl crate::Writable for TasksClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CLEAR to value 0"]
impl crate::Resettable for TasksClearSpec {}
