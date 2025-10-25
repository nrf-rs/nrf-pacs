#[doc = "Register `TASKS_PREPARETX` writer"]
pub type W = crate::W<TasksPreparetxSpec>;
#[doc = "Prepare the TWI slave to respond to a read command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPreparetx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPreparetx> for bool {
    #[inline(always)]
    fn from(variant: TasksPreparetx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PREPARETX` writer - Prepare the TWI slave to respond to a read command"]
pub type TasksPreparetxW<'a, REG> = crate::BitWriter<'a, REG, TasksPreparetx>;
impl<'a, REG> TasksPreparetxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPreparetx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Prepare the TWI slave to respond to a read command"]
    #[inline(always)]
    pub fn tasks_preparetx(&mut self) -> TasksPreparetxW<'_, TasksPreparetxSpec> {
        TasksPreparetxW::new(self, 0)
    }
}
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
