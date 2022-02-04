#[doc = "Register `ERASEALL` reader"]
pub struct R(crate::R<ERASEALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEALL` writer"]
pub struct W(crate::W<ERASEALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEALL_SPEC>;
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
impl From<crate::W<ERASEALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Starts the erasing of all user NVM (code region 0/1 and UICR registers).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEALL_A {
    #[doc = "0: No operation."]
    NOOPERATION = 0,
    #[doc = "1: Start chip erase."]
    ERASE = 1,
}
impl From<ERASEALL_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEALL` reader - Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
pub struct ERASEALL_R(crate::FieldReader<bool, ERASEALL_A>);
impl ERASEALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERASEALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEALL_A {
        match self.bits {
            false => ERASEALL_A::NOOPERATION,
            true => ERASEALL_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NOOPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        **self == ERASEALL_A::NOOPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == ERASEALL_A::ERASE
    }
}
impl core::ops::Deref for ERASEALL_R {
    type Target = crate::FieldReader<bool, ERASEALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASEALL` writer - Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
pub struct ERASEALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEALL_A::NOOPERATION)
    }
    #[doc = "Start chip erase."]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEALL_A::ERASE)
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
impl R {
    #[doc = "Bit 0 - Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
    #[inline(always)]
    pub fn eraseall(&self) -> ERASEALL_R {
        ERASEALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> ERASEALL_W {
        ERASEALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing all non-volatile user memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseall](index.html) module"]
pub struct ERASEALL_SPEC;
impl crate::RegisterSpec for ERASEALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eraseall::R](R) reader structure"]
impl crate::Readable for ERASEALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eraseall::W](W) writer structure"]
impl crate::Writable for ERASEALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for ERASEALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
