#[doc = "Register `REGION` reader"]
pub type R = crate::R<RegionSpec>;
#[doc = "Register `REGION` writer"]
pub type W = crate::W<RegionSpec>;
#[doc = "Field `REGION` reader - Region number"]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - Region number"]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: This register can be updated"]
    Unlocked = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
impl R {
    #[doc = "Bits 0:4 - Region number"]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Region number"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> RegionW<RegionSpec> {
        RegionW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<RegionSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegionSpec;
impl crate::RegisterSpec for RegionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region::R`](R) reader structure"]
impl crate::Readable for RegionSpec {}
#[doc = "`write(|w| ..)` method takes [`region::W`](W) writer structure"]
impl crate::Writable for RegionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION to value 0"]
impl crate::Resettable for RegionSpec {
    const RESET_VALUE: u32 = 0;
}
