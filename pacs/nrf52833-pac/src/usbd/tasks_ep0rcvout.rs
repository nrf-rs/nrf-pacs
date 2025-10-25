#[doc = "Register `TASKS_EP0RCVOUT` writer"]
pub type W = crate::W<TasksEp0rcvoutSpec>;
#[doc = "Allows OUT data stage on control endpoint 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEp0rcvout {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEp0rcvout> for bool {
    #[inline(always)]
    fn from(variant: TasksEp0rcvout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EP0RCVOUT` writer - Allows OUT data stage on control endpoint 0"]
pub type TasksEp0rcvoutW<'a, REG> = crate::BitWriter<'a, REG, TasksEp0rcvout>;
impl<'a, REG> TasksEp0rcvoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEp0rcvout::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Allows OUT data stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0rcvout(&mut self) -> TasksEp0rcvoutW<'_, TasksEp0rcvoutSpec> {
        TasksEp0rcvoutW::new(self, 0)
    }
}
#[doc = "Allows OUT data stage on control endpoint 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ep0rcvout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEp0rcvoutSpec;
impl crate::RegisterSpec for TasksEp0rcvoutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ep0rcvout::W`](W) writer structure"]
impl crate::Writable for TasksEp0rcvoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_EP0RCVOUT to value 0"]
impl crate::Resettable for TasksEp0rcvoutSpec {}
