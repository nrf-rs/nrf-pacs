#[doc = "Register `WRITEUICRNS` writer"]
pub struct W(crate::W<WRITEUICRNS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITEUICRNS_SPEC>;
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
impl From<crate::W<WRITEUICRNS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITEUICRNS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allow non-secure code to set APPROTECT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_AW {
    #[doc = "1: Set value"]
    SET = 1,
}
impl From<SET_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET` writer - Allow non-secure code to set APPROTECT"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set value"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SET_AW::SET)
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
#[doc = "Key to write in order to validate the write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEY_AW {
    #[doc = "184280487: Key value"]
    KEYVALID = 184280487,
}
impl From<KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Key to write in order to validate the write operation"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value"]
    #[inline(always)]
    pub fn keyvalid(self) -> &'a mut W {
        self.variant(KEY_AW::KEYVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Allow non-secure code to set APPROTECT"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Bits 4:31 - Key to write in order to validate the write operation"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-secure APPROTECT enable register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writeuicrns](index.html) module"]
pub struct WRITEUICRNS_SPEC;
impl crate::RegisterSpec for WRITEUICRNS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writeuicrns::W](W) writer structure"]
impl crate::Writable for WRITEUICRNS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITEUICRNS to value 0"]
impl crate::Resettable for WRITEUICRNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
