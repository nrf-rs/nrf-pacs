#[doc = "Register `TASKS_LOWPWR` writer"]
pub type W = crate::W<TasksLowpwrSpec>;
#[doc = "Enable low power mode (variable latency)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksLowpwr {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksLowpwr> for bool {
    #[inline(always)]
    fn from(variant: TasksLowpwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_LOWPWR` writer - Enable low power mode (variable latency)"]
pub type TasksLowpwrW<'a, REG> = crate::BitWriter<'a, REG, TasksLowpwr>;
impl<'a, REG> TasksLowpwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksLowpwr::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enable low power mode (variable latency)"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_lowpwr(&mut self) -> TasksLowpwrW<TasksLowpwrSpec> {
        TasksLowpwrW::new(self, 0)
    }
}
#[doc = "Enable low power mode (variable latency)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_lowpwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLowpwrSpec;
impl crate::RegisterSpec for TasksLowpwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lowpwr::W`](W) writer structure"]
impl crate::Writable for TasksLowpwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_LOWPWR to value 0"]
impl crate::Resettable for TasksLowpwrSpec {
    const RESET_VALUE: u32 = 0;
}
