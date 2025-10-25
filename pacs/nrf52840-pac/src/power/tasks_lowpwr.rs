#[doc = "Register `TASKS_LOWPWR` writer"]
pub type W = crate::W<TasksLowpwrSpec>;
#[doc = "Enable Low-power mode (variable latency)\n\nValue on reset: 0"]
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
#[doc = "Field `TASKS_LOWPWR` writer - Enable Low-power mode (variable latency)"]
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
    #[doc = "Bit 0 - Enable Low-power mode (variable latency)"]
    #[inline(always)]
    pub fn tasks_lowpwr(&mut self) -> TasksLowpwrW<'_, TasksLowpwrSpec> {
        TasksLowpwrW::new(self, 0)
    }
}
#[doc = "Enable Low-power mode (variable latency)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLowpwrSpec;
impl crate::RegisterSpec for TasksLowpwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lowpwr::W`](W) writer structure"]
impl crate::Writable for TasksLowpwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LOWPWR to value 0"]
impl crate::Resettable for TasksLowpwrSpec {}
