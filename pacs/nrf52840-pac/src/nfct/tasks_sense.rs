#[doc = "Register `TASKS_SENSE` writer"]
pub type W = crate::W<TasksSenseSpec>;
#[doc = "Enable NFC sense field mode, change state to sense mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSense {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSense> for bool {
    #[inline(always)]
    fn from(variant: TasksSense) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SENSE` writer - Enable NFC sense field mode, change state to sense mode"]
pub type TasksSenseW<'a, REG> = crate::BitWriter<'a, REG, TasksSense>;
impl<'a, REG> TasksSenseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSense::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NFC sense field mode, change state to sense mode"]
    #[inline(always)]
    pub fn tasks_sense(&mut self) -> TasksSenseW<'_, TasksSenseSpec> {
        TasksSenseW::new(self, 0)
    }
}
#[doc = "Enable NFC sense field mode, change state to sense mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sense::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSenseSpec;
impl crate::RegisterSpec for TasksSenseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_sense::W`](W) writer structure"]
impl crate::Writable for TasksSenseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SENSE to value 0"]
impl crate::Resettable for TasksSenseSpec {}
