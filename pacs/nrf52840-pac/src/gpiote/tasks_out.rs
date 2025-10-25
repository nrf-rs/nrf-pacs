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
    pub fn tasks_out(&mut self) -> TasksOutW<'_, TasksOutSpec> {
        TasksOutW::new(self, 0)
    }
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_out::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksOutSpec;
impl crate::RegisterSpec for TasksOutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_out::W`](W) writer structure"]
impl crate::Writable for TasksOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_OUT[%s] to value 0"]
impl crate::Resettable for TasksOutSpec {}
