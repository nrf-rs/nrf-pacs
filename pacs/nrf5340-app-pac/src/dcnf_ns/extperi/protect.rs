#[doc = "Register `PROTECT` reader"]
pub struct R(crate::R<PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECT` writer"]
pub struct W(crate::W<PROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECT_SPEC>;
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
impl From<crate::W<PROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control access to slave 0 of master EXTPERI\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE0_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE0` reader - Control access to slave 0 of master EXTPERI\\[n\\]"]
pub struct SLAVE0_R(crate::FieldReader<bool, SLAVE0_A>);
impl SLAVE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::ALLOWED,
            true => SLAVE0_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == SLAVE0_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == SLAVE0_A::BLOCKED
    }
}
impl core::ops::Deref for SLAVE0_R {
    type Target = crate::FieldReader<bool, SLAVE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE0` writer - Control access to slave 0 of master EXTPERI\\[n\\]"]
pub struct SLAVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE0_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE0_A::BLOCKED)
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
    #[doc = "Bit 0 - Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> SLAVE0_W {
        SLAVE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protect](index.html) module"]
pub struct PROTECT_SPEC;
impl crate::RegisterSpec for PROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protect::R](R) reader structure"]
impl crate::Readable for PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protect::W](W) writer structure"]
impl crate::Writable for PROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for PROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
