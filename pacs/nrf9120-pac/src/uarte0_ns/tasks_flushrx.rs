#[doc = "Register `TASKS_FLUSHRX` writer"]
pub type W = crate::W<TasksFlushrxSpec>;
#[doc = "Flush RX FIFO into RX buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksFlushrx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksFlushrx> for bool {
    #[inline(always)]
    fn from(variant: TasksFlushrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_FLUSHRX` writer - Flush RX FIFO into RX buffer"]
pub type TasksFlushrxW<'a, REG> = crate::BitWriter<'a, REG, TasksFlushrx>;
impl<'a, REG> TasksFlushrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksFlushrx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Flush RX FIFO into RX buffer"]
    #[inline(always)]
    pub fn tasks_flushrx(&mut self) -> TasksFlushrxW<'_, TasksFlushrxSpec> {
        TasksFlushrxW::new(self, 0)
    }
}
#[doc = "Flush RX FIFO into RX buffer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_flushrx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksFlushrxSpec;
impl crate::RegisterSpec for TasksFlushrxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_flushrx::W`](W) writer structure"]
impl crate::Writable for TasksFlushrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_FLUSHRX to value 0"]
impl crate::Resettable for TasksFlushrxSpec {}
