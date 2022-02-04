#[doc = "Register `STALLSTAT` reader"]
pub struct R(crate::R<STALLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STALLSTAT` writer"]
pub struct W(crate::W<STALLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STALLSTAT_SPEC>;
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
impl From<crate::W<STALLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STALLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stall status for EasyDMA RAM reads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: No stall"]
    NOSTALL = 0,
    #[doc = "1: A stall has occurred"]
    STALL = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Stall status for EasyDMA RAM reads"]
pub struct TX_R(crate::FieldReader<bool, TX_A>);
impl TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::NOSTALL,
            true => TX_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        **self == TX_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == TX_A::STALL
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<bool, TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX` writer - Stall status for EasyDMA RAM reads"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut W {
        self.variant(TX_A::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(TX_A::STALL)
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
#[doc = "Stall status for EasyDMA RAM writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: No stall"]
    NOSTALL = 0,
    #[doc = "1: A stall has occurred"]
    STALL = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Stall status for EasyDMA RAM writes"]
pub struct RX_R(crate::FieldReader<bool, RX_A>);
impl RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::NOSTALL,
            true => RX_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        **self == RX_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == RX_A::STALL
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX` writer - Stall status for EasyDMA RAM writes"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut W {
        self.variant(RX_A::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(RX_A::STALL)
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
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stallstat](index.html) module"]
pub struct STALLSTAT_SPEC;
impl crate::RegisterSpec for STALLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stallstat::R](R) reader structure"]
impl crate::Readable for STALLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stallstat::W](W) writer structure"]
impl crate::Writable for STALLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STALLSTAT to value 0"]
impl crate::Resettable for STALLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
