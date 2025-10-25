#[doc = "Register `TASKS_CONSTLAT` writer"]
pub type W = crate::W<TasksConstlatSpec>;
#[doc = "Enable Constant Latency mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksConstlat {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksConstlat> for bool {
    #[inline(always)]
    fn from(variant: TasksConstlat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CONSTLAT` writer - Enable Constant Latency mode"]
pub type TasksConstlatW<'a, REG> = crate::BitWriter<'a, REG, TasksConstlat>;
impl<'a, REG> TasksConstlatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksConstlat::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Constant Latency mode"]
    #[inline(always)]
    pub fn tasks_constlat(&mut self) -> TasksConstlatW<'_, TasksConstlatSpec> {
        TasksConstlatW::new(self, 0)
    }
}
#[doc = "Enable Constant Latency mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksConstlatSpec;
impl crate::RegisterSpec for TasksConstlatSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_constlat::W`](W) writer structure"]
impl crate::Writable for TasksConstlatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CONSTLAT to value 0"]
impl crate::Resettable for TasksConstlatSpec {}
