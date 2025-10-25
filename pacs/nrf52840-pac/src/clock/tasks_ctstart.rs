#[doc = "Register `TASKS_CTSTART` writer"]
pub type W = crate::W<TasksCtstartSpec>;
#[doc = "Start calibration timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCtstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCtstart> for bool {
    #[inline(always)]
    fn from(variant: TasksCtstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CTSTART` writer - Start calibration timer"]
pub type TasksCtstartW<'a, REG> = crate::BitWriter<'a, REG, TasksCtstart>;
impl<'a, REG> TasksCtstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCtstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start calibration timer"]
    #[inline(always)]
    pub fn tasks_ctstart(&mut self) -> TasksCtstartW<'_, TasksCtstartSpec> {
        TasksCtstartW::new(self, 0)
    }
}
#[doc = "Start calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCtstartSpec;
impl crate::RegisterSpec for TasksCtstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ctstart::W`](W) writer structure"]
impl crate::Writable for TasksCtstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CTSTART to value 0"]
impl crate::Resettable for TasksCtstartSpec {}
