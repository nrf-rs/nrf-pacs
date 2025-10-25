#[doc = "Register `TASKS_PWMREQSTOP` writer"]
pub type W = crate::W<TasksPwmreqstopSpec>;
#[doc = "Stop requesting forcing PWM mode in PMIC DC/DC buck regulator\n\nValue on reset: 0"]
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
#[doc = "Field `TASKS_PWMREQSTOP` writer - Stop requesting forcing PWM mode in PMIC DC/DC buck regulator"]
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
    #[doc = "Bit 0 - Stop requesting forcing PWM mode in PMIC DC/DC buck regulator"]
    #[inline(always)]
    pub fn tasks_pwmreqstop(&mut self) -> TasksPwmreqstopW<'_, TasksPwmreqstopSpec> {
        TasksPwmreqstopW::new(self, 0)
    }
}
#[doc = "Stop requesting forcing PWM mode in PMIC DC/DC buck regulator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pwmreqstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPwmreqstopSpec;
impl crate::RegisterSpec for TasksPwmreqstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pwmreqstop::W`](W) writer structure"]
impl crate::Writable for TasksPwmreqstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_PWMREQSTOP to value 0"]
impl crate::Resettable for TasksPwmreqstopSpec {}
