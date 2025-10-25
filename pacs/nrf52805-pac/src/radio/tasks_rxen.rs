#[doc = "Register `TASKS_RXEN` writer"]
pub type W = crate::W<TasksRxenSpec>;
#[doc = "Enable RADIO in RX mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRxen {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRxen> for bool {
    #[inline(always)]
    fn from(variant: TasksRxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RXEN` writer - Enable RADIO in RX mode"]
pub type TasksRxenW<'a, REG> = crate::BitWriter<'a, REG, TasksRxen>;
impl<'a, REG> TasksRxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRxen::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RADIO in RX mode"]
    #[inline(always)]
    pub fn tasks_rxen(&mut self) -> TasksRxenW<'_, TasksRxenSpec> {
        TasksRxenW::new(self, 0)
    }
}
#[doc = "Enable RADIO in RX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rxen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRxenSpec;
impl crate::RegisterSpec for TasksRxenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rxen::W`](W) writer structure"]
impl crate::Writable for TasksRxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RXEN to value 0"]
impl crate::Resettable for TasksRxenSpec {}
