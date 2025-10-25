#[doc = "Register `ITTRFLIN` reader"]
pub type R = crate::R<IttrflinSpec>;
#[doc = "Register `ITTRFLIN` writer"]
pub type W = crate::W<IttrflinSpec>;
#[doc = "Reads the value of trigin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigin {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Trigin> for bool {
    #[inline(always)]
    fn from(variant: Trigin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGIN` reader - Reads the value of trigin."]
pub type TriginR = crate::BitReader<Trigin>;
impl TriginR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigin {
        match self.bits {
            false => Trigin::Low,
            true => Trigin::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Trigin::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Trigin::High
    }
}
#[doc = "Field `TRIGIN` writer - Reads the value of trigin."]
pub type TriginW<'a, REG> = crate::BitWriter<'a, REG, Trigin>;
impl<'a, REG> TriginW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Trigin::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Trigin::High)
    }
}
#[doc = "Reads the value of flushin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flushin {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Flushin> for bool {
    #[inline(always)]
    fn from(variant: Flushin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHIN` reader - Reads the value of flushin."]
pub type FlushinR = crate::BitReader<Flushin>;
impl FlushinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flushin {
        match self.bits {
            false => Flushin::Low,
            true => Flushin::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Flushin::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Flushin::High
    }
}
#[doc = "Field `FLUSHIN` writer - Reads the value of flushin."]
pub type FlushinW<'a, REG> = crate::BitWriter<'a, REG, Flushin>;
impl<'a, REG> FlushinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Flushin::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Flushin::High)
    }
}
impl R {
    #[doc = "Bit 0 - Reads the value of trigin."]
    #[inline(always)]
    pub fn trigin(&self) -> TriginR {
        TriginR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reads the value of flushin."]
    #[inline(always)]
    pub fn flushin(&self) -> FlushinR {
        FlushinR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reads the value of trigin."]
    #[inline(always)]
    pub fn trigin(&mut self) -> TriginW<'_, IttrflinSpec> {
        TriginW::new(self, 0)
    }
    #[doc = "Bit 1 - Reads the value of flushin."]
    #[inline(always)]
    pub fn flushin(&mut self) -> FlushinW<'_, IttrflinSpec> {
        FlushinW::new(self, 1)
    }
}
#[doc = "The ITTRFLIN register contains the values of the flushin and trigin inputs to the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrflinSpec;
impl crate::RegisterSpec for IttrflinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrflin::R`](R) reader structure"]
impl crate::Readable for IttrflinSpec {}
#[doc = "`write(|w| ..)` method takes [`ittrflin::W`](W) writer structure"]
impl crate::Writable for IttrflinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITTRFLIN to value 0"]
impl crate::Resettable for IttrflinSpec {}
