#[doc = "Register `TASKS_HFCLK192MSTOP` writer"]
pub type W = crate::W<TasksHfclk192mstopSpec>;
#[doc = "Stop HFCLK192M source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclk192mstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclk192mstop> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclk192mstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLK192MSTOP` writer - Stop HFCLK192M source"]
pub type TasksHfclk192mstopW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclk192mstop>;
impl<'a, REG> TasksHfclk192mstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclk192mstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop HFCLK192M source"]
    #[inline(always)]
    pub fn tasks_hfclk192mstop(&mut self) -> TasksHfclk192mstopW<'_, TasksHfclk192mstopSpec> {
        TasksHfclk192mstopW::new(self, 0)
    }
}
#[doc = "Stop HFCLK192M source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclk192mstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclk192mstopSpec;
impl crate::RegisterSpec for TasksHfclk192mstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclk192mstop::W`](W) writer structure"]
impl crate::Writable for TasksHfclk192mstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLK192MSTOP to value 0"]
impl crate::Resettable for TasksHfclk192mstopSpec {}
