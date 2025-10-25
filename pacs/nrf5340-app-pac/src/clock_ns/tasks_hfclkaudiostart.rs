#[doc = "Register `TASKS_HFCLKAUDIOSTART` writer"]
pub type W = crate::W<TasksHfclkaudiostartSpec>;
#[doc = "Start HFCLKAUDIO source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclkaudiostart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclkaudiostart> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclkaudiostart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLKAUDIOSTART` writer - Start HFCLKAUDIO source"]
pub type TasksHfclkaudiostartW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclkaudiostart>;
impl<'a, REG> TasksHfclkaudiostartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclkaudiostart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start HFCLKAUDIO source"]
    #[inline(always)]
    pub fn tasks_hfclkaudiostart(&mut self) -> TasksHfclkaudiostartW<'_, TasksHfclkaudiostartSpec> {
        TasksHfclkaudiostartW::new(self, 0)
    }
}
#[doc = "Start HFCLKAUDIO source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkaudiostart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkaudiostartSpec;
impl crate::RegisterSpec for TasksHfclkaudiostartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkaudiostart::W`](W) writer structure"]
impl crate::Writable for TasksHfclkaudiostartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLKAUDIOSTART to value 0"]
impl crate::Resettable for TasksHfclkaudiostartSpec {}
