#[doc = "Register `TASKS_ENABLERXDATA` writer"]
pub type W = crate::W<TasksEnablerxdataSpec>;
#[doc = "Initializes the EasyDMA for receive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEnablerxdata {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEnablerxdata> for bool {
    #[inline(always)]
    fn from(variant: TasksEnablerxdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ENABLERXDATA` writer - Initializes the EasyDMA for receive."]
pub type TasksEnablerxdataW<'a, REG> = crate::BitWriter<'a, REG, TasksEnablerxdata>;
impl<'a, REG> TasksEnablerxdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEnablerxdata::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Initializes the EasyDMA for receive."]
    #[inline(always)]
    pub fn tasks_enablerxdata(&mut self) -> TasksEnablerxdataW<'_, TasksEnablerxdataSpec> {
        TasksEnablerxdataW::new(self, 0)
    }
}
#[doc = "Initializes the EasyDMA for receive.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_enablerxdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEnablerxdataSpec;
impl crate::RegisterSpec for TasksEnablerxdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_enablerxdata::W`](W) writer structure"]
impl crate::Writable for TasksEnablerxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ENABLERXDATA to value 0"]
impl crate::Resettable for TasksEnablerxdataSpec {}
