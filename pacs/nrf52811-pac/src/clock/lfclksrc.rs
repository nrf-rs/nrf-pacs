#[doc = "Register `LFCLKSRC` reader"]
pub type R = crate::R<LfclksrcSpec>;
#[doc = "Register `LFCLKSRC` writer"]
pub type W = crate::W<LfclksrcSpec>;
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: 32.768 kHz RC oscillator"]
    Rc = 0,
    #[doc = "1: 32.768 kHz crystal oscillator"]
    Xtal = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK"]
    Synth = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Rc),
            1 => Some(Src::Xtal),
            2 => Some(Src::Synth),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Src::Rc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Src::Xtal
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == Src::Synth
    }
}
#[doc = "Field `SRC` writer - Clock source"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn rc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Rc)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Xtal)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn synth(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Synth)
    }
}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Disable (use with Xtal or low-swing external source)"]
    Disabled = 0,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    Enabled = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Disabled,
            true => Bypass::Enabled,
        }
    }
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bypass::Disabled
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bypass::Enabled
    }
}
#[doc = "Field `BYPASS` writer - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Disabled)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Enabled)
    }
}
#[doc = "Enable or disable external source for LFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum External {
    #[doc = "0: Disable external source (use with Xtal)"]
    Disabled = 0,
    #[doc = "1: Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    Enabled = 1,
}
impl From<External> for bool {
    #[inline(always)]
    fn from(variant: External) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTERNAL` reader - Enable or disable external source for LFCLK"]
pub type ExternalR = crate::BitReader<External>;
impl ExternalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> External {
        match self.bits {
            false => External::Disabled,
            true => External::Enabled,
        }
    }
    #[doc = "Disable external source (use with Xtal)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == External::Disabled
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == External::Enabled
    }
}
#[doc = "Field `EXTERNAL` writer - Enable or disable external source for LFCLK"]
pub type ExternalW<'a, REG> = crate::BitWriter<'a, REG, External>;
impl<'a, REG> ExternalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable external source (use with Xtal)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(External::Disabled)
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(External::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&self) -> ExternalR {
        ExternalR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, LfclksrcSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, LfclksrcSpec> {
        BypassW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&mut self) -> ExternalW<'_, LfclksrcSpec> {
        ExternalW::new(self, 17)
    }
}
#[doc = "Clock source for the LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrcSpec;
impl crate::RegisterSpec for LfclksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrc::R`](R) reader structure"]
impl crate::Readable for LfclksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclksrc::W`](W) writer structure"]
impl crate::Writable for LfclksrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0"]
impl crate::Resettable for LfclksrcSpec {}
