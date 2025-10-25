#[doc = "Register `TASKS_GOSLEEP` writer"]
pub type W = crate::W<TasksGosleepSpec>;
#[doc = "Force state machine to SLEEP_A state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksGosleep {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksGosleep> for bool {
    #[inline(always)]
    fn from(variant: TasksGosleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_GOSLEEP` writer - Force state machine to SLEEP_A state"]
pub type TasksGosleepW<'a, REG> = crate::BitWriter<'a, REG, TasksGosleep>;
impl<'a, REG> TasksGosleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksGosleep::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Force state machine to SLEEP_A state"]
    #[inline(always)]
    pub fn tasks_gosleep(&mut self) -> TasksGosleepW<'_, TasksGosleepSpec> {
        TasksGosleepW::new(self, 0)
    }
}
#[doc = "Force state machine to SLEEP_A state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_gosleep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksGosleepSpec;
impl crate::RegisterSpec for TasksGosleepSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_gosleep::W`](W) writer structure"]
impl crate::Writable for TasksGosleepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_GOSLEEP to value 0"]
impl crate::Resettable for TasksGosleepSpec {}
