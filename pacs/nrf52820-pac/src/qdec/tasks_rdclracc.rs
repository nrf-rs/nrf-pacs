#[doc = "Register `TASKS_RDCLRACC` writer"]
pub type W = crate::W<TasksRdclraccSpec>;
#[doc = "Read and clear ACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRdclracc {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRdclracc> for bool {
    #[inline(always)]
    fn from(variant: TasksRdclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RDCLRACC` writer - Read and clear ACC"]
pub type TasksRdclraccW<'a, REG> = crate::BitWriter<'a, REG, TasksRdclracc>;
impl<'a, REG> TasksRdclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRdclracc::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Read and clear ACC"]
    #[inline(always)]
    pub fn tasks_rdclracc(&mut self) -> TasksRdclraccW<'_, TasksRdclraccSpec> {
        TasksRdclraccW::new(self, 0)
    }
}
#[doc = "Read and clear ACC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclracc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRdclraccSpec;
impl crate::RegisterSpec for TasksRdclraccSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rdclracc::W`](W) writer structure"]
impl crate::Writable for TasksRdclraccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RDCLRACC to value 0"]
impl crate::Resettable for TasksRdclraccSpec {}
