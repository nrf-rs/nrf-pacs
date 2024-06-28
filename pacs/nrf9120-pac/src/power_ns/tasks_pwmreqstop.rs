#[doc = "Register `TASKS_PWMREQSTOP` writer"]
pub type W = crate::W<TasksPwmreqstopSpec>;
#[doc = "Stop requesting forcing PWM mode in external DC/DC voltage regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPwmreqstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPwmreqstop> for bool {
    #[inline(always)]
    fn from(variant: TasksPwmreqstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PWMREQSTOP` writer - Stop requesting forcing PWM mode in external DC/DC voltage regulator"]
pub type TasksPwmreqstopW<'a, REG> = crate::BitWriter<'a, REG, TasksPwmreqstop>;
impl<'a, REG> TasksPwmreqstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPwmreqstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop requesting forcing PWM mode in external DC/DC voltage regulator"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_pwmreqstop(&mut self) -> TasksPwmreqstopW<TasksPwmreqstopSpec> {
        TasksPwmreqstopW::new(self, 0)
    }
}
#[doc = "Stop requesting forcing PWM mode in external DC/DC voltage regulator\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_pwmreqstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPwmreqstopSpec;
impl crate::RegisterSpec for TasksPwmreqstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pwmreqstop::W`](W) writer structure"]
impl crate::Writable for TasksPwmreqstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PWMREQSTOP to value 0"]
impl crate::Resettable for TasksPwmreqstopSpec {
    const RESET_VALUE: u32 = 0;
}
