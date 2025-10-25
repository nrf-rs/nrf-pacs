#[doc = "Register `TASKS_STARTISOOUT` writer"]
pub type W = crate::W<TasksStartisooutSpec>;
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartisoout {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartisoout> for bool {
    #[inline(always)]
    fn from(variant: TasksStartisoout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTISOOUT` writer - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub type TasksStartisooutW<'a, REG> = crate::BitWriter<'a, REG, TasksStartisoout>;
impl<'a, REG> TasksStartisooutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartisoout::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    #[inline(always)]
    pub fn tasks_startisoout(&mut self) -> TasksStartisooutW<'_, TasksStartisooutSpec> {
        TasksStartisooutW::new(self, 0)
    }
}
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startisoout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartisooutSpec;
impl crate::RegisterSpec for TasksStartisooutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startisoout::W`](W) writer structure"]
impl crate::Writable for TasksStartisooutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTISOOUT to value 0"]
impl crate::Resettable for TasksStartisooutSpec {}
