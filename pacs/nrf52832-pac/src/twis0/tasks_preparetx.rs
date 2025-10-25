#[doc = "Register `TASKS_PREPARETX` writer"]
pub type W = crate::W<TasksPreparetxSpec>;
impl core::fmt::Debug for crate::generic::Reg<TasksPreparetxSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Prepare the TWI slave to respond to a read command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_preparetx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPreparetxSpec;
impl crate::RegisterSpec for TasksPreparetxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_preparetx::W`](W) writer structure"]
impl crate::Writable for TasksPreparetxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_PREPARETX to value 0"]
impl crate::Resettable for TasksPreparetxSpec {}
