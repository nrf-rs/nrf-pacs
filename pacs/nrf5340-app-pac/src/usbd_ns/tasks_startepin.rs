#[doc = "Register `TASKS_STARTEPIN[%s]` writer"]
pub type W = crate::W<TasksStartepinSpec>;
#[doc = "Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStartepin {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStartepin> for bool {
    #[inline(always)]
    fn from(variant: TasksStartepin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTEPIN` writer - Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub type TasksStartepinW<'a, REG> = crate::BitWriter<'a, REG, TasksStartepin>;
impl<'a, REG> TasksStartepinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStartepin::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepin(&mut self) -> TasksStartepinW<'_, TasksStartepinSpec> {
        TasksStartepinW::new(self, 0)
    }
}
#[doc = "Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_startepin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartepinSpec;
impl crate::RegisterSpec for TasksStartepinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_startepin::W`](W) writer structure"]
impl crate::Writable for TasksStartepinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_STARTEPIN[%s] to value 0"]
impl crate::Resettable for TasksStartepinSpec {}
