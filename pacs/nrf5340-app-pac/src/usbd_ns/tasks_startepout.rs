#[doc = "Register `TASKS_STARTEPOUT[%s]` writer"]
pub type W = crate::W<TasksStartepoutSpec>;
#[doc = "Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartepout {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartepout> for bool {
    #[inline(always)]
    fn from(variant: TasksStartepout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTEPOUT` writer - Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub type TasksStartepoutW<'a, REG> = crate::BitWriter<'a, REG, TasksStartepout>;
impl<'a, REG> TasksStartepoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartepout::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepout(&mut self) -> TasksStartepoutW<'_, TasksStartepoutSpec> {
        TasksStartepoutW::new(self, 0)
    }
}
#[doc = "Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startepout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartepoutSpec;
impl crate::RegisterSpec for TasksStartepoutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startepout::W`](W) writer structure"]
impl crate::Writable for TasksStartepoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTEPOUT[%s] to value 0"]
impl crate::Resettable for TasksStartepoutSpec {}
