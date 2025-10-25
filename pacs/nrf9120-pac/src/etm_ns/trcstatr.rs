#[doc = "Register `TRCSTATR` reader"]
pub type R = crate::R<TrcstatrSpec>;
#[doc = "Register `TRCSTATR` writer"]
pub type W = crate::W<TrcstatrSpec>;
#[doc = "Trace unit enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: The trace unit is not idle."]
    NotIdle = 0,
    #[doc = "1: The trace unit is idle."]
    Idle = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Trace unit enable bit"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::NotIdle,
            true => Idle::Idle,
        }
    }
    #[doc = "The trace unit is not idle."]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == Idle::NotIdle
    }
    #[doc = "The trace unit is idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Idle::Idle
    }
}
#[doc = "Field `IDLE` writer - Trace unit enable bit"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit is not idle."]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::NotIdle)
    }
    #[doc = "The trace unit is idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Idle)
    }
}
#[doc = "Programmers' model stable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmstable {
    #[doc = "0: The programmers' model is not stable."]
    NotStable = 0,
    #[doc = "1: The programmers' model is stable."]
    Stable = 1,
}
impl From<Pmstable> for bool {
    #[inline(always)]
    fn from(variant: Pmstable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMSTABLE` reader - Programmers' model stable bit"]
pub type PmstableR = crate::BitReader<Pmstable>;
impl PmstableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmstable {
        match self.bits {
            false => Pmstable::NotStable,
            true => Pmstable::Stable,
        }
    }
    #[doc = "The programmers' model is not stable."]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == Pmstable::NotStable
    }
    #[doc = "The programmers' model is stable."]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == Pmstable::Stable
    }
}
#[doc = "Field `PMSTABLE` writer - Programmers' model stable bit"]
pub type PmstableW<'a, REG> = crate::BitWriter<'a, REG, Pmstable>;
impl<'a, REG> PmstableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The programmers' model is not stable."]
    #[inline(always)]
    pub fn not_stable(self) -> &'a mut crate::W<REG> {
        self.variant(Pmstable::NotStable)
    }
    #[doc = "The programmers' model is stable."]
    #[inline(always)]
    pub fn stable(self) -> &'a mut crate::W<REG> {
        self.variant(Pmstable::Stable)
    }
}
impl R {
    #[doc = "Bit 0 - Trace unit enable bit"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programmers' model stable bit"]
    #[inline(always)]
    pub fn pmstable(&self) -> PmstableR {
        PmstableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace unit enable bit"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, TrcstatrSpec> {
        IdleW::new(self, 0)
    }
    #[doc = "Bit 1 - Programmers' model stable bit"]
    #[inline(always)]
    pub fn pmstable(&mut self) -> PmstableW<'_, TrcstatrSpec> {
        PmstableW::new(self, 1)
    }
}
#[doc = "Idle status bit\n\nYou can [`read`](crate::Reg::read) this register and get [`trcstatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcstatrSpec;
impl crate::RegisterSpec for TrcstatrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcstatr::R`](R) reader structure"]
impl crate::Readable for TrcstatrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcstatr::W`](W) writer structure"]
impl crate::Writable for TrcstatrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSTATR to value 0"]
impl crate::Resettable for TrcstatrSpec {}
