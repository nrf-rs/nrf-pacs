#[doc = "Register `ERRORSTATUS` reader"]
pub struct R(crate::R<ERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRORSTATUS` writer"]
pub struct W(crate::W<ERRORSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRORSTATUS_SPEC>;
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
impl From<crate::W<ERRORSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRORSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYTIMEOUT` reader - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub struct FRAMEDELAYTIMEOUT_R(crate::FieldReader<bool, bool>);
impl FRAMEDELAYTIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAMEDELAYTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEDELAYTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEDELAYTIMEOUT` writer - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub struct FRAMEDELAYTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYTIMEOUT_W<'a> {
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
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&self) -> FRAMEDELAYTIMEOUT_R {
        FRAMEDELAYTIMEOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&mut self) -> FRAMEDELAYTIMEOUT_W {
        FRAMEDELAYTIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorstatus](index.html) module"]
pub struct ERRORSTATUS_SPEC;
impl crate::RegisterSpec for ERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorstatus::R](R) reader structure"]
impl crate::Readable for ERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errorstatus::W](W) writer structure"]
impl crate::Writable for ERRORSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRORSTATUS to value 0"]
impl crate::Resettable for ERRORSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
