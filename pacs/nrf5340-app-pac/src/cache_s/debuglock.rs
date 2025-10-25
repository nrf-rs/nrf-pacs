#[doc = "Register `DEBUGLOCK` reader"]
pub type R = crate::R<DebuglockSpec>;
#[doc = "Register `DEBUGLOCK` writer"]
pub type W = crate::W<DebuglockSpec>;
#[doc = "Lock debug mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debuglock {
    #[doc = "0: Debug mode unlocked"]
    Unlocked = 0,
    #[doc = "1: Debug mode locked"]
    Locked = 1,
}
impl From<Debuglock> for bool {
    #[inline(always)]
    fn from(variant: Debuglock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGLOCK` reader - Lock debug mode"]
pub type DebuglockR = crate::BitReader<Debuglock>;
impl DebuglockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debuglock {
        match self.bits {
            false => Debuglock::Unlocked,
            true => Debuglock::Locked,
        }
    }
    #[doc = "Debug mode unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Debuglock::Unlocked
    }
    #[doc = "Debug mode locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Debuglock::Locked
    }
}
#[doc = "Field `DEBUGLOCK` writer - Lock debug mode"]
pub type DebuglockW<'a, REG> = crate::BitWriter<'a, REG, Debuglock>;
impl<'a, REG> DebuglockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug mode unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Debuglock::Unlocked)
    }
    #[doc = "Debug mode locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Debuglock::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&self) -> DebuglockR {
        DebuglockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&mut self) -> DebuglockW<'_, DebuglockSpec> {
        DebuglockW::new(self, 0)
    }
}
#[doc = "Lock debug mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`debuglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debuglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebuglockSpec;
impl crate::RegisterSpec for DebuglockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debuglock::R`](R) reader structure"]
impl crate::Readable for DebuglockSpec {}
#[doc = "`write(|w| ..)` method takes [`debuglock::W`](W) writer structure"]
impl crate::Writable for DebuglockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGLOCK to value 0"]
impl crate::Resettable for DebuglockSpec {}
