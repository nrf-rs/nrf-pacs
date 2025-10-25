#[doc = "Register `TASKS_READSTART` writer"]
pub type W = crate::W<TasksReadstartSpec>;
#[doc = "Start transfer from external flash memory to internal RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksReadstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksReadstart> for bool {
    #[inline(always)]
    fn from(variant: TasksReadstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_READSTART` writer - Start transfer from external flash memory to internal RAM"]
pub type TasksReadstartW<'a, REG> = crate::BitWriter<'a, REG, TasksReadstart>;
impl<'a, REG> TasksReadstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksReadstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start transfer from external flash memory to internal RAM"]
    #[inline(always)]
    pub fn tasks_readstart(&mut self) -> TasksReadstartW<'_, TasksReadstartSpec> {
        TasksReadstartW::new(self, 0)
    }
}
#[doc = "Start transfer from external flash memory to internal RAM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReadstartSpec;
impl crate::RegisterSpec for TasksReadstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_readstart::W`](W) writer structure"]
impl crate::Writable for TasksReadstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_READSTART to value 0"]
impl crate::Resettable for TasksReadstartSpec {}
