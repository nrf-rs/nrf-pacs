#[doc = "Register `TASKS_RDCLRDBL` writer"]
pub type W = crate::W<TasksRdclrdblSpec>;
#[doc = "Read and clear ACCDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRdclrdbl {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRdclrdbl> for bool {
    #[inline(always)]
    fn from(variant: TasksRdclrdbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RDCLRDBL` writer - Read and clear ACCDBL"]
pub type TasksRdclrdblW<'a, REG> = crate::BitWriter<'a, REG, TasksRdclrdbl>;
impl<'a, REG> TasksRdclrdblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRdclrdbl::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Read and clear ACCDBL"]
    #[inline(always)]
    pub fn tasks_rdclrdbl(&mut self) -> TasksRdclrdblW<'_, TasksRdclrdblSpec> {
        TasksRdclrdblW::new(self, 0)
    }
}
#[doc = "Read and clear ACCDBL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclrdbl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRdclrdblSpec;
impl crate::RegisterSpec for TasksRdclrdblSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rdclrdbl::W`](W) writer structure"]
impl crate::Writable for TasksRdclrdblSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RDCLRDBL to value 0"]
impl crate::Resettable for TasksRdclrdblSpec {}
