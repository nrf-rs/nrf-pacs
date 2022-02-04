#[doc = "Register `DECODER` reader"]
pub struct R(crate::R<DECODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODER` writer"]
pub struct W(crate::W<DECODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODER_SPEC>;
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
impl From<crate::W<DECODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "How a sequence is read from RAM and spread to the compare register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOAD_A {
    #[doc = "0: 1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON = 0,
    #[doc = "1: 1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED = 1,
    #[doc = "2: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL = 2,
    #[doc = "3: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM = 3,
}
impl From<LOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOAD` reader - How a sequence is read from RAM and spread to the compare register"]
pub struct LOAD_R(crate::FieldReader<u8, LOAD_A>);
impl LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            0 => LOAD_A::COMMON,
            1 => LOAD_A::GROUPED,
            2 => LOAD_A::INDIVIDUAL,
            3 => LOAD_A::WAVEFORM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMMON`"]
    #[inline(always)]
    pub fn is_common(&self) -> bool {
        **self == LOAD_A::COMMON
    }
    #[doc = "Checks if the value of the field is `GROUPED`"]
    #[inline(always)]
    pub fn is_grouped(&self) -> bool {
        **self == LOAD_A::GROUPED
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        **self == LOAD_A::INDIVIDUAL
    }
    #[doc = "Checks if the value of the field is `WAVEFORM`"]
    #[inline(always)]
    pub fn is_wave_form(&self) -> bool {
        **self == LOAD_A::WAVEFORM
    }
}
impl core::ops::Deref for LOAD_R {
    type Target = crate::FieldReader<u8, LOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` writer - How a sequence is read from RAM and spread to the compare register"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOAD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    #[inline(always)]
    pub fn common(self) -> &'a mut W {
        self.variant(LOAD_A::COMMON)
    }
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    #[inline(always)]
    pub fn grouped(self) -> &'a mut W {
        self.variant(LOAD_A::GROUPED)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut W {
        self.variant(LOAD_A::INDIVIDUAL)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    #[inline(always)]
    pub fn wave_form(self) -> &'a mut W {
        self.variant(LOAD_A::WAVEFORM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Selects source for advancing the active sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT = 0,
    #[doc = "1: NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Selects source for advancing the active sequence"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::REFRESHCOUNT,
            true => MODE_A::NEXTSTEP,
        }
    }
    #[doc = "Checks if the value of the field is `REFRESHCOUNT`"]
    #[inline(always)]
    pub fn is_refresh_count(&self) -> bool {
        **self == MODE_A::REFRESHCOUNT
    }
    #[doc = "Checks if the value of the field is `NEXTSTEP`"]
    #[inline(always)]
    pub fn is_next_step(&self) -> bool {
        **self == MODE_A::NEXTSTEP
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Selects source for advancing the active sequence"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    #[inline(always)]
    pub fn refresh_count(self) -> &'a mut W {
        self.variant(MODE_A::REFRESHCOUNT)
    }
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    #[inline(always)]
    pub fn next_step(self) -> &'a mut W {
        self.variant(MODE_A::NEXTSTEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of the decoder\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decoder](index.html) module"]
pub struct DECODER_SPEC;
impl crate::RegisterSpec for DECODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decoder::R](R) reader structure"]
impl crate::Readable for DECODER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decoder::W](W) writer structure"]
impl crate::Writable for DECODER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DECODER to value 0"]
impl crate::Resettable for DECODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
