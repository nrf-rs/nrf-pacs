#[doc = "Register `MUTEX[%s]` reader"]
pub type R = crate::R<MutexSpec>;
#[doc = "Register `MUTEX[%s]` writer"]
pub type W = crate::W<MutexSpec>;
#[doc = "Mutex register n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutex {
    #[doc = "0: Mutex n is in unlocked state"]
    Unlocked = 0,
    #[doc = "1: Mutex n is in locked state"]
    Locked = 1,
}
impl From<Mutex> for bool {
    #[inline(always)]
    fn from(variant: Mutex) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEX` reader - Mutex register n"]
pub type MutexR = crate::BitReader<Mutex>;
impl MutexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mutex {
        match self.bits {
            false => Mutex::Unlocked,
            true => Mutex::Locked,
        }
    }
    #[doc = "Mutex n is in unlocked state"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Mutex::Unlocked
    }
    #[doc = "Mutex n is in locked state"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Mutex::Locked
    }
}
#[doc = "Field `MUTEX` writer - Mutex register n"]
pub type MutexW<'a, REG> = crate::BitWriter<'a, REG, Mutex>;
impl<'a, REG> MutexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mutex n is in unlocked state"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex::Unlocked)
    }
    #[doc = "Mutex n is in locked state"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&self) -> MutexR {
        MutexR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&mut self) -> MutexW<'_, MutexSpec> {
        MutexW::new(self, 0)
    }
}
#[doc = "Description collection: Mutex register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexSpec;
impl crate::RegisterSpec for MutexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex::R`](R) reader structure"]
impl crate::Readable for MutexSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex::W`](W) writer structure"]
impl crate::Writable for MutexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUTEX[%s] to value 0"]
impl crate::Resettable for MutexSpec {}
