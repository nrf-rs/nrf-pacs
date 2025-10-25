#[doc = "Register `TASKS_RATEOVERRIDE` writer"]
pub type W = crate::W<TasksRateoverrideSpec>;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRateoverride {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRateoverride> for bool {
    #[inline(always)]
    fn from(variant: TasksRateoverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RATEOVERRIDE` writer - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub type TasksRateoverrideW<'a, REG> = crate::BitWriter<'a, REG, TasksRateoverride>;
impl<'a, REG> TasksRateoverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRateoverride::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    #[inline(always)]
    pub fn tasks_rateoverride(&mut self) -> TasksRateoverrideW<'_, TasksRateoverrideSpec> {
        TasksRateoverrideW::new(self, 0)
    }
}
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rateoverride::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRateoverrideSpec;
impl crate::RegisterSpec for TasksRateoverrideSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rateoverride::W`](W) writer structure"]
impl crate::Writable for TasksRateoverrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RATEOVERRIDE to value 0"]
impl crate::Resettable for TasksRateoverrideSpec {}
