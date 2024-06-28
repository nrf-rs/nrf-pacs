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
    #[must_use]
    pub fn tasks_count(&mut self) -> TasksCountW<TasksCountSpec> {
        TasksCountW::new(self, 0)
    }
}
#[doc = "Increment Timer (Counter mode only)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_count::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCountSpec;
impl crate::RegisterSpec for TasksCountSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_count::W`](W) writer structure"]
impl crate::Writable for TasksCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_COUNT to value 0"]
impl crate::Resettable for TasksCountSpec {
    const RESET_VALUE: u32 = 0;
}
