#[doc = "Register `TASKS_DPDMNODRIVE` writer"]
pub type W = crate::W<TasksDpdmnodriveSpec>;
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksDpdmnodrive {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksDpdmnodrive> for bool {
    #[inline(always)]
    fn from(variant: TasksDpdmnodrive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_DPDMNODRIVE` writer - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub type TasksDpdmnodriveW<'a, REG> = crate::BitWriter<'a, REG, TasksDpdmnodrive>;
impl<'a, REG> TasksDpdmnodriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksDpdmnodrive::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    #[inline(always)]
    pub fn tasks_dpdmnodrive(&mut self) -> TasksDpdmnodriveW<'_, TasksDpdmnodriveSpec> {
        TasksDpdmnodriveW::new(self, 0)
    }
}
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_dpdmnodrive::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksDpdmnodriveSpec;
impl crate::RegisterSpec for TasksDpdmnodriveSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_dpdmnodrive::W`](W) writer structure"]
impl crate::Writable for TasksDpdmnodriveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_DPDMNODRIVE to value 0"]
impl crate::Resettable for TasksDpdmnodriveSpec {}
