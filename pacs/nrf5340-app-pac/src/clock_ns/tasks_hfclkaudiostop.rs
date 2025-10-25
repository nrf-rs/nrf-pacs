#[doc = "Register `TASKS_HFCLKAUDIOSTOP` writer"]
pub type W = crate::W<TasksHfclkaudiostopSpec>;
#[doc = "Stop HFCLKAUDIO source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclkaudiostop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclkaudiostop> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclkaudiostop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLKAUDIOSTOP` writer - Stop HFCLKAUDIO source"]
pub type TasksHfclkaudiostopW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclkaudiostop>;
impl<'a, REG> TasksHfclkaudiostopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclkaudiostop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop HFCLKAUDIO source"]
    #[inline(always)]
    pub fn tasks_hfclkaudiostop(&mut self) -> TasksHfclkaudiostopW<'_, TasksHfclkaudiostopSpec> {
        TasksHfclkaudiostopW::new(self, 0)
    }
}
#[doc = "Stop HFCLKAUDIO source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkaudiostop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkaudiostopSpec;
impl crate::RegisterSpec for TasksHfclkaudiostopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkaudiostop::W`](W) writer structure"]
impl crate::Writable for TasksHfclkaudiostopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLKAUDIOSTOP to value 0"]
impl crate::Resettable for TasksHfclkaudiostopSpec {}
