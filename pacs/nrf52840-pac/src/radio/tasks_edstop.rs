#[doc = "Register `TASKS_EDSTOP` writer"]
pub type W = crate::W<TasksEdstopSpec>;
#[doc = "Stop the energy detect measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEdstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEdstop> for bool {
    #[inline(always)]
    fn from(variant: TasksEdstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EDSTOP` writer - Stop the energy detect measurement"]
pub type TasksEdstopW<'a, REG> = crate::BitWriter<'a, REG, TasksEdstop>;
impl<'a, REG> TasksEdstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEdstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the energy detect measurement"]
    #[inline(always)]
    pub fn tasks_edstop(&mut self) -> TasksEdstopW<'_, TasksEdstopSpec> {
        TasksEdstopW::new(self, 0)
    }
}
#[doc = "Stop the energy detect measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEdstopSpec;
impl crate::RegisterSpec for TasksEdstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_edstop::W`](W) writer structure"]
impl crate::Writable for TasksEdstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_EDSTOP to value 0"]
impl crate::Resettable for TasksEdstopSpec {}
