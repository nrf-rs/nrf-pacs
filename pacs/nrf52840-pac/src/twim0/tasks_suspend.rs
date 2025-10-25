#[doc = "Register `TASKS_SUSPEND` writer"]
pub type W = crate::W<TasksSuspendSpec>;
#[doc = "Suspend TWI transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSuspend {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSuspend> for bool {
    #[inline(always)]
    fn from(variant: TasksSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SUSPEND` writer - Suspend TWI transaction"]
pub type TasksSuspendW<'a, REG> = crate::BitWriter<'a, REG, TasksSuspend>;
impl<'a, REG> TasksSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSuspend::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend TWI transaction"]
    #[inline(always)]
    pub fn tasks_suspend(&mut self) -> TasksSuspendW<'_, TasksSuspendSpec> {
        TasksSuspendW::new(self, 0)
    }
}
#[doc = "Suspend TWI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSuspendSpec;
impl crate::RegisterSpec for TasksSuspendSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_suspend::W`](W) writer structure"]
impl crate::Writable for TasksSuspendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SUSPEND to value 0"]
impl crate::Resettable for TasksSuspendSpec {}
