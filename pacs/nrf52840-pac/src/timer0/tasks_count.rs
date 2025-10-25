#[doc = "Register `TASKS_COUNT` writer"]
pub type W = crate::W<TasksCountSpec>;
#[doc = "Increment Timer (Counter mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCount {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCount> for bool {
    #[inline(always)]
    fn from(variant: TasksCount) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_COUNT` writer - Increment Timer (Counter mode only)"]
pub type TasksCountW<'a, REG> = crate::BitWriter<'a, REG, TasksCount>;
impl<'a, REG> TasksCountW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCount::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Increment Timer (Counter mode only)"]
    #[inline(always)]
    pub fn tasks_count(&mut self) -> TasksCountW<'_, TasksCountSpec> {
        TasksCountW::new(self, 0)
    }
}
#[doc = "Increment Timer (Counter mode only)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_count::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCountSpec;
impl crate::RegisterSpec for TasksCountSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_count::W`](W) writer structure"]
impl crate::Writable for TasksCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_COUNT to value 0"]
impl crate::Resettable for TasksCountSpec {}
