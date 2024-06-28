#[doc = "Register `TASKS_OUT[%s]` writer"]
pub type W = crate::W<TasksOutSpec>;
#[doc = "Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksOut {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksOut> for bool {
    #[inline(always)]
    fn from(variant: TasksOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_OUT` writer - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
pub type TasksOutW<'a, REG> = crate::BitWriter<'a, REG, TasksOut>;
impl<'a, REG> TasksOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksOut::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_out(&mut self) -> TasksOutW<TasksOutSpec> {
        TasksOutW::new(self, 0)
    }
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_out::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksOutSpec;
impl crate::RegisterSpec for TasksOutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_out::W`](W) writer structure"]
impl crate::Writable for TasksOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_OUT[%s]
to value 0"]
impl crate::Resettable for TasksOutSpec {
    const RESET_VALUE: u32 = 0;
}
