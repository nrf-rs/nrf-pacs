#[doc = "Register `HFCLKCTRL` reader"]
pub struct R(crate::R<HFCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFCLKCTRL` writer"]
pub struct W(crate::W<HFCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCLKCTRL_SPEC>;
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
impl From<crate::W<HFCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "High frequency clock HCLK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCLK_A {
    #[doc = "0: Divide HFCLK by 1"]
    DIV1 = 0,
    #[doc = "1: Divide HFCLK by 2"]
    DIV2 = 1,
}
impl From<HCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HCLK` reader - High frequency clock HCLK"]
pub struct HCLK_R(crate::FieldReader<u8, HCLK_A>);
impl HCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HCLK_A> {
        match self.bits {
            0 => Some(HCLK_A::DIV1),
            1 => Some(HCLK_A::DIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HCLK_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HCLK_A::DIV2
    }
}
impl core::ops::Deref for HCLK_R {
    type Target = crate::FieldReader<u8, HCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLK` writer - High frequency clock HCLK"]
pub struct HCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide HFCLK by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HCLK_A::DIV1)
    }
    #[doc = "Divide HFCLK by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HCLK_A::DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&self) -> HCLK_R {
        HCLK_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&mut self) -> HCLK_W {
        HCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFCLK128M frequency configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkctrl](index.html) module"]
pub struct HFCLKCTRL_SPEC;
impl crate::RegisterSpec for HFCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkctrl::R](R) reader structure"]
impl crate::Readable for HFCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfclkctrl::W](W) writer structure"]
impl crate::Writable for HFCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFCLKCTRL to value 0x01"]
impl crate::Resettable for HFCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
