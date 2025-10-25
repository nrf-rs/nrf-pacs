#[doc = "Register `TASKS_RELEASE` writer"]
pub type W = crate::W<TasksReleaseSpec>;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRelease {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRelease> for bool {
    #[inline(always)]
    fn from(variant: TasksRelease) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RELEASE` writer - Release SPI semaphore, enabling the SPI slave to acquire it"]
pub type TasksReleaseW<'a, REG> = crate::BitWriter<'a, REG, TasksRelease>;
impl<'a, REG> TasksReleaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRelease::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    #[inline(always)]
    pub fn tasks_release(&mut self) -> TasksReleaseW<'_, TasksReleaseSpec> {
        TasksReleaseW::new(self, 0)
    }
}
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReleaseSpec;
impl crate::RegisterSpec for TasksReleaseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_release::W`](W) writer structure"]
impl crate::Writable for TasksReleaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RELEASE to value 0"]
impl crate::Resettable for TasksReleaseSpec {}
