#[doc = "Register `ERRORSRC` reader"]
pub struct R(crate::R<ERRORSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRORSRC` writer"]
pub struct W(crate::W<ERRORSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRORSRC_SPEC>;
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
impl From<crate::W<ERRORSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRORSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Error did not occur"]
    NOTDETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - RX buffer overflow detected, and prevented"]
pub struct OVERFLOW_R(crate::FieldReader<bool, OVERFLOW_A>);
impl OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::NOTDETECTED,
            true => OVERFLOW_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == OVERFLOW_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == OVERFLOW_A::DETECTED
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, OVERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::DETECTED)
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
#[doc = "NACK sent after receiving a data byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACK_A {
    #[doc = "0: Error did not occur"]
    NOTRECEIVED = 0,
    #[doc = "1: Error occurred"]
    RECEIVED = 1,
}
impl From<DNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - NACK sent after receiving a data byte"]
pub struct DNACK_R(crate::FieldReader<bool, DNACK_A>);
impl DNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNACK_A {
        match self.bits {
            false => DNACK_A::NOTRECEIVED,
            true => DNACK_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        **self == DNACK_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        **self == DNACK_A::RECEIVED
    }
}
impl core::ops::Deref for DNACK_R {
    type Target = crate::FieldReader<bool, DNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNACK` writer - NACK sent after receiving a data byte"]
pub struct DNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut W {
        self.variant(DNACK_A::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(DNACK_A::RECEIVED)
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
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_A {
    #[doc = "0: Error did not occur"]
    NOTDETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
}
impl From<OVERREAD_A> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERREAD` reader - TX buffer over-read detected, and prevented"]
pub struct OVERREAD_R(crate::FieldReader<bool, OVERREAD_A>);
impl OVERREAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERREAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERREAD_A {
        match self.bits {
            false => OVERREAD_A::NOTDETECTED,
            true => OVERREAD_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == OVERREAD_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == OVERREAD_A::DETECTED
    }
}
impl core::ops::Deref for OVERREAD_R {
    type Target = crate::FieldReader<bool, OVERREAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub struct OVERREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERREAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::DETECTED)
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
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DNACK_W {
        DNACK_W { w: self }
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OVERREAD_W {
        OVERREAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](index.html) module"]
pub struct ERRORSRC_SPEC;
impl crate::RegisterSpec for ERRORSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorsrc::R](R) reader structure"]
impl crate::Readable for ERRORSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](W) writer structure"]
impl crate::Writable for ERRORSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ERRORSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
