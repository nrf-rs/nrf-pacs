#[doc = "Register `HFCLKALWAYSRUN` reader"]
pub type R = crate::R<HfclkalwaysrunSpec>;
#[doc = "Register `HFCLKALWAYSRUN` writer"]
pub type W = crate::W<HfclkalwaysrunSpec>;
#[doc = "Ensure clock is always running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alwaysrun {
    #[doc = "0: Use automatic clock control"]
    Automatic = 0,
    #[doc = "1: Ensure clock is always running"]
    AlwaysRun = 1,
}
impl From<Alwaysrun> for bool {
    #[inline(always)]
    fn from(variant: Alwaysrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSRUN` reader - Ensure clock is always running"]
pub type AlwaysrunR = crate::BitReader<Alwaysrun>;
impl AlwaysrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alwaysrun {
        match self.bits {
            false => Alwaysrun::Automatic,
            true => Alwaysrun::AlwaysRun,
        }
    }
    #[doc = "Use automatic clock control"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Alwaysrun::Automatic
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn is_always_run(&self) -> bool {
        *self == Alwaysrun::AlwaysRun
    }
}
#[doc = "Field `ALWAYSRUN` writer - Ensure clock is always running"]
pub type AlwaysrunW<'a, REG> = crate::BitWriter<'a, REG, Alwaysrun>;
impl<'a, REG> AlwaysrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use automatic clock control"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Alwaysrun::Automatic)
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn always_run(self) -> &'a mut crate::W<REG> {
        self.variant(Alwaysrun::AlwaysRun)
    }
}
impl R {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&self) -> AlwaysrunR {
        AlwaysrunR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&mut self) -> AlwaysrunW<'_, HfclkalwaysrunSpec> {
        AlwaysrunW::new(self, 0)
    }
}
#[doc = "Automatic or manual control of HFCLK128M/HFCLK64M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkalwaysrun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkalwaysrun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkalwaysrunSpec;
impl crate::RegisterSpec for HfclkalwaysrunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkalwaysrun::R`](R) reader structure"]
impl crate::Readable for HfclkalwaysrunSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclkalwaysrun::W`](W) writer structure"]
impl crate::Writable for HfclkalwaysrunSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCLKALWAYSRUN to value 0"]
impl crate::Resettable for HfclkalwaysrunSpec {}
