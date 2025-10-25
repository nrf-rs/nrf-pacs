#[doc = "Register `TASKS_ACTIVATE` writer"]
pub type W = crate::W<TasksActivateSpec>;
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksActivate {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksActivate> for bool {
    #[inline(always)]
    fn from(variant: TasksActivate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ACTIVATE` writer - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub type TasksActivateW<'a, REG> = crate::BitWriter<'a, REG, TasksActivate>;
impl<'a, REG> TasksActivateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksActivate::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub fn tasks_activate(&mut self) -> TasksActivateW<'_, TasksActivateSpec> {
        TasksActivateW::new(self, 0)
    }
}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_activate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksActivateSpec;
impl crate::RegisterSpec for TasksActivateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_activate::W`](W) writer structure"]
impl crate::Writable for TasksActivateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_ACTIVATE to value 0"]
impl crate::Resettable for TasksActivateSpec {}
