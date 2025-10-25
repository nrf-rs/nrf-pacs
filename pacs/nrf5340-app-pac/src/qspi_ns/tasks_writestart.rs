#[doc = "Register `TASKS_WRITESTART` writer"]
pub type W = crate::W<TasksWritestartSpec>;
#[doc = "Start transfer from internal RAM to external flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksWritestart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksWritestart> for bool {
    #[inline(always)]
    fn from(variant: TasksWritestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_WRITESTART` writer - Start transfer from internal RAM to external flash memory"]
pub type TasksWritestartW<'a, REG> = crate::BitWriter<'a, REG, TasksWritestart>;
impl<'a, REG> TasksWritestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksWritestart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start transfer from internal RAM to external flash memory"]
    #[inline(always)]
    pub fn tasks_writestart(&mut self) -> TasksWritestartW<'_, TasksWritestartSpec> {
        TasksWritestartW::new(self, 0)
    }
}
#[doc = "Start transfer from internal RAM to external flash memory\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_writestart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksWritestartSpec;
impl crate::RegisterSpec for TasksWritestartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_writestart::W`](W) writer structure"]
impl crate::Writable for TasksWritestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_WRITESTART to value 0"]
impl crate::Resettable for TasksWritestartSpec {}
