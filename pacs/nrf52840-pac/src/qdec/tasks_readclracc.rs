#[doc = "Register `TASKS_READCLRACC` writer"]
pub type W = crate::W<TasksReadclraccSpec>;
#[doc = "Read and clear ACC and ACCDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksReadclracc {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksReadclracc> for bool {
    #[inline(always)]
    fn from(variant: TasksReadclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_READCLRACC` writer - Read and clear ACC and ACCDBL"]
pub type TasksReadclraccW<'a, REG> = crate::BitWriter<'a, REG, TasksReadclracc>;
impl<'a, REG> TasksReadclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksReadclracc::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Read and clear ACC and ACCDBL"]
    #[inline(always)]
    pub fn tasks_readclracc(&mut self) -> TasksReadclraccW<'_, TasksReadclraccSpec> {
        TasksReadclraccW::new(self, 0)
    }
}
#[doc = "Read and clear ACC and ACCDBL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readclracc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReadclraccSpec;
impl crate::RegisterSpec for TasksReadclraccSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_readclracc::W`](W) writer structure"]
impl crate::Writable for TasksReadclraccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_READCLRACC to value 0"]
impl crate::Resettable for TasksReadclraccSpec {}
