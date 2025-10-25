#[doc = "Register `TASKS_STARTISOIN` writer"]
pub type W = crate::W<TasksStartisoinSpec>;
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartisoin {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartisoin> for bool {
    #[inline(always)]
    fn from(variant: TasksStartisoin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTISOIN` writer - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub type TasksStartisoinW<'a, REG> = crate::BitWriter<'a, REG, TasksStartisoin>;
impl<'a, REG> TasksStartisoinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartisoin::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    #[inline(always)]
    pub fn tasks_startisoin(&mut self) -> TasksStartisoinW<'_, TasksStartisoinSpec> {
        TasksStartisoinW::new(self, 0)
    }
}
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startisoin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartisoinSpec;
impl crate::RegisterSpec for TasksStartisoinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startisoin::W`](W) writer structure"]
impl crate::Writable for TasksStartisoinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTISOIN to value 0"]
impl crate::Resettable for TasksStartisoinSpec {}
