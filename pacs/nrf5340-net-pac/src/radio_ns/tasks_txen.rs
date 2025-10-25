#[doc = "Register `TASKS_TXEN` writer"]
pub type W = crate::W<TasksTxenSpec>;
#[doc = "Enable RADIO in TX mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksTxen {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksTxen> for bool {
    #[inline(always)]
    fn from(variant: TasksTxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_TXEN` writer - Enable RADIO in TX mode"]
pub type TasksTxenW<'a, REG> = crate::BitWriter<'a, REG, TasksTxen>;
impl<'a, REG> TasksTxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksTxen::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RADIO in TX mode"]
    #[inline(always)]
    pub fn tasks_txen(&mut self) -> TasksTxenW<'_, TasksTxenSpec> {
        TasksTxenW::new(self, 0)
    }
}
#[doc = "Enable RADIO in TX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_txen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTxenSpec;
impl crate::RegisterSpec for TasksTxenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_txen::W`](W) writer structure"]
impl crate::Writable for TasksTxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_TXEN to value 0"]
impl crate::Resettable for TasksTxenSpec {}
