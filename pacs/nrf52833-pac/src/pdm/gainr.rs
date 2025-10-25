#[doc = "Register `GAINR` reader"]
pub type R = crate::R<GainrSpec>;
#[doc = "Register `GAINR` writer"]
pub type W = crate::W<GainrSpec>;
#[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainr {
    #[doc = "0: -20 dB gain adjustment (minimum)"]
    MinGain = 0,
    #[doc = "40: 0 dB gain adjustment"]
    DefaultGain = 40,
    #[doc = "80: +20 dB gain adjustment (maximum)"]
    MaxGain = 80,
}
impl From<Gainr> for u8 {
    #[inline(always)]
    fn from(variant: Gainr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainr {
    type Ux = u8;
}
impl crate::IsEnum for Gainr {}
#[doc = "Field `GAINR` reader - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub type GainrR = crate::FieldReader<Gainr>;
impl GainrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gainr> {
        match self.bits {
            0 => Some(Gainr::MinGain),
            40 => Some(Gainr::DefaultGain),
            80 => Some(Gainr::MaxGain),
            _ => None,
        }
    }
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == Gainr::MinGain
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == Gainr::DefaultGain
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == Gainr::MaxGain
    }
}
#[doc = "Field `GAINR` writer - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub type GainrW<'a, REG> = crate::FieldWriter<'a, REG, 7, Gainr>;
impl<'a, REG> GainrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainr::MinGain)
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainr::DefaultGain)
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainr::MaxGain)
    }
}
impl R {
    #[doc = "Bits 0:6 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&self) -> GainrR {
        GainrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&mut self) -> GainrW<'_, GainrSpec> {
        GainrW::new(self, 0)
    }
}
#[doc = "Right output gain adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`gainr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gainr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GainrSpec;
impl crate::RegisterSpec for GainrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gainr::R`](R) reader structure"]
impl crate::Readable for GainrSpec {}
#[doc = "`write(|w| ..)` method takes [`gainr::W`](W) writer structure"]
impl crate::Writable for GainrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAINR to value 0x28"]
impl crate::Resettable for GainrSpec {
    const RESET_VALUE: u32 = 0x28;
}
