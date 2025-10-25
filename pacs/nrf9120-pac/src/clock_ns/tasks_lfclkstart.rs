#[doc = "Register `TASKS_LFCLKSTART` writer"]
pub type W = crate::W<TasksLfclkstartSpec>;
#[doc = "Start LFCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksLfclkstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksLfclkstart> for bool {
    #[inline(always)]
    fn from(variant: TasksLfclkstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_LFCLKSTART` writer - Start LFCLK source"]
pub type TasksLfclkstartW<'a, REG> = crate::BitWriter<'a, REG, TasksLfclkstart>;
impl<'a, REG> TasksLfclkstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksLfclkstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start LFCLK source"]
    #[inline(always)]
    pub fn tasks_lfclkstart(&mut self) -> TasksLfclkstartW<'_, TasksLfclkstartSpec> {
        TasksLfclkstartW::new(self, 0)
    }
}
#[doc = "Start LFCLK source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksLfclkstartSpec;
impl crate::RegisterSpec for TasksLfclkstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_lfclkstart::W`](W) writer structure"]
impl crate::Writable for TasksLfclkstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_LFCLKSTART to value 0"]
impl crate::Resettable for TasksLfclkstartSpec {}
