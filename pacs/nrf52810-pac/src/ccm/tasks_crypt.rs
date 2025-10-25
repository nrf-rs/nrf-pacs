#[doc = "Register `TASKS_CRYPT` writer"]
pub type W = crate::W<TasksCryptSpec>;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCrypt {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCrypt> for bool {
    #[inline(always)]
    fn from(variant: TasksCrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CRYPT` writer - Start encryption/decryption. This operation will stop by itself when completed."]
pub type TasksCryptW<'a, REG> = crate::BitWriter<'a, REG, TasksCrypt>;
impl<'a, REG> TasksCryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCrypt::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start encryption/decryption. This operation will stop by itself when completed."]
    #[inline(always)]
    pub fn tasks_crypt(&mut self) -> TasksCryptW<'_, TasksCryptSpec> {
        TasksCryptW::new(self, 0)
    }
}
#[doc = "Start encryption/decryption. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_crypt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCryptSpec;
impl crate::RegisterSpec for TasksCryptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_crypt::W`](W) writer structure"]
impl crate::Writable for TasksCryptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CRYPT to value 0"]
impl crate::Resettable for TasksCryptSpec {}
