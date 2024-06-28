#[doc = "Register `TASKS_RESUME` writer"]
pub type W = crate::W<TasksResumeSpec>;
#[doc = "Resume TWI transaction\n\nValue on reset: 0"]
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
#[doc = "Field `TASKS_RESUME` writer - Resume TWI transaction"]
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
    #[doc = "Bit 0 - Resume TWI transaction"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_resume(&mut self) -> TasksResumeW<TasksResumeSpec> {
        TasksResumeW::new(self, 0)
    }
}
#[doc = "Resume TWI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_resume::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksResumeSpec;
impl crate::RegisterSpec for TasksResumeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_resume::W`](W) writer structure"]
impl crate::Writable for TasksResumeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_RESUME to value 0"]
impl crate::Resettable for TasksResumeSpec {
    const RESET_VALUE: u32 = 0;
}
