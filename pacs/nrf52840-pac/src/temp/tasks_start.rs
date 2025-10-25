#[doc = "Register `TASKS_START` writer"]
pub type W = crate::W<TasksStartSpec>;
#[doc = "Start temperature measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStart> for bool {
    #[inline(always)]
    fn from(variant: TasksStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_START` writer - Start temperature measurement"]
pub type TasksStartW<'a, REG> = crate::BitWriter<'a, REG, TasksStart>;
impl<'a, REG> TasksStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start temperature measurement"]
    #[inline(always)]
    pub fn tasks_start(&mut self) -> TasksStartW<'_, TasksStartSpec> {
        TasksStartW::new(self, 0)
    }
}
#[doc = "Start temperature measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartSpec;
impl crate::RegisterSpec for TasksStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_start::W`](W) writer structure"]
impl crate::Writable for TasksStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_START to value 0"]
impl crate::Resettable for TasksStartSpec {}
