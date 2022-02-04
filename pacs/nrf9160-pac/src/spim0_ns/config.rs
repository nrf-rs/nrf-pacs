#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bit order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORDER_A {
    #[doc = "0: Most significant bit shifted out first"]
    MSBFIRST = 0,
    #[doc = "1: Least significant bit shifted out first"]
    LSBFIRST = 1,
}
impl From<ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORDER` reader - Bit order"]
pub struct ORDER_R(crate::FieldReader<bool, ORDER_A>);
impl ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ORDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDER_A {
        match self.bits {
            false => ORDER_A::MSBFIRST,
            true => ORDER_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        **self == ORDER_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        **self == ORDER_A::LSBFIRST
    }
}
impl core::ops::Deref for ORDER_R {
    type Target = crate::FieldReader<bool, ORDER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORDER` writer - Bit order"]
pub struct ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORDER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Most significant bit shifted out first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(ORDER_A::MSBFIRST)
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(ORDER_A::LSBFIRST)
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
#[doc = "Serial clock (SCK) phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING = 0,
    #[doc = "1: Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Serial clock (SCK) phase"]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::LEADING,
            true => CPHA_A::TRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING`"]
    #[inline(always)]
    pub fn is_leading(&self) -> bool {
        **self == CPHA_A::LEADING
    }
    #[doc = "Checks if the value of the field is `TRAILING`"]
    #[inline(always)]
    pub fn is_trailing(&self) -> bool {
        **self == CPHA_A::TRAILING
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Serial clock (SCK) phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline(always)]
    pub fn leading(self) -> &'a mut W {
        self.variant(CPHA_A::LEADING)
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline(always)]
    pub fn trailing(self) -> &'a mut W {
        self.variant(CPHA_A::TRAILING)
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
#[doc = "Serial clock (SCK) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Active high"]
    ACTIVEHIGH = 0,
    #[doc = "1: Active low"]
    ACTIVELOW = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Serial clock (SCK) polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::ACTIVEHIGH,
            true => CPOL_A::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == CPOL_A::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == CPOL_A::ACTIVELOW
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Serial clock (SCK) polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVEHIGH)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVELOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&self) -> ORDER_R {
        ORDER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&mut self) -> ORDER_W {
        ORDER_W { w: self }
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
