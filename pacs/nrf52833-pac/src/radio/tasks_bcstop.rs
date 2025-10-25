#[doc = "Register `TASKS_BCSTOP` writer"]
pub type W = crate::W<TasksBcstopSpec>;
#[doc = "Stop the bit counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksBcstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksBcstop> for bool {
    #[inline(always)]
    fn from(variant: TasksBcstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_BCSTOP` writer - Stop the bit counter"]
pub type TasksBcstopW<'a, REG> = crate::BitWriter<'a, REG, TasksBcstop>;
impl<'a, REG> TasksBcstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksBcstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the bit counter"]
    #[inline(always)]
    pub fn tasks_bcstop(&mut self) -> TasksBcstopW<'_, TasksBcstopSpec> {
        TasksBcstopW::new(self, 0)
    }
}
#[doc = "Stop the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksBcstopSpec;
impl crate::RegisterSpec for TasksBcstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_bcstop::W`](W) writer structure"]
impl crate::Writable for TasksBcstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_BCSTOP to value 0"]
impl crate::Resettable for TasksBcstopSpec {}
