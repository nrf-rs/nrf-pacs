#[doc = "Register `CTIGATE` reader"]
pub struct R(crate::R<CTIGATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIGATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIGATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIGATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIGATE` writer"]
pub struct W(crate::W<CTIGATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIGATE_SPEC>;
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
impl From<crate::W<CTIGATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIGATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable ctichout0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_0_A {
    #[doc = "1: Enable ctichout channel 0 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 0 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_0` reader - Enable ctichout0."]
pub struct CTIGATEEN_0_R(crate::FieldReader<bool, CTIGATEEN_0_A>);
impl CTIGATEEN_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIGATEEN_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_0_A {
        match self.bits {
            true => CTIGATEEN_0_A::ENABLED,
            false => CTIGATEEN_0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTIGATEEN_0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTIGATEEN_0_A::DISABLED
    }
}
impl core::ops::Deref for CTIGATEEN_0_R {
    type Target = crate::FieldReader<bool, CTIGATEEN_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIGATEEN_0` writer - Enable ctichout0."]
pub struct CTIGATEEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::DISABLED)
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
#[doc = "Enable ctichout1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_1_A {
    #[doc = "1: Enable ctichout channel 1 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 1 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_1` reader - Enable ctichout1."]
pub struct CTIGATEEN_1_R(crate::FieldReader<bool, CTIGATEEN_1_A>);
impl CTIGATEEN_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIGATEEN_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_1_A {
        match self.bits {
            true => CTIGATEEN_1_A::ENABLED,
            false => CTIGATEEN_1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTIGATEEN_1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTIGATEEN_1_A::DISABLED
    }
}
impl core::ops::Deref for CTIGATEEN_1_R {
    type Target = crate::FieldReader<bool, CTIGATEEN_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIGATEEN_1` writer - Enable ctichout1."]
pub struct CTIGATEEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable ctichout2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_2_A {
    #[doc = "1: Enable ctichout channel 2 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 2 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_2` reader - Enable ctichout2."]
pub struct CTIGATEEN_2_R(crate::FieldReader<bool, CTIGATEEN_2_A>);
impl CTIGATEEN_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIGATEEN_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_2_A {
        match self.bits {
            true => CTIGATEEN_2_A::ENABLED,
            false => CTIGATEEN_2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTIGATEEN_2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTIGATEEN_2_A::DISABLED
    }
}
impl core::ops::Deref for CTIGATEEN_2_R {
    type Target = crate::FieldReader<bool, CTIGATEEN_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIGATEEN_2` writer - Enable ctichout2."]
pub struct CTIGATEEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable ctichout3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_3_A {
    #[doc = "1: Enable ctichout channel 3 propagation."]
    ENABLED = 1,
    #[doc = "0: Disable ctichout channel 3 propagation."]
    DISABLED = 0,
}
impl From<CTIGATEEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_3` reader - Enable ctichout3."]
pub struct CTIGATEEN_3_R(crate::FieldReader<bool, CTIGATEEN_3_A>);
impl CTIGATEEN_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIGATEEN_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_3_A {
        match self.bits {
            true => CTIGATEEN_3_A::ENABLED,
            false => CTIGATEEN_3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTIGATEEN_3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTIGATEEN_3_A::DISABLED
    }
}
impl core::ops::Deref for CTIGATEEN_3_R {
    type Target = crate::FieldReader<bool, CTIGATEEN_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIGATEEN_3` writer - Enable ctichout3."]
pub struct CTIGATEEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&self) -> CTIGATEEN_0_R {
        CTIGATEEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&self) -> CTIGATEEN_1_R {
        CTIGATEEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&self) -> CTIGATEEN_2_R {
        CTIGATEEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&self) -> CTIGATEEN_3_R {
        CTIGATEEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&mut self) -> CTIGATEEN_0_W {
        CTIGATEEN_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&mut self) -> CTIGATEEN_1_W {
        CTIGATEEN_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&mut self) -> CTIGATEEN_2_W {
        CTIGATEEN_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&mut self) -> CTIGATEEN_3_W {
        CTIGATEEN_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable CTI Channel Gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctigate](index.html) module"]
pub struct CTIGATE_SPEC;
impl crate::RegisterSpec for CTIGATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctigate::R](R) reader structure"]
impl crate::Readable for CTIGATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctigate::W](W) writer structure"]
impl crate::Writable for CTIGATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIGATE to value 0x0f"]
impl crate::Resettable for CTIGATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
