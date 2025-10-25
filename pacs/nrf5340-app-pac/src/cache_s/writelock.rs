#[doc = "Register `WRITELOCK` reader"]
pub type R = crate::R<WritelockSpec>;
#[doc = "Register `WRITELOCK` writer"]
pub type W = crate::W<WritelockSpec>;
#[doc = "Lock cache updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writelock {
    #[doc = "0: Cache updates unlocked"]
    Unlocked = 0,
    #[doc = "1: Cache updates locked"]
    Locked = 1,
}
impl From<Writelock> for bool {
    #[inline(always)]
    fn from(variant: Writelock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITELOCK` reader - Lock cache updates"]
pub type WritelockR = crate::BitReader<Writelock>;
impl WritelockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Writelock {
        match self.bits {
            false => Writelock::Unlocked,
            true => Writelock::Locked,
        }
    }
    #[doc = "Cache updates unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Writelock::Unlocked
    }
    #[doc = "Cache updates locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Writelock::Locked
    }
}
#[doc = "Field `WRITELOCK` writer - Lock cache updates"]
pub type WritelockW<'a, REG> = crate::BitWriter<'a, REG, Writelock>;
impl<'a, REG> WritelockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache updates unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Writelock::Unlocked)
    }
    #[doc = "Cache updates locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Writelock::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&self) -> WritelockR {
        WritelockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&mut self) -> WritelockW<'_, WritelockSpec> {
        WritelockW::new(self, 0)
    }
}
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritelockSpec;
impl crate::RegisterSpec for WritelockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writelock::R`](R) reader structure"]
impl crate::Readable for WritelockSpec {}
#[doc = "`write(|w| ..)` method takes [`writelock::W`](W) writer structure"]
impl crate::Writable for WritelockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITELOCK to value 0"]
impl crate::Resettable for WritelockSpec {}
