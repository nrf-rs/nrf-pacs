#[doc = "Register `TASKS_LFCLKSTOP` writer"]
pub type W = crate::W<TasksLfclkstopSpec>;
#[doc = "Stop LFCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksLfclkstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksLfclkstop> for bool {
    #[inline(always)]
    fn from(variant: TasksLfclkstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_LFCLKSTOP` writer - Stop LFCLK source"]
pub type TasksLfclkstopW<'a, REG> = crate::BitWriter<'a, REG, TasksLfclkstop>;
impl<'a, REG> TasksLfclkstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksLfclkstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop LFCLK source"]
    #[inline(always)]
    pub fn tasks_lfclkstop(&mut self) -> TasksLfclkstopW<'_, TasksLfclkstopSpec> {
        TasksLfclkstopW::new(self, 0)
    }
}
#[doc = "Stop LFCLK source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLfclkstopSpec;
impl crate::RegisterSpec for TasksLfclkstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lfclkstop::W`](W) writer structure"]
impl crate::Writable for TasksLfclkstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LFCLKSTOP to value 0"]
impl crate::Resettable for TasksLfclkstopSpec {}
