#[doc = "Register `TASKS_PREPARERX` writer"]
pub type W = crate::W<TasksPreparerxSpec>;
#[doc = "Prepare the TWI slave to respond to a write command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPreparerx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPreparerx> for bool {
    #[inline(always)]
    fn from(variant: TasksPreparerx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PREPARERX` writer - Prepare the TWI slave to respond to a write command"]
pub type TasksPreparerxW<'a, REG> = crate::BitWriter<'a, REG, TasksPreparerx>;
impl<'a, REG> TasksPreparerxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPreparerx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Prepare the TWI slave to respond to a write command"]
    #[inline(always)]
    pub fn tasks_preparerx(&mut self) -> TasksPreparerxW<'_, TasksPreparerxSpec> {
        TasksPreparerxW::new(self, 0)
    }
}
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
