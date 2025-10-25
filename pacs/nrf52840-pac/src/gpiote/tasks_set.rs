#[doc = "Register `TASKS_SET[%s]` writer"]
pub type W = crate::W<TasksSetSpec>;
#[doc = "Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSet {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSet> for bool {
    #[inline(always)]
    fn from(variant: TasksSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SET` writer - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
pub type TasksSetW<'a, REG> = crate::BitWriter<'a, REG, TasksSet>;
impl<'a, REG> TasksSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSet::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub fn tasks_set(&mut self) -> TasksSetW<'_, TasksSetSpec> {
        TasksSetW::new(self, 0)
    }
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSetSpec;
impl crate::RegisterSpec for TasksSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_set::W`](W) writer structure"]
impl crate::Writable for TasksSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_SET[%s] to value 0"]
impl crate::Resettable for TasksSetSpec {}
