#[doc = "Register `TASKS_CLOCKSTOP` writer"]
pub type W = crate::W<TasksClockstopSpec>;
#[doc = "Stop all trace and debug clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClockstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClockstop> for bool {
    #[inline(always)]
    fn from(variant: TasksClockstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLOCKSTOP` writer - Stop all trace and debug clocks."]
pub type TasksClockstopW<'a, REG> = crate::BitWriter<'a, REG, TasksClockstop>;
impl<'a, REG> TasksClockstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClockstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop all trace and debug clocks."]
    #[inline(always)]
    pub fn tasks_clockstop(&mut self) -> TasksClockstopW<'_, TasksClockstopSpec> {
        TasksClockstopW::new(self, 0)
    }
}
#[doc = "Stop all trace and debug clocks.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clockstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClockstopSpec;
impl crate::RegisterSpec for TasksClockstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clockstop::W`](W) writer structure"]
impl crate::Writable for TasksClockstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CLOCKSTOP to value 0"]
impl crate::Resettable for TasksClockstopSpec {}
