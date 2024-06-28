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
    #[must_use]
    pub fn tasks_preparetx(&mut self) -> TasksPreparetxW<TasksPreparetxSpec> {
        TasksPreparetxW::new(self, 0)
    }
}
#[doc = "Prepare the TWI slave to respond to a read command\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_preparetx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPreparetxSpec;
impl crate::RegisterSpec for TasksPreparetxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_preparetx::W`](W) writer structure"]
impl crate::Writable for TasksPreparetxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PREPARETX to value 0"]
impl crate::Resettable for TasksPreparetxSpec {
    const RESET_VALUE: u32 = 0;
}
