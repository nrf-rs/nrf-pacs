#[doc = "Register `HFCLK192MCTRL` reader"]
pub struct R(crate::R<HFCLK192MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLK192MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLK192MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLK192MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFCLK192MCTRL` writer"]
pub struct W(crate::W<HFCLK192MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCLK192MCTRL_SPEC>;
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
impl From<crate::W<HFCLK192MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFCLK192MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "High frequency clock HCLK192M\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCLK192M_A {
    #[doc = "0: Divide HFCLK192M by 1"]
    DIV1 = 0,
    #[doc = "1: Divide HFCLK192M by 2"]
    DIV2 = 1,
    #[doc = "2: Divide HFCLK192M by 4"]
    DIV4 = 2,
}
impl From<HCLK192M_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLK192M_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HCLK192M` reader - High frequency clock HCLK192M"]
pub struct HCLK192M_R(crate::FieldReader<u8, HCLK192M_A>);
impl HCLK192M_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLK192M_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HCLK192M_A> {
        match self.bits {
            0 => Some(HCLK192M_A::DIV1),
            1 => Some(HCLK192M_A::DIV2),
            2 => Some(HCLK192M_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HCLK192M_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HCLK192M_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HCLK192M_A::DIV4
    }
}
impl core::ops::Deref for HCLK192M_R {
    type Target = crate::FieldReader<u8, HCLK192M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLK192M` writer - High frequency clock HCLK192M"]
pub struct HCLK192M_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK192M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLK192M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide HFCLK192M by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV1)
    }
    #[doc = "Divide HFCLK192M by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV2)
    }
    #[doc = "Divide HFCLK192M by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&self) -> HCLK192M_R {
        HCLK192M_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&mut self) -> HCLK192M_W {
        HCLK192M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFCLK192M frequency configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclk192mctrl](index.html) module"]
pub struct HFCLK192MCTRL_SPEC;
impl crate::RegisterSpec for HFCLK192MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclk192mctrl::R](R) reader structure"]
impl crate::Readable for HFCLK192MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfclk192mctrl::W](W) writer structure"]
impl crate::Writable for HFCLK192MCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFCLK192MCTRL to value 0x02"]
impl crate::Resettable for HFCLK192MCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
