#[doc = "Register `TASKS_HFCLKSTOP` writer"]
pub type W = crate::W<TasksHfclkstopSpec>;
#[doc = "Stop HFCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclkstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclkstop> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclkstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLKSTOP` writer - Stop HFCLK source"]
pub type TasksHfclkstopW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclkstop>;
impl<'a, REG> TasksHfclkstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclkstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop HFCLK source"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_hfclkstop(&mut self) -> TasksHfclkstopW<TasksHfclkstopSpec> {
        TasksHfclkstopW::new(self, 0)
    }
}
#[doc = "Stop HFCLK source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_hfclkstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclkstopSpec;
impl crate::RegisterSpec for TasksHfclkstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclkstop::W`](W) writer structure"]
impl crate::Writable for TasksHfclkstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_HFCLKSTOP to value 0"]
impl crate::Resettable for TasksHfclkstopSpec {
    const RESET_VALUE: u32 = 0;
}
