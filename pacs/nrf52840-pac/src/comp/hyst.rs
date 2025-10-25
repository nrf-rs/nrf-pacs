#[doc = "Register `HYST` reader"]
pub type R = crate::R<HystSpec>;
#[doc = "Register `HYST` writer"]
pub type W = crate::W<HystSpec>;
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hyst {
    #[doc = "0: Comparator hysteresis disabled"]
    NoHyst = 0,
    #[doc = "1: Comparator hysteresis enabled"]
    Hyst50mV = 1,
}
impl From<Hyst> for bool {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST` reader - Comparator hysteresis"]
pub type HystR = crate::BitReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            false => Hyst::NoHyst,
            true => Hyst::Hyst50mV,
        }
    }
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn is_no_hyst(&self) -> bool {
        *self == Hyst::NoHyst
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn is_hyst50m_v(&self) -> bool {
        *self == Hyst::Hyst50mV
    }
}
#[doc = "Field `HYST` writer - Comparator hysteresis"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG, Hyst>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn no_hyst(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::NoHyst)
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn hyst50m_v(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst50mV)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator hysteresis"]
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
