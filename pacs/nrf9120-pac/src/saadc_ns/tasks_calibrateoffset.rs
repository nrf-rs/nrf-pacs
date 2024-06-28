#[doc = "Register `TASKS_CALIBRATEOFFSET` writer"]
pub type W = crate::W<TasksCalibrateoffsetSpec>;
#[doc = "Starts offset auto-calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCalibrateoffset {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCalibrateoffset> for bool {
    #[inline(always)]
    fn from(variant: TasksCalibrateoffset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CALIBRATEOFFSET` writer - Starts offset auto-calibration"]
pub type TasksCalibrateoffsetW<'a, REG> = crate::BitWriter<'a, REG, TasksCalibrateoffset>;
impl<'a, REG> TasksCalibrateoffsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCalibrateoffset::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Starts offset auto-calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_calibrateoffset(&mut self) -> TasksCalibrateoffsetW<TasksCalibrateoffsetSpec> {
        TasksCalibrateoffsetW::new(self, 0)
    }
}
#[doc = "Starts offset auto-calibration\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_calibrateoffset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCalibrateoffsetSpec;
impl crate::RegisterSpec for TasksCalibrateoffsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_calibrateoffset::W`](W) writer structure"]
impl crate::Writable for TasksCalibrateoffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CALIBRATEOFFSET to value 0"]
impl crate::Resettable for TasksCalibrateoffsetSpec {
    const RESET_VALUE: u32 = 0;
}
