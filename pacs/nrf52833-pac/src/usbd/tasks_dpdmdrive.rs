#[doc = "Register `TASKS_DPDMDRIVE` writer"]
pub type W = crate::W<TasksDpdmdriveSpec>;
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksDpdmdrive {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksDpdmdrive> for bool {
    #[inline(always)]
    fn from(variant: TasksDpdmdrive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_DPDMDRIVE` writer - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub type TasksDpdmdriveW<'a, REG> = crate::BitWriter<'a, REG, TasksDpdmdrive>;
impl<'a, REG> TasksDpdmdriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksDpdmdrive::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    #[inline(always)]
    pub fn tasks_dpdmdrive(&mut self) -> TasksDpdmdriveW<'_, TasksDpdmdriveSpec> {
        TasksDpdmdriveW::new(self, 0)
    }
}
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_dpdmdrive::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksDpdmdriveSpec;
impl crate::RegisterSpec for TasksDpdmdriveSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_dpdmdrive::W`](W) writer structure"]
impl crate::Writable for TasksDpdmdriveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_DPDMDRIVE to value 0"]
impl crate::Resettable for TasksDpdmdriveSpec {}
