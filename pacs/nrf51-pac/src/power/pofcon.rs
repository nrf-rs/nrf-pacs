#[doc = "Register `POFCON` reader"]
pub struct R(crate::R<POFCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POFCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POFCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POFCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POFCON` writer"]
pub struct W(crate::W<POFCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POFCON_SPEC>;
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
impl From<crate::W<POFCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POFCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power failure comparator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POF_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Power failure comparator enable."]
pub struct POF_R(crate::FieldReader<bool, POF_A>);
impl POF_R {
    pub(crate) fn new(bits: bool) -> Self {
        POF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == POF_A::ENABLED
    }
}
impl core::ops::Deref for POF_R {
    type Target = crate::FieldReader<bool, POF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POF` writer - Power failure comparator enable."]
pub struct POF_W<'a> {
    w: &'a mut W,
}
impl<'a> POF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Set threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "0: Set threshold to 2.1Volts."]
    V21 = 0,
    #[doc = "1: Set threshold to 2.3Volts."]
    V23 = 1,
    #[doc = "2: Set threshold to 2.5Volts."]
    V25 = 2,
    #[doc = "3: Set threshold to 2.7Volts."]
    V27 = 3,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `THRESHOLD` reader - Set threshold level."]
pub struct THRESHOLD_R(crate::FieldReader<u8, THRESHOLD_A>);
impl THRESHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        THRESHOLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLD_A {
        match self.bits {
            0 => THRESHOLD_A::V21,
            1 => THRESHOLD_A::V23,
            2 => THRESHOLD_A::V25,
            3 => THRESHOLD_A::V27,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        **self == THRESHOLD_A::V21
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        **self == THRESHOLD_A::V23
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        **self == THRESHOLD_A::V25
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        **self == THRESHOLD_A::V27
    }
}
impl core::ops::Deref for THRESHOLD_R {
    type Target = crate::FieldReader<u8, THRESHOLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESHOLD` writer - Set threshold level."]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set threshold to 2.1Volts."]
    #[inline(always)]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V21)
    }
    #[doc = "Set threshold to 2.3Volts."]
    #[inline(always)]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V23)
    }
    #[doc = "Set threshold to 2.5Volts."]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V25)
    }
    #[doc = "Set threshold to 2.7Volts."]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V27)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&mut self) -> POF_W {
        POF_W { w: self }
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power failure configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pofcon](index.html) module"]
pub struct POFCON_SPEC;
impl crate::RegisterSpec for POFCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pofcon::R](R) reader structure"]
impl crate::Readable for POFCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pofcon::W](W) writer structure"]
impl crate::Writable for POFCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for POFCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
