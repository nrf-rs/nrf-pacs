#[doc = "Register `TASKS_CAPTURE[%s]` writer"]
pub type W = crate::W<TasksCaptureSpec>;
#[doc = "Capture RTC counter to CC\\[n\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCapture {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCapture> for bool {
    #[inline(always)]
    fn from(variant: TasksCapture) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CAPTURE` writer - Capture RTC counter to CC\\[n\\] register"]
pub type TasksCaptureW<'a, REG> = crate::BitWriter<'a, REG, TasksCapture>;
impl<'a, REG> TasksCaptureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCapture::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Capture RTC counter to CC\\[n\\] register"]
    #[inline(always)]
    pub fn tasks_capture(&mut self) -> TasksCaptureW<'_, TasksCaptureSpec> {
        TasksCaptureW::new(self, 0)
    }
}
#[doc = "Description collection: Capture RTC counter to CC\\[n\\] register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCaptureSpec;
impl crate::RegisterSpec for TasksCaptureSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_capture::W`](W) writer structure"]
impl crate::Writable for TasksCaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CAPTURE[%s] to value 0"]
impl crate::Resettable for TasksCaptureSpec {}
