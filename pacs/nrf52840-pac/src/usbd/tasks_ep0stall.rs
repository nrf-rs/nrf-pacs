#[doc = "Register `TASKS_EP0STALL` writer"]
pub type W = crate::W<TasksEp0stallSpec>;
#[doc = "Stalls data and status stage on control endpoint 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEp0stall {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEp0stall> for bool {
    #[inline(always)]
    fn from(variant: TasksEp0stall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EP0STALL` writer - Stalls data and status stage on control endpoint 0"]
pub type TasksEp0stallW<'a, REG> = crate::BitWriter<'a, REG, TasksEp0stall>;
impl<'a, REG> TasksEp0stallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEp0stall::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stalls data and status stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0stall(&mut self) -> TasksEp0stallW<'_, TasksEp0stallSpec> {
        TasksEp0stallW::new(self, 0)
    }
}
#[doc = "Stalls data and status stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0stall::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEp0stallSpec;
impl crate::RegisterSpec for TasksEp0stallSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ep0stall::W`](W) writer structure"]
impl crate::Writable for TasksEp0stallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_EP0STALL to value 0"]
impl crate::Resettable for TasksEp0stallSpec {}
