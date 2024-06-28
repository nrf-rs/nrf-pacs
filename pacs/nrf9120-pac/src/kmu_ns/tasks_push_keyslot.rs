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
    #[must_use]
    pub fn tasks_push_keyslot(&mut self) -> TasksPushKeyslotW<TasksPushKeyslotSpec> {
        TasksPushKeyslotW::new(self, 0)
    }
}
#[doc = "Push a key slot over secure APB\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_push_keyslot::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPushKeyslotSpec;
impl crate::RegisterSpec for TasksPushKeyslotSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_push_keyslot::W`](W) writer structure"]
impl crate::Writable for TasksPushKeyslotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PUSH_KEYSLOT to value 0"]
impl crate::Resettable for TasksPushKeyslotSpec {
    const RESET_VALUE: u32 = 0;
}
