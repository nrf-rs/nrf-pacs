#[doc = "Register `TASKS_GOIDLE` writer"]
pub type W = crate::W<TasksGoidleSpec>;
#[doc = "Force state machine to IDLE state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksGoidle {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksGoidle> for bool {
    #[inline(always)]
    fn from(variant: TasksGoidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_GOIDLE` writer - Force state machine to IDLE state"]
pub type TasksGoidleW<'a, REG> = crate::BitWriter<'a, REG, TasksGoidle>;
impl<'a, REG> TasksGoidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksGoidle::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Force state machine to IDLE state"]
    #[inline(always)]
    pub fn tasks_goidle(&mut self) -> TasksGoidleW<'_, TasksGoidleSpec> {
        TasksGoidleW::new(self, 0)
    }
}
#[doc = "Force state machine to IDLE state\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_goidle::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksGoidleSpec;
impl crate::RegisterSpec for TasksGoidleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_goidle::W`](W) writer structure"]
impl crate::Writable for TasksGoidleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_GOIDLE to value 0"]
impl crate::Resettable for TasksGoidleSpec {}
