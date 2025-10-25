#[doc = "Register `TASKS_DISABLE` writer"]
pub type W = crate::W<TasksDisableSpec>;
#[doc = "Disable NFCT peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksDisable {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksDisable> for bool {
    #[inline(always)]
    fn from(variant: TasksDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_DISABLE` writer - Disable NFCT peripheral"]
pub type TasksDisableW<'a, REG> = crate::BitWriter<'a, REG, TasksDisable>;
impl<'a, REG> TasksDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksDisable::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Disable NFCT peripheral"]
    #[inline(always)]
    pub fn tasks_disable(&mut self) -> TasksDisableW<'_, TasksDisableSpec> {
        TasksDisableW::new(self, 0)
    }
}
#[doc = "Disable NFCT peripheral\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksDisableSpec;
impl crate::RegisterSpec for TasksDisableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_disable::W`](W) writer structure"]
impl crate::Writable for TasksDisableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_DISABLE to value 0"]
impl crate::Resettable for TasksDisableSpec {}
