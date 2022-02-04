#[doc = "Register `LFCLKSRC` reader"]
pub struct R(crate::R<LFCLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKSRC` writer"]
pub struct W(crate::W<LFCLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LFCLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: 32.768 kHz RC oscillator (LFRC)"]
    RC = 0,
    #[doc = "1: 32.768 kHz crystal oscillator (LFXO)"]
    XTAL = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Clock source"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::RC),
            1 => Some(SRC_A::XTAL),
            2 => Some(SRC_A::SYNTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        **self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        **self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        **self == SRC_A::SYNTH
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Clock source"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRC_A::RC)
    }
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRC_A::XTAL)
    }
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    #[inline(always)]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRC_A::SYNTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable (use with Xtal or low-swing external source)"]
    DISABLED = 0,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BYPASS_A::ENABLED
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Enable or disable external source for LFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNAL_A {
    #[doc = "0: Disable external source (use with Xtal)"]
    DISABLED = 0,
    #[doc = "1: Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED = 1,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTERNAL` reader - Enable or disable external source for LFCLK"]
pub struct EXTERNAL_R(crate::FieldReader<bool, EXTERNAL_A>);
impl EXTERNAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::DISABLED,
            true => EXTERNAL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTERNAL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EXTERNAL_A::ENABLED
    }
}
impl core::ops::Deref for EXTERNAL_R {
    type Target = crate::FieldReader<bool, EXTERNAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTERNAL` writer - Enable or disable external source for LFCLK"]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTERNAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable external source (use with Xtal)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::DISABLED)
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source for the LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrc](index.html) module"]
pub struct LFCLKSRC_SPEC;
impl crate::RegisterSpec for LFCLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksrc::R](R) reader structure"]
impl crate::Readable for LFCLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](W) writer structure"]
impl crate::Writable for LFCLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0"]
impl crate::Resettable for LFCLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
