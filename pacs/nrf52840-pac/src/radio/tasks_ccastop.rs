#[doc = "Register `TASKS_CCASTOP` writer"]
pub type W = crate::W<TasksCcastopSpec>;
#[doc = "Stop the clear channel assessment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCcastop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCcastop> for bool {
    #[inline(always)]
    fn from(variant: TasksCcastop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CCASTOP` writer - Stop the clear channel assessment"]
pub type TasksCcastopW<'a, REG> = crate::BitWriter<'a, REG, TasksCcastop>;
impl<'a, REG> TasksCcastopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCcastop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the clear channel assessment"]
    #[inline(always)]
    pub fn tasks_ccastop(&mut self) -> TasksCcastopW<'_, TasksCcastopSpec> {
        TasksCcastopW::new(self, 0)
    }
}
#[doc = "Stop the clear channel assessment\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCcastopSpec;
impl crate::RegisterSpec for TasksCcastopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ccastop::W`](W) writer structure"]
impl crate::Writable for TasksCcastopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CCASTOP to value 0"]
impl crate::Resettable for TasksCcastopSpec {}
