#[doc = "Register `TASKS_PREPARERX` writer"]
pub type W = crate::W<TasksPreparerxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksPreparerxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Prepare the TWI slave to respond to a write command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_preparerx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPreparerxSpec;
impl crate::RegisterSpec for TasksPreparerxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_preparerx::W`](W) writer structure"]
impl crate::Writable for TasksPreparerxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_PREPARERX to value 0"]
impl crate::Resettable for TasksPreparerxSpec {}
