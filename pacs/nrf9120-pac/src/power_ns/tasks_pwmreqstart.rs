#[doc = "Register `TASKS_PWMREQSTART` writer"]
pub type W = crate::W<TasksPwmreqstartSpec>;
#[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPwmreqstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPwmreqstart> for bool {
    #[inline(always)]
    fn from(variant: TasksPwmreqstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PWMREQSTART` writer - Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
pub type TasksPwmreqstartW<'a, REG> = crate::BitWriter<'a, REG, TasksPwmreqstart>;
impl<'a, REG> TasksPwmreqstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPwmreqstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_pwmreqstart(&mut self) -> TasksPwmreqstartW<TasksPwmreqstartSpec> {
        TasksPwmreqstartW::new(self, 0)
    }
}
#[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR).\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_pwmreqstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPwmreqstartSpec;
impl crate::RegisterSpec for TasksPwmreqstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pwmreqstart::W`](W) writer structure"]
impl crate::Writable for TasksPwmreqstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PWMREQSTART to value 0"]
impl crate::Resettable for TasksPwmreqstartSpec {
    const RESET_VALUE: u32 = 0;
}
