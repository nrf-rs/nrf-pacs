#[doc = "Register `MCLKCONFIG` reader"]
pub struct R(crate::R<MCLKCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCONFIG` writer"]
pub struct W(crate::W<MCLKCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCONFIG_SPEC>;
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
impl From<crate::W<MCLKCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: 32 MHz peripheral clock"]
    PCLK32M = 0,
    #[doc = "1: Audio PLL clock"]
    ACLK = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Master clock source selection"]
pub struct SRC_R(crate::FieldReader<bool, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::PCLK32M,
            true => SRC_A::ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK32M`"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        **self == SRC_A::PCLK32M
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        **self == SRC_A::ACLK
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Master clock source selection"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32 MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut W {
        self.variant(SRC_A::PCLK32M)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(SRC_A::ACLK)
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
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master clock generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkconfig](index.html) module"]
pub struct MCLKCONFIG_SPEC;
impl crate::RegisterSpec for MCLKCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkconfig::R](R) reader structure"]
impl crate::Readable for MCLKCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkconfig::W](W) writer structure"]
impl crate::Writable for MCLKCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKCONFIG to value 0"]
impl crate::Resettable for MCLKCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
