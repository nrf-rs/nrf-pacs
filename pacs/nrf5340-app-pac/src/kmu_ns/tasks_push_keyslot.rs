#[doc = "Register `TASKS_PUSH_KEYSLOT` writer"]
pub type W = crate::W<TasksPushKeyslotSpec>;
#[doc = "Push a key slot over secure APB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPushKeyslot {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPushKeyslot> for bool {
    #[inline(always)]
    fn from(variant: TasksPushKeyslot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PUSH_KEYSLOT` writer - Push a key slot over secure APB"]
pub type TasksPushKeyslotW<'a, REG> = crate::BitWriter<'a, REG, TasksPushKeyslot>;
impl<'a, REG> TasksPushKeyslotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPushKeyslot::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Push a key slot over secure APB"]
    #[inline(always)]
    pub fn tasks_push_keyslot(&mut self) -> TasksPushKeyslotW<'_, TasksPushKeyslotSpec> {
        TasksPushKeyslotW::new(self, 0)
    }
}
#[doc = "Push a key slot over secure APB\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_push_keyslot::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPushKeyslotSpec;
impl crate::RegisterSpec for TasksPushKeyslotSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_push_keyslot::W`](W) writer structure"]
impl crate::Writable for TasksPushKeyslotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_PUSH_KEYSLOT to value 0"]
impl crate::Resettable for TasksPushKeyslotSpec {}
