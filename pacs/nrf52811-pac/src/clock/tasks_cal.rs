#[doc = "Register `TASKS_CAL` writer"]
pub type W = crate::W<TasksCalSpec>;
#[doc = "Start calibration of LFRC oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCal {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCal> for bool {
    #[inline(always)]
    fn from(variant: TasksCal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CAL` writer - Start calibration of LFRC oscillator"]
pub type TasksCalW<'a, REG> = crate::BitWriter<'a, REG, TasksCal>;
impl<'a, REG> TasksCalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCal::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start calibration of LFRC oscillator"]
    #[inline(always)]
    pub fn tasks_cal(&mut self) -> TasksCalW<'_, TasksCalSpec> {
        TasksCalW::new(self, 0)
    }
}
#[doc = "Start calibration of LFRC oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCalSpec;
impl crate::RegisterSpec for TasksCalSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_cal::W`](W) writer structure"]
impl crate::Writable for TasksCalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CAL to value 0"]
impl crate::Resettable for TasksCalSpec {}
