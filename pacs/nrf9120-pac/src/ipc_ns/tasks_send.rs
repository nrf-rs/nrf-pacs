#[doc = "Register `TASKS_SEND[%s]` writer"]
pub type W = crate::W<TasksSendSpec>;
#[doc = "Trigger events on IPC channel enabled in SEND_CNF\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSend {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSend> for bool {
    #[inline(always)]
    fn from(variant: TasksSend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SEND` writer - Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
pub type TasksSendW<'a, REG> = crate::BitWriter<'a, REG, TasksSend>;
impl<'a, REG> TasksSendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSend::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_send(&mut self) -> TasksSendW<TasksSendSpec> {
        TasksSendW::new(self, 0)
    }
}
#[doc = "Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_send::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSendSpec;
impl crate::RegisterSpec for TasksSendSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_send::W`](W) writer structure"]
impl crate::Writable for TasksSendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_SEND[%s]
to value 0"]
impl crate::Resettable for TasksSendSpec {
    const RESET_VALUE: u32 = 0;
}
