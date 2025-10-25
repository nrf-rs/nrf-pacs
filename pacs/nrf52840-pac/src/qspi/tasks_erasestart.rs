#[doc = "Register `TASKS_ERASESTART` writer"]
pub type W = crate::W<TasksErasestartSpec>;
#[doc = "Start external flash memory erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksErasestart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksErasestart> for bool {
    #[inline(always)]
    fn from(variant: TasksErasestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ERASESTART` writer - Start external flash memory erase operation"]
pub type TasksErasestartW<'a, REG> = crate::BitWriter<'a, REG, TasksErasestart>;
impl<'a, REG> TasksErasestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksErasestart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start external flash memory erase operation"]
    #[inline(always)]
    pub fn tasks_erasestart(&mut self) -> TasksErasestartW<'_, TasksErasestartSpec> {
        TasksErasestartW::new(self, 0)
    }
}
#[doc = "Start external flash memory erase operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_erasestart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksErasestartSpec;
impl crate::RegisterSpec for TasksErasestartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_erasestart::W`](W) writer structure"]
impl crate::Writable for TasksErasestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ERASESTART to value 0"]
impl crate::Resettable for TasksErasestartSpec {}
