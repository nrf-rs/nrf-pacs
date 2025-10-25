#[doc = "Register `GAINL` reader"]
pub type R = crate::R<GainlSpec>;
#[doc = "Register `GAINL` writer"]
pub type W = crate::W<GainlSpec>;
#[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainl {
    #[doc = "0: -20 dB gain adjustment (minimum)"]
    MinGain = 0,
    #[doc = "40: 0 dB gain adjustment"]
    DefaultGain = 40,
    #[doc = "80: +20 dB gain adjustment (maximum)"]
    MaxGain = 80,
}
impl From<Gainl> for u8 {
    #[inline(always)]
    fn from(variant: Gainl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainl {
    type Ux = u8;
}
impl crate::IsEnum for Gainl {}
#[doc = "Field `GAINL` reader - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub type GainlR = crate::FieldReader<Gainl>;
impl GainlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gainl> {
        match self.bits {
            0 => Some(Gainl::MinGain),
            40 => Some(Gainl::DefaultGain),
            80 => Some(Gainl::MaxGain),
            _ => None,
        }
    }
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == Gainl::MinGain
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == Gainl::DefaultGain
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == Gainl::MaxGain
    }
}
#[doc = "Field `GAINL` writer - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub type GainlW<'a, REG> = crate::FieldWriter<'a, REG, 7, Gainl>;
impl<'a, REG> GainlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainl::MinGain)
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainl::DefaultGain)
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Gainl::MaxGain)
    }
}
impl R {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&self) -> GainlR {
        GainlR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&mut self) -> GainlW<'_, GainlSpec> {
        GainlW::new(self, 0)
    }
}
#[doc = "Left output gain adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`gainl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gainl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GainlSpec;
impl crate::RegisterSpec for GainlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gainl::R`](R) reader structure"]
impl crate::Readable for GainlSpec {}
#[doc = "`write(|w| ..)` method takes [`gainl::W`](W) writer structure"]
impl crate::Writable for GainlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAINL to value 0x28"]
impl crate::Resettable for GainlSpec {
    const RESET_VALUE: u32 = 0x28;
}
