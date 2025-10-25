#[doc = "Register `TASKS_HFCLKSTART` writer"]
pub type W = crate::W<TasksHfclkstartSpec>;
#[doc = "Start HFCLK crystal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclkstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclkstart> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclkstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLKSTART` writer - Start HFCLK crystal oscillator"]
pub type TasksHfclkstartW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclkstart>;
impl<'a, REG> TasksHfclkstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclkstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start HFCLK crystal oscillator"]
    #[inline(always)]
    pub fn tasks_hfclkstart(&mut self) -> TasksHfclkstartW<'_, TasksHfclkstartSpec> {
        TasksHfclkstartW::new(self, 0)
    }
}
#[doc = "Start HFCLK crystal oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclkstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkstartSpec;
impl crate::RegisterSpec for TasksHfclkstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkstart::W`](W) writer structure"]
impl crate::Writable for TasksHfclkstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLKSTART to value 0"]
impl crate::Resettable for TasksHfclkstartSpec {}
