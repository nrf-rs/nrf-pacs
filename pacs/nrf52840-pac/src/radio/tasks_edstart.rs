#[doc = "Register `TASKS_EDSTART` writer"]
pub type W = crate::W<TasksEdstartSpec>;
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksEdstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksEdstart> for bool {
    #[inline(always)]
    fn from(variant: TasksEdstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EDSTART` writer - Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub type TasksEdstartW<'a, REG> = crate::BitWriter<'a, REG, TasksEdstart>;
impl<'a, REG> TasksEdstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksEdstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub fn tasks_edstart(&mut self) -> TasksEdstartW<'_, TasksEdstartSpec> {
        TasksEdstartW::new(self, 0)
    }
}
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksEdstartSpec;
impl crate::RegisterSpec for TasksEdstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_edstart::W`](W) writer structure"]
impl crate::Writable for TasksEdstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_EDSTART to value 0"]
impl crate::Resettable for TasksEdstartSpec {}
