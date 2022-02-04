#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => OVERREAD_A::NOTPRESENT,
            true => OVERREAD_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == OVERREAD_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == OVERREAD_A::PRESENT
    }
}
impl core::ops::Deref for OVERREAD_R {
    type Target = crate::FieldReader<bool, OVERREAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERREAD_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub struct OVERREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERREAD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERREAD_AW::CLEAR)
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
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => OVERFLOW_A::NOTPRESENT,
            true => OVERFLOW_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == OVERFLOW_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == OVERFLOW_A::PRESENT
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, OVERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OVERREAD_W {
        OVERREAD_W { w: self }
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status from last transaction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
