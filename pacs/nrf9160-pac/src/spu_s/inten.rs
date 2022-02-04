#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RAMACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: RAMACCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMACCERR` reader - Enable or disable interrupt for event RAMACCERR"]
pub struct RAMACCERR_R(crate::FieldReader<bool, RAMACCERR_A>);
impl RAMACCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMACCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMACCERR_A {
        match self.bits {
            false => RAMACCERR_A::DISABLED,
            true => RAMACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RAMACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RAMACCERR_A::ENABLED
    }
}
impl core::ops::Deref for RAMACCERR_R {
    type Target = crate::FieldReader<bool, RAMACCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACCERR` writer - Enable or disable interrupt for event RAMACCERR"]
pub struct RAMACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMACCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAMACCERR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAMACCERR_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FLASHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHACCERR` reader - Enable or disable interrupt for event FLASHACCERR"]
pub struct FLASHACCERR_R(crate::FieldReader<bool, FLASHACCERR_A>);
impl FLASHACCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASHACCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHACCERR_A {
        match self.bits {
            false => FLASHACCERR_A::DISABLED,
            true => FLASHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLASHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FLASHACCERR_A::ENABLED
    }
}
impl core::ops::Deref for FLASHACCERR_R {
    type Target = crate::FieldReader<bool, FLASHACCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHACCERR` writer - Enable or disable interrupt for event FLASHACCERR"]
pub struct FLASHACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHACCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHACCERR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHACCERR_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PERIPHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHACCERR` reader - Enable or disable interrupt for event PERIPHACCERR"]
pub struct PERIPHACCERR_R(crate::FieldReader<bool, PERIPHACCERR_A>);
impl PERIPHACCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERIPHACCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHACCERR_A {
        match self.bits {
            false => PERIPHACCERR_A::DISABLED,
            true => PERIPHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PERIPHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PERIPHACCERR_A::ENABLED
    }
}
impl core::ops::Deref for PERIPHACCERR_R {
    type Target = crate::FieldReader<bool, PERIPHACCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIPHACCERR` writer - Enable or disable interrupt for event PERIPHACCERR"]
pub struct PERIPHACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPHACCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHACCERR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHACCERR_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&self) -> RAMACCERR_R {
        RAMACCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&self) -> FLASHACCERR_R {
        FLASHACCERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&self) -> PERIPHACCERR_R {
        PERIPHACCERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&mut self) -> RAMACCERR_W {
        RAMACCERR_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&mut self) -> FLASHACCERR_W {
        FLASHACCERR_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&mut self) -> PERIPHACCERR_W {
        PERIPHACCERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
