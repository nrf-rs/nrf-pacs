#[doc = "Register `TASKS_STARTRX` writer"]
pub type W = crate::W<TasksStartrxSpec>;
#[doc = "Start TWI receive sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartrx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartrx> for bool {
    #[inline(always)]
    fn from(variant: TasksStartrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTRX` writer - Start TWI receive sequence"]
pub type TasksStartrxW<'a, REG> = crate::BitWriter<'a, REG, TasksStartrx>;
impl<'a, REG> TasksStartrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartrx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start TWI receive sequence"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_startrx(&mut self) -> TasksStartrxW<TasksStartrxSpec> {
        TasksStartrxW::new(self, 0)
    }
}
#[doc = "Start TWI receive sequence\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_startrx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartrxSpec;
impl crate::RegisterSpec for TasksStartrxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startrx::W`](W) writer structure"]
impl crate::Writable for TasksStartrxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_STARTRX to value 0"]
impl crate::Resettable for TasksStartrxSpec {
    const RESET_VALUE: u32 = 0;
}
