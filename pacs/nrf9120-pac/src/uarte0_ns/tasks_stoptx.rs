#[doc = "Register `TASKS_STOPTX` writer"]
pub type W = crate::W<TasksStoptxSpec>;
#[doc = "Stop UART transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStoptx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStoptx> for bool {
    #[inline(always)]
    fn from(variant: TasksStoptx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STOPTX` writer - Stop UART transmitter"]
pub type TasksStoptxW<'a, REG> = crate::BitWriter<'a, REG, TasksStoptx>;
impl<'a, REG> TasksStoptxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStoptx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop UART transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_stoptx(&mut self) -> TasksStoptxW<TasksStoptxSpec> {
        TasksStoptxW::new(self, 0)
    }
}
#[doc = "Stop UART transmitter\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_stoptx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStoptxSpec;
impl crate::RegisterSpec for TasksStoptxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stoptx::W`](W) writer structure"]
impl crate::Writable for TasksStoptxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_STOPTX to value 0"]
impl crate::Resettable for TasksStoptxSpec {
    const RESET_VALUE: u32 = 0;
}
