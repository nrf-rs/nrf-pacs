#[doc = "Register `TASKS_ACQUIRE` writer"]
pub type W = crate::W<TasksAcquireSpec>;
#[doc = "Acquire SPI semaphore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksAcquire {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksAcquire> for bool {
    #[inline(always)]
    fn from(variant: TasksAcquire) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ACQUIRE` writer - Acquire SPI semaphore"]
pub type TasksAcquireW<'a, REG> = crate::BitWriter<'a, REG, TasksAcquire>;
impl<'a, REG> TasksAcquireW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksAcquire::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Acquire SPI semaphore"]
    #[inline(always)]
    pub fn tasks_acquire(&mut self) -> TasksAcquireW<'_, TasksAcquireSpec> {
        TasksAcquireW::new(self, 0)
    }
}
#[doc = "Acquire SPI semaphore\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_acquire::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksAcquireSpec;
impl crate::RegisterSpec for TasksAcquireSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_acquire::W`](W) writer structure"]
impl crate::Writable for TasksAcquireSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ACQUIRE to value 0"]
impl crate::Resettable for TasksAcquireSpec {}
