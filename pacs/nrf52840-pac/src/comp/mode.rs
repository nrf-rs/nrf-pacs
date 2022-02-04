#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Speed and power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SP_A {
    #[doc = "0: Low-power mode"]
    LOW = 0,
    #[doc = "1: Normal mode"]
    NORMAL = 1,
    #[doc = "2: High-speed mode"]
    HIGH = 2,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SP` reader - Speed and power modes"]
pub struct SP_R(crate::FieldReader<u8, SP_A>);
impl SP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SP_A> {
        match self.bits {
            0 => Some(SP_A::LOW),
            1 => Some(SP_A::NORMAL),
            2 => Some(SP_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SP_A::LOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SP_A::HIGH
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<u8, SP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` writer - Speed and power modes"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SP_A::LOW)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SP_A::NORMAL)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SP_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Main operation modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_A {
    #[doc = "0: Single-ended mode"]
    SE = 0,
    #[doc = "1: Differential mode"]
    DIFF = 1,
}
impl From<MAIN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAIN` reader - Main operation modes"]
pub struct MAIN_R(crate::FieldReader<bool, MAIN_A>);
impl MAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_A {
        match self.bits {
            false => MAIN_A::SE,
            true => MAIN_A::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        **self == MAIN_A::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        **self == MAIN_A::DIFF
    }
}
impl core::ops::Deref for MAIN_R {
    type Target = crate::FieldReader<bool, MAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN` writer - Main operation modes"]
pub struct MAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single-ended mode"]
    #[inline(always)]
    pub fn se(self) -> &'a mut W {
        self.variant(MAIN_A::SE)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut W {
        self.variant(MAIN_A::DIFF)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&self) -> MAIN_R {
        MAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&mut self) -> MAIN_W {
        MAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
