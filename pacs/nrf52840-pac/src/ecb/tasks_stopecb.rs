#[doc = "Register `TASKS_STOPECB` writer"]
pub type W = crate::W<TasksStopecbSpec>;
#[doc = "Abort a possible executing ECB operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStopecb {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStopecb> for bool {
    #[inline(always)]
    fn from(variant: TasksStopecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STOPECB` writer - Abort a possible executing ECB operation"]
pub type TasksStopecbW<'a, REG> = crate::BitWriter<'a, REG, TasksStopecb>;
impl<'a, REG> TasksStopecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStopecb::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Abort a possible executing ECB operation"]
    #[inline(always)]
    pub fn tasks_stopecb(&mut self) -> TasksStopecbW<'_, TasksStopecbSpec> {
        TasksStopecbW::new(self, 0)
    }
}
#[doc = "Abort a possible executing ECB operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stopecb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStopecbSpec;
impl crate::RegisterSpec for TasksStopecbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stopecb::W`](W) writer structure"]
impl crate::Writable for TasksStopecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STOPECB to value 0"]
impl crate::Resettable for TasksStopecbSpec {}
