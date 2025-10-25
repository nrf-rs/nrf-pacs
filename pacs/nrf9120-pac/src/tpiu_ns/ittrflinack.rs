#[doc = "Register `ITTRFLINACK` reader"]
pub type R = crate::R<IttrflinackSpec>;
#[doc = "Register `ITTRFLINACK` writer"]
pub type W = crate::W<IttrflinackSpec>;
#[doc = "Sets the value of triginack.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triginack {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Triginack> for bool {
    #[inline(always)]
    fn from(variant: Triginack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGINACK` reader - Sets the value of triginack."]
pub type TriginackR = crate::BitReader<Triginack>;
impl TriginackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triginack {
        match self.bits {
            false => Triginack::Low,
            true => Triginack::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Triginack::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Triginack::High
    }
}
#[doc = "Field `TRIGINACK` writer - Sets the value of triginack."]
pub type TriginackW<'a, REG> = crate::BitWriter<'a, REG, Triginack>;
impl<'a, REG> TriginackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Triginack::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Triginack::High)
    }
}
#[doc = "Sets the value of flushinack.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flushinack {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Flushinack> for bool {
    #[inline(always)]
    fn from(variant: Flushinack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHINACK` reader - Sets the value of flushinack."]
pub type FlushinackR = crate::BitReader<Flushinack>;
impl FlushinackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flushinack {
        match self.bits {
            false => Flushinack::Low,
            true => Flushinack::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Flushinack::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Flushinack::High
    }
}
#[doc = "Field `FLUSHINACK` writer - Sets the value of flushinack."]
pub type FlushinackW<'a, REG> = crate::BitWriter<'a, REG, Flushinack>;
impl<'a, REG> FlushinackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Flushinack::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Flushinack::High)
    }
}
impl R {
    #[doc = "Bit 0 - Sets the value of triginack."]
    #[inline(always)]
    pub fn triginack(&self) -> TriginackR {
        TriginackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the value of flushinack."]
    #[inline(always)]
    pub fn flushinack(&self) -> FlushinackR {
        FlushinackR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the value of triginack."]
    #[inline(always)]
    pub fn triginack(&mut self) -> TriginackW<'_, IttrflinackSpec> {
        TriginackW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the value of flushinack."]
    #[inline(always)]
    pub fn flushinack(&mut self) -> FlushinackW<'_, IttrflinackSpec> {
        FlushinackW::new(self, 1)
    }
}
#[doc = "The ITTRFLINACK register enables control of the triginack and flushinack outputs from the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflinack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflinack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrflinackSpec;
impl crate::RegisterSpec for IttrflinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrflinack::R`](R) reader structure"]
impl crate::Readable for IttrflinackSpec {}
#[doc = "`write(|w| ..)` method takes [`ittrflinack::W`](W) writer structure"]
impl crate::Writable for IttrflinackSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITTRFLINACK to value 0"]
impl crate::Resettable for IttrflinackSpec {}
