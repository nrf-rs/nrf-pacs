#[doc = "Register `TASKS_SHUTDOWN` writer"]
pub type W = crate::W<TasksShutdownSpec>;
#[doc = "Deprecated field - Shut down timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksShutdown {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksShutdown> for bool {
    #[inline(always)]
    fn from(variant: TasksShutdown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SHUTDOWN` writer - Deprecated field - Shut down timer"]
pub type TasksShutdownW<'a, REG> = crate::BitWriter<'a, REG, TasksShutdown>;
impl<'a, REG> TasksShutdownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksShutdown::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Deprecated field - Shut down timer"]
    #[inline(always)]
    pub fn tasks_shutdown(&mut self) -> TasksShutdownW<'_, TasksShutdownSpec> {
        TasksShutdownW::new(self, 0)
    }
}
#[doc = "Deprecated register - Shut down timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_shutdown::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksShutdownSpec;
impl crate::RegisterSpec for TasksShutdownSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_shutdown::W`](W) writer structure"]
impl crate::Writable for TasksShutdownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SHUTDOWN to value 0"]
impl crate::Resettable for TasksShutdownSpec {}
