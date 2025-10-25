#[doc = "Register `TASKS_STARTECB` writer"]
pub type W = crate::W<TasksStartecbSpec>;
#[doc = "Start ECB block encrypt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartecb {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartecb> for bool {
    #[inline(always)]
    fn from(variant: TasksStartecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTECB` writer - Start ECB block encrypt"]
pub type TasksStartecbW<'a, REG> = crate::BitWriter<'a, REG, TasksStartecb>;
impl<'a, REG> TasksStartecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartecb::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start ECB block encrypt"]
    #[inline(always)]
    pub fn tasks_startecb(&mut self) -> TasksStartecbW<'_, TasksStartecbSpec> {
        TasksStartecbW::new(self, 0)
    }
}
#[doc = "Start ECB block encrypt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startecb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartecbSpec;
impl crate::RegisterSpec for TasksStartecbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startecb::W`](W) writer structure"]
impl crate::Writable for TasksStartecbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTECB to value 0"]
impl crate::Resettable for TasksStartecbSpec {}
