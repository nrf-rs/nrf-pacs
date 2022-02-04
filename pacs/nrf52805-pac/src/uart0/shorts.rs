#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between event CTS and task STARTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CTS_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_STARTRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS_STARTRX` reader - Shortcut between event CTS and task STARTRX"]
pub struct CTS_STARTRX_R(crate::FieldReader<bool, CTS_STARTRX_A>);
impl CTS_STARTRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_STARTRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_STARTRX_A {
        match self.bits {
            false => CTS_STARTRX_A::DISABLED,
            true => CTS_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTS_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTS_STARTRX_A::ENABLED
    }
}
impl core::ops::Deref for CTS_STARTRX_R {
    type Target = crate::FieldReader<bool, CTS_STARTRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_STARTRX` writer - Shortcut between event CTS and task STARTRX"]
pub struct CTS_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_STARTRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTS_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTS_STARTRX_A::ENABLED)
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
#[doc = "Shortcut between event NCTS and task STOPRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_STOPRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<NCTS_STOPRX_A> for bool {
    #[inline(always)]
    fn from(variant: NCTS_STOPRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS_STOPRX` reader - Shortcut between event NCTS and task STOPRX"]
pub struct NCTS_STOPRX_R(crate::FieldReader<bool, NCTS_STOPRX_A>);
impl NCTS_STOPRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NCTS_STOPRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCTS_STOPRX_A {
        match self.bits {
            false => NCTS_STOPRX_A::DISABLED,
            true => NCTS_STOPRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NCTS_STOPRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NCTS_STOPRX_A::ENABLED
    }
}
impl core::ops::Deref for NCTS_STOPRX_R {
    type Target = crate::FieldReader<bool, NCTS_STOPRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCTS_STOPRX` writer - Shortcut between event NCTS and task STOPRX"]
pub struct NCTS_STOPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> NCTS_STOPRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCTS_STOPRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NCTS_STOPRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NCTS_STOPRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Shortcut between event CTS and task STARTRX"]
    #[inline(always)]
    pub fn cts_startrx(&self) -> CTS_STARTRX_R {
        CTS_STARTRX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event NCTS and task STOPRX"]
    #[inline(always)]
    pub fn ncts_stoprx(&self) -> NCTS_STOPRX_R {
        NCTS_STOPRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shortcut between event CTS and task STARTRX"]
    #[inline(always)]
    pub fn cts_startrx(&mut self) -> CTS_STARTRX_W {
        CTS_STARTRX_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event NCTS and task STOPRX"]
    #[inline(always)]
    pub fn ncts_stoprx(&mut self) -> NCTS_STOPRX_W {
        NCTS_STOPRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
