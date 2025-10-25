#[doc = "Register `TASKS_NEXTSTEP` writer"]
pub type W = crate::W<TasksNextstepSpec>;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksNextstep {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksNextstep> for bool {
    #[inline(always)]
    fn from(variant: TasksNextstep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_NEXTSTEP` writer - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub type TasksNextstepW<'a, REG> = crate::BitWriter<'a, REG, TasksNextstep>;
impl<'a, REG> TasksNextstepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksNextstep::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    #[inline(always)]
    pub fn tasks_nextstep(&mut self) -> TasksNextstepW<'_, TasksNextstepSpec> {
        TasksNextstepW::new(self, 0)
    }
}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_nextstep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksNextstepSpec;
impl crate::RegisterSpec for TasksNextstepSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_nextstep::W`](W) writer structure"]
impl crate::Writable for TasksNextstepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_NEXTSTEP to value 0"]
impl crate::Resettable for TasksNextstepSpec {}
