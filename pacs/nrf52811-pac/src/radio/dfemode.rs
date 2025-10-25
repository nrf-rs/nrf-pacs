#[doc = "Register `DFEMODE` reader"]
pub type R = crate::R<DfemodeSpec>;
#[doc = "Register `DFEMODE` writer"]
pub type W = crate::W<DfemodeSpec>;
#[doc = "Direction finding operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfeopmode {
    #[doc = "0: Direction finding mode disabled"]
    Disabled = 0,
    #[doc = "2: Direction finding mode set to AoD"]
    AoD = 2,
    #[doc = "3: Direction finding mode set to AoA"]
    AoA = 3,
}
impl From<Dfeopmode> for u8 {
    #[inline(always)]
    fn from(variant: Dfeopmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfeopmode {
    type Ux = u8;
}
impl crate::IsEnum for Dfeopmode {}
#[doc = "Field `DFEOPMODE` reader - Direction finding operation mode"]
pub type DfeopmodeR = crate::FieldReader<Dfeopmode>;
impl DfeopmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dfeopmode> {
        match self.bits {
            0 => Some(Dfeopmode::Disabled),
            2 => Some(Dfeopmode::AoD),
            3 => Some(Dfeopmode::AoA),
            _ => None,
        }
    }
    #[doc = "Direction finding mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dfeopmode::Disabled
    }
    #[doc = "Direction finding mode set to AoD"]
    #[inline(always)]
    pub fn is_ao_d(&self) -> bool {
        *self == Dfeopmode::AoD
    }
    #[doc = "Direction finding mode set to AoA"]
    #[inline(always)]
    pub fn is_ao_a(&self) -> bool {
        *self == Dfeopmode::AoA
    }
}
#[doc = "Field `DFEOPMODE` writer - Direction finding operation mode"]
pub type DfeopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dfeopmode>;
impl<'a, REG> DfeopmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direction finding mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dfeopmode::Disabled)
    }
    #[doc = "Direction finding mode set to AoD"]
    #[inline(always)]
    pub fn ao_d(self) -> &'a mut crate::W<REG> {
        self.variant(Dfeopmode::AoD)
    }
    #[doc = "Direction finding mode set to AoA"]
    #[inline(always)]
    pub fn ao_a(self) -> &'a mut crate::W<REG> {
        self.variant(Dfeopmode::AoA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&self) -> DfeopmodeR {
        DfeopmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&mut self) -> DfeopmodeW<'_, DfemodeSpec> {
        DfeopmodeW::new(self, 0)
    }
}
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dfemode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfemode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfemodeSpec;
impl crate::RegisterSpec for DfemodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfemode::R`](R) reader structure"]
impl crate::Readable for DfemodeSpec {}
#[doc = "`write(|w| ..)` method takes [`dfemode::W`](W) writer structure"]
impl crate::Writable for DfemodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFEMODE to value 0"]
impl crate::Resettable for DfemodeSpec {}
