#[doc = "Register `TASKS_EP0STATUS` writer"]
pub type W = crate::W<TasksEp0statusSpec>;
#[doc = "Allows status stage on control endpoint 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEp0status {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEp0status> for bool {
    #[inline(always)]
    fn from(variant: TasksEp0status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EP0STATUS` writer - Allows status stage on control endpoint 0"]
pub type TasksEp0statusW<'a, REG> = crate::BitWriter<'a, REG, TasksEp0status>;
impl<'a, REG> TasksEp0statusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEp0status::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Allows status stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0status(&mut self) -> TasksEp0statusW<'_, TasksEp0statusSpec> {
        TasksEp0statusW::new(self, 0)
    }
}
#[doc = "Allows status stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEp0statusSpec;
impl crate::RegisterSpec for TasksEp0statusSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ep0status::W`](W) writer structure"]
impl crate::Writable for TasksEp0statusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_EP0STATUS to value 0"]
impl crate::Resettable for TasksEp0statusSpec {}
