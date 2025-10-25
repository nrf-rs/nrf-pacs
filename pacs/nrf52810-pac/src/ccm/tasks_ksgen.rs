#[doc = "Register `TASKS_KSGEN` writer"]
pub type W = crate::W<TasksKsgenSpec>;
#[doc = "Start generation of key-stream. This operation will stop by itself when completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksKsgen {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksKsgen> for bool {
    #[inline(always)]
    fn from(variant: TasksKsgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_KSGEN` writer - Start generation of key-stream. This operation will stop by itself when completed."]
pub type TasksKsgenW<'a, REG> = crate::BitWriter<'a, REG, TasksKsgen>;
impl<'a, REG> TasksKsgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksKsgen::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start generation of key-stream. This operation will stop by itself when completed."]
    #[inline(always)]
    pub fn tasks_ksgen(&mut self) -> TasksKsgenW<'_, TasksKsgenSpec> {
        TasksKsgenW::new(self, 0)
    }
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ksgen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksKsgenSpec;
impl crate::RegisterSpec for TasksKsgenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ksgen::W`](W) writer structure"]
impl crate::Writable for TasksKsgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_KSGEN to value 0"]
impl crate::Resettable for TasksKsgenSpec {}
