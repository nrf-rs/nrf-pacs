#[doc = "Register `TASKS_DEACTIVATE` writer"]
pub type W = crate::W<TasksDeactivateSpec>;
#[doc = "Deactivate QSPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksDeactivate {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksDeactivate> for bool {
    #[inline(always)]
    fn from(variant: TasksDeactivate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_DEACTIVATE` writer - Deactivate QSPI interface"]
pub type TasksDeactivateW<'a, REG> = crate::BitWriter<'a, REG, TasksDeactivate>;
impl<'a, REG> TasksDeactivateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksDeactivate::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate QSPI interface"]
    #[inline(always)]
    pub fn tasks_deactivate(&mut self) -> TasksDeactivateW<'_, TasksDeactivateSpec> {
        TasksDeactivateW::new(self, 0)
    }
}
#[doc = "Deactivate QSPI interface\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_deactivate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksDeactivateSpec;
impl crate::RegisterSpec for TasksDeactivateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_deactivate::W`](W) writer structure"]
impl crate::Writable for TasksDeactivateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_DEACTIVATE to value 0"]
impl crate::Resettable for TasksDeactivateSpec {}
