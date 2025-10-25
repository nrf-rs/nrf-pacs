#[doc = "Register `TASKS_RESUME` writer"]
pub type W = crate::W<TasksResumeSpec>;
#[doc = "Resume SPI transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksResume {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksResume> for bool {
    #[inline(always)]
    fn from(variant: TasksResume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RESUME` writer - Resume SPI transaction"]
pub type TasksResumeW<'a, REG> = crate::BitWriter<'a, REG, TasksResume>;
impl<'a, REG> TasksResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksResume::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Resume SPI transaction"]
    #[inline(always)]
    pub fn tasks_resume(&mut self) -> TasksResumeW<'_, TasksResumeSpec> {
        TasksResumeW::new(self, 0)
    }
}
#[doc = "Resume SPI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_resume::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksResumeSpec;
impl crate::RegisterSpec for TasksResumeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_resume::W`](W) writer structure"]
impl crate::Writable for TasksResumeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RESUME to value 0"]
impl crate::Resettable for TasksResumeSpec {}
