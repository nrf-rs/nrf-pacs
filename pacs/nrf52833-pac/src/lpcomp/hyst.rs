#[doc = "Register `HYST` reader"]
pub type R = crate::R<HystSpec>;
#[doc = "Register `HYST` writer"]
pub type W = crate::W<HystSpec>;
#[doc = "Comparator hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hyst {
    #[doc = "0: Comparator hysteresis disabled"]
    Disabled = 0,
    #[doc = "1: Comparator hysteresis enabled"]
    Enabled = 1,
}
impl From<Hyst> for bool {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST` reader - Comparator hysteresis enable"]
pub type HystR = crate::BitReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            false => Hyst::Disabled,
            true => Hyst::Enabled,
        }
    }
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hyst::Disabled
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hyst::Enabled
    }
}
#[doc = "Field `HYST` writer - Comparator hysteresis enable"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG, Hyst>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Disabled)
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, HystSpec> {
        HystW::new(self, 0)
    }
}
#[doc = "Comparator hysteresis enable\n\nYou can [`read`](crate::Reg::read) this register and get [`hyst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HystSpec;
impl crate::RegisterSpec for HystSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hyst::R`](R) reader structure"]
impl crate::Readable for HystSpec {}
#[doc = "`write(|w| ..)` method takes [`hyst::W`](W) writer structure"]
impl crate::Writable for HystSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HYST to value 0"]
impl crate::Resettable for HystSpec {}
