#[doc = "Register `TASKS_SEQSTART[%s]` writer"]
pub type W = crate::W<TasksSeqstartSpec>;
#[doc = "Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSeqstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSeqstart> for bool {
    #[inline(always)]
    fn from(variant: TasksSeqstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SEQSTART` writer - Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub type TasksSeqstartW<'a, REG> = crate::BitWriter<'a, REG, TasksSeqstart>;
impl<'a, REG> TasksSeqstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSeqstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_seqstart(&mut self) -> TasksSeqstartW<TasksSeqstartSpec> {
        TasksSeqstartW::new(self, 0)
    }
}
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_seqstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSeqstartSpec;
impl crate::RegisterSpec for TasksSeqstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_seqstart::W`](W) writer structure"]
impl crate::Writable for TasksSeqstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_SEQSTART[%s]
to value 0"]
impl crate::Resettable for TasksSeqstartSpec {
    const RESET_VALUE: u32 = 0;
}
