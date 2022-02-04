#[doc = "Register `DCDCFORCE` reader"]
pub struct R(crate::R<DCDCFORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCFORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCFORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCFORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCFORCE` writer"]
pub struct W(crate::W<DCDCFORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCFORCE_SPEC>;
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
impl From<crate::W<DCDCFORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCFORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DCDC power-up force off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFF_A {
    #[doc = "0: No force."]
    NOFORCE = 0,
    #[doc = "1: Force."]
    FORCE = 1,
}
impl From<FORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEOFF` reader - DCDC power-up force off."]
pub struct FORCEOFF_R(crate::FieldReader<bool, FORCEOFF_A>);
impl FORCEOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCEOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFF_A {
        match self.bits {
            false => FORCEOFF_A::NOFORCE,
            true => FORCEOFF_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        **self == FORCEOFF_A::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        **self == FORCEOFF_A::FORCE
    }
}
impl core::ops::Deref for FORCEOFF_R {
    type Target = crate::FieldReader<bool, FORCEOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEOFF` writer - DCDC power-up force off."]
pub struct FORCEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::NOFORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::FORCE)
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
#[doc = "DCDC power-up force on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEON_A {
    #[doc = "0: No force."]
    NOFORCE = 0,
    #[doc = "1: Force."]
    FORCE = 1,
}
impl From<FORCEON_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEON` reader - DCDC power-up force on."]
pub struct FORCEON_R(crate::FieldReader<bool, FORCEON_A>);
impl FORCEON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCEON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEON_A {
        match self.bits {
            false => FORCEON_A::NOFORCE,
            true => FORCEON_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        **self == FORCEON_A::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        **self == FORCEON_A::FORCE
    }
}
impl core::ops::Deref for FORCEON_R {
    type Target = crate::FieldReader<bool, FORCEON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEON` writer - DCDC power-up force on."]
pub struct FORCEON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEON_A::NOFORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEON_A::FORCE)
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
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&self) -> FORCEOFF_R {
        FORCEOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&self) -> FORCEON_R {
        FORCEON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&mut self) -> FORCEOFF_W {
        FORCEOFF_W { w: self }
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&mut self) -> FORCEON_W {
        FORCEON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC power-up force register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcforce](index.html) module"]
pub struct DCDCFORCE_SPEC;
impl crate::RegisterSpec for DCDCFORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcforce::R](R) reader structure"]
impl crate::Readable for DCDCFORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcforce::W](W) writer structure"]
impl crate::Writable for DCDCFORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCFORCE to value 0"]
impl crate::Resettable for DCDCFORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
