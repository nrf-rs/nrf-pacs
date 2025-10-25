#[doc = "Register `TASKS_RSSISTART` writer"]
pub type W = crate::W<TasksRssistartSpec>;
#[doc = "Start the RSSI and take one single sample of the receive signal strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRssistart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRssistart> for bool {
    #[inline(always)]
    fn from(variant: TasksRssistart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_RSSISTART` writer - Start the RSSI and take one single sample of the receive signal strength"]
pub type TasksRssistartW<'a, REG> = crate::BitWriter<'a, REG, TasksRssistart>;
impl<'a, REG> TasksRssistartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRssistart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start the RSSI and take one single sample of the receive signal strength"]
    #[inline(always)]
    pub fn tasks_rssistart(&mut self) -> TasksRssistartW<'_, TasksRssistartSpec> {
        TasksRssistartW::new(self, 0)
    }
}
#[doc = "Start the RSSI and take one single sample of the receive signal strength\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRssistartSpec;
impl crate::RegisterSpec for TasksRssistartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_rssistart::W`](W) writer structure"]
impl crate::Writable for TasksRssistartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_RSSISTART to value 0"]
impl crate::Resettable for TasksRssistartSpec {}
