#[doc = "Register `TASKS_RSSISTOP` writer"]
pub type W = crate::W<TasksRssistopSpec>;
#[doc = "Stop the RSSI measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRssistop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRssistop> for bool {
    #[inline(always)]
    fn from(variant: TasksRssistop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RSSISTOP` writer - Stop the RSSI measurement"]
pub type TasksRssistopW<'a, REG> = crate::BitWriter<'a, REG, TasksRssistop>;
impl<'a, REG> TasksRssistopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRssistop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the RSSI measurement"]
    #[inline(always)]
    pub fn tasks_rssistop(&mut self) -> TasksRssistopW<'_, TasksRssistopSpec> {
        TasksRssistopW::new(self, 0)
    }
}
#[doc = "Stop the RSSI measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRssistopSpec;
impl crate::RegisterSpec for TasksRssistopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rssistop::W`](W) writer structure"]
impl crate::Writable for TasksRssistopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RSSISTOP to value 0"]
impl crate::Resettable for TasksRssistopSpec {}
