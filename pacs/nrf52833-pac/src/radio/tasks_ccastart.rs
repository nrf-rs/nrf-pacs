#[doc = "Register `TASKS_CCASTART` writer"]
pub type W = crate::W<TasksCcastartSpec>;
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCcastart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCcastart> for bool {
    #[inline(always)]
    fn from(variant: TasksCcastart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CCASTART` writer - Start the clear channel assessment used in IEEE 802.15.4 mode"]
pub type TasksCcastartW<'a, REG> = crate::BitWriter<'a, REG, TasksCcastart>;
impl<'a, REG> TasksCcastartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCcastart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start the clear channel assessment used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub fn tasks_ccastart(&mut self) -> TasksCcastartW<'_, TasksCcastartSpec> {
        TasksCcastartW::new(self, 0)
    }
}
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCcastartSpec;
impl crate::RegisterSpec for TasksCcastartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ccastart::W`](W) writer structure"]
impl crate::Writable for TasksCcastartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CCASTART to value 0"]
impl crate::Resettable for TasksCcastartSpec {}
