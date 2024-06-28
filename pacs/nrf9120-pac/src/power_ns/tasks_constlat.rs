#[doc = "Register `TASKS_CONSTLAT` writer"]
pub type W = crate::W<TasksConstlatSpec>;
#[doc = "Enable constant latency mode.\n\nValue on reset: 0"]
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
#[doc = "Field `TASKS_CONSTLAT` writer - Enable constant latency mode."]
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
    #[doc = "Bit 0 - Enable constant latency mode."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_constlat(&mut self) -> TasksConstlatW<TasksConstlatSpec> {
        TasksConstlatW::new(self, 0)
    }
}
#[doc = "Enable constant latency mode.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_constlat::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksConstlatSpec;
impl crate::RegisterSpec for TasksConstlatSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_constlat::W`](W) writer structure"]
impl crate::Writable for TasksConstlatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CONSTLAT to value 0"]
impl crate::Resettable for TasksConstlatSpec {
    const RESET_VALUE: u32 = 0;
}
