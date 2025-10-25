#[doc = "Register `TASKS_TRIGGER[%s]` writer"]
pub type W = crate::W<TasksTriggerSpec>;
#[doc = "Trigger n for triggering the corresponding TRIGGERED\\[n\\] event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksTrigger {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksTrigger> for bool {
    #[inline(always)]
    fn from(variant: TasksTrigger) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_TRIGGER` writer - Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
pub type TasksTriggerW<'a, REG> = crate::BitWriter<'a, REG, TasksTrigger>;
impl<'a, REG> TasksTriggerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksTrigger::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
    #[inline(always)]
    pub fn tasks_trigger(&mut self) -> TasksTriggerW<'_, TasksTriggerSpec> {
        TasksTriggerW::new(self, 0)
    }
}
#[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTriggerSpec;
impl crate::RegisterSpec for TasksTriggerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_trigger::W`](W) writer structure"]
impl crate::Writable for TasksTriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_TRIGGER[%s] to value 0"]
impl crate::Resettable for TasksTriggerSpec {}
