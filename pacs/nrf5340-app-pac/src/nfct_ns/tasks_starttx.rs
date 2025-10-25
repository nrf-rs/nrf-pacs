#[doc = "Register `TASKS_STARTTX` writer"]
pub type W = crate::W<TasksStarttxSpec>;
#[doc = "Start transmission of an outgoing frame, change state to transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStarttx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStarttx> for bool {
    #[inline(always)]
    fn from(variant: TasksStarttx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTTX` writer - Start transmission of an outgoing frame, change state to transmit"]
pub type TasksStarttxW<'a, REG> = crate::BitWriter<'a, REG, TasksStarttx>;
impl<'a, REG> TasksStarttxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStarttx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start transmission of an outgoing frame, change state to transmit"]
    #[inline(always)]
    pub fn tasks_starttx(&mut self) -> TasksStarttxW<'_, TasksStarttxSpec> {
        TasksStarttxW::new(self, 0)
    }
}
#[doc = "Start transmission of an outgoing frame, change state to transmit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_starttx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStarttxSpec;
impl crate::RegisterSpec for TasksStarttxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_starttx::W`](W) writer structure"]
impl crate::Writable for TasksStarttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTTX to value 0"]
impl crate::Resettable for TasksStarttxSpec {}
