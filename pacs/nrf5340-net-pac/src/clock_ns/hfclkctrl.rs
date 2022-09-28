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
#[doc = "Field `HCLK` reader - High frequency clock HCLK"]
pub type HCLK_R = crate::FieldReader<u8, HCLK_A>;
#[doc = "High frequency clock HCLK\n\nValue on reset: 0"]
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
impl HCLK_R {
    #[doc = "Get enumerated values variant"]
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
        *self == HCLK_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HCLK_A::DIV2
    }
}
#[doc = "Field `HCLK` writer - High frequency clock HCLK"]
pub type HCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFCLKCTRL_SPEC, u8, HCLK_A, 2, O>;
impl<'a, const O: u8> HCLK_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&self) -> HCLK_R {
        HCLK_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&mut self) -> HCLK_W<0> {
        HCLK_W::new(self)
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
#[doc = "`reset()` method sets HFCLKCTRL to value 0"]
impl crate::Resettable for HFCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
