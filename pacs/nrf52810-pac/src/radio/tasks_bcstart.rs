#[doc = "Register `TASKS_BCSTART` writer"]
pub type W = crate::W<TasksBcstartSpec>;
#[doc = "Start the bit counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksBcstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksBcstart> for bool {
    #[inline(always)]
    fn from(variant: TasksBcstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_BCSTART` writer - Start the bit counter"]
pub type TasksBcstartW<'a, REG> = crate::BitWriter<'a, REG, TasksBcstart>;
impl<'a, REG> TasksBcstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksBcstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start the bit counter"]
    #[inline(always)]
    pub fn tasks_bcstart(&mut self) -> TasksBcstartW<'_, TasksBcstartSpec> {
        TasksBcstartW::new(self, 0)
    }
}
#[doc = "Start the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksBcstartSpec;
impl crate::RegisterSpec for TasksBcstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_bcstart::W`](W) writer structure"]
impl crate::Writable for TasksBcstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_BCSTART to value 0"]
impl crate::Resettable for TasksBcstartSpec {}
