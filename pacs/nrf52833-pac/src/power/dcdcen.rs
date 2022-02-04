#[doc = "Register `DCDCEN` reader"]
pub struct R(crate::R<DCDCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCEN` writer"]
pub struct W(crate::W<DCDCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCEN_SPEC>;
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
impl From<crate::W<DCDCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable DC/DC converter for REG1 stage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDCEN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<DCDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDCEN` reader - Enable DC/DC converter for REG1 stage."]
pub struct DCDCEN_R(crate::FieldReader<bool, DCDCEN_A>);
impl DCDCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCEN_A {
        match self.bits {
            false => DCDCEN_A::DISABLED,
            true => DCDCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCDCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCDCEN_A::ENABLED
    }
}
impl core::ops::Deref for DCDCEN_R {
    type Target = crate::FieldReader<bool, DCDCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDCEN` writer - Enable DC/DC converter for REG1 stage."]
pub struct DCDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::ENABLED)
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
    #[doc = "Bit 0 - Enable DC/DC converter for REG1 stage."]
    #[inline(always)]
    pub fn dcdcen(&self) -> DCDCEN_R {
        DCDCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC/DC converter for REG1 stage."]
    #[inline(always)]
    pub fn dcdcen(&mut self) -> DCDCEN_W {
        DCDCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DC/DC converter for REG1 stage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcen](index.html) module"]
pub struct DCDCEN_SPEC;
impl crate::RegisterSpec for DCDCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcen::R](R) reader structure"]
impl crate::Readable for DCDCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcen::W](W) writer structure"]
impl crate::Writable for DCDCEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCEN to value 0"]
impl crate::Resettable for DCDCEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
