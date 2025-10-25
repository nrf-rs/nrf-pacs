#[doc = "Register `TASKS_CLEAR` writer"]
pub type W = crate::W<TasksClearSpec>;
#[doc = "Clear time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClear {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClear> for bool {
    #[inline(always)]
    fn from(variant: TasksClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLEAR` writer - Clear time"]
pub type TasksClearW<'a, REG> = crate::BitWriter<'a, REG, TasksClear>;
impl<'a, REG> TasksClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClear::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Clear time"]
    #[inline(always)]
    pub fn tasks_clear(&mut self) -> TasksClearW<'_, TasksClearSpec> {
        TasksClearW::new(self, 0)
    }
}
#[doc = "Clear time\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClearSpec;
impl crate::RegisterSpec for TasksClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clear::W`](W) writer structure"]
impl crate::Writable for TasksClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CLEAR to value 0"]
impl crate::Resettable for TasksClearSpec {}
