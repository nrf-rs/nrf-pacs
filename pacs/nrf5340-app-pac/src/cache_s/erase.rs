#[doc = "Register `ERASE` writer"]
pub struct W(crate::W<ERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASE_SPEC>;
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
impl From<crate::W<ERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Erase the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASE_AW {
    #[doc = "1: Erase cache"]
    ERASE = 1,
}
impl From<ERASE_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` writer - Erase the cache"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Erase cache"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASE_AW::ERASE)
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
impl W {
    #[doc = "Bit 0 - Erase the cache"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase the cache.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase](index.html) module"]
pub struct ERASE_SPEC;
impl crate::RegisterSpec for ERASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [erase::W](W) writer structure"]
impl crate::Writable for ERASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASE to value 0"]
impl crate::Resettable for ERASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
