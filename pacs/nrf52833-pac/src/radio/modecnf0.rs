#[doc = "Register `MODECNF0` reader"]
pub struct R(crate::R<MODECNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODECNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODECNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODECNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODECNF0` writer"]
pub struct W(crate::W<MODECNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODECNF0_SPEC>;
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
impl From<crate::W<MODECNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODECNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Radio ramp-up time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RU_A {
    #[doc = "0: Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    DEFAULT = 0,
    #[doc = "1: Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specification for more information"]
    FAST = 1,
}
impl From<RU_A> for bool {
    #[inline(always)]
    fn from(variant: RU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RU` reader - Radio ramp-up time"]
pub struct RU_R(crate::FieldReader<bool, RU_A>);
impl RU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RU_A {
        match self.bits {
            false => RU_A::DEFAULT,
            true => RU_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == RU_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == RU_A::FAST
    }
}
impl core::ops::Deref for RU_R {
    type Target = crate::FieldReader<bool, RU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RU` writer - Radio ramp-up time"]
pub struct RU_W<'a> {
    w: &'a mut W,
}
impl<'a> RU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RU_A::DEFAULT)
    }
    #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specification for more information"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(RU_A::FAST)
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
#[doc = "Default TX value\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTX_A {
    #[doc = "0: Transmit '1'"]
    B1 = 0,
    #[doc = "1: Transmit '0'"]
    B0 = 1,
    #[doc = "2: Transmit center frequency"]
    CENTER = 2,
}
impl From<DTX_A> for u8 {
    #[inline(always)]
    fn from(variant: DTX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTX` reader - Default TX value"]
pub struct DTX_R(crate::FieldReader<u8, DTX_A>);
impl DTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTX_A> {
        match self.bits {
            0 => Some(DTX_A::B1),
            1 => Some(DTX_A::B0),
            2 => Some(DTX_A::CENTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        **self == DTX_A::B1
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        **self == DTX_A::B0
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        **self == DTX_A::CENTER
    }
}
impl core::ops::Deref for DTX_R {
    type Target = crate::FieldReader<u8, DTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTX` writer - Default TX value"]
pub struct DTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmit '1'"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(DTX_A::B1)
    }
    #[doc = "Transmit '0'"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut W {
        self.variant(DTX_A::B0)
    }
    #[doc = "Transmit center frequency"]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(DTX_A::CENTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&self) -> DTX_R {
        DTX_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&mut self) -> RU_W {
        RU_W { w: self }
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&mut self) -> DTX_W {
        DTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio mode configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modecnf0](index.html) module"]
pub struct MODECNF0_SPEC;
impl crate::RegisterSpec for MODECNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modecnf0::R](R) reader structure"]
impl crate::Readable for MODECNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modecnf0::W](W) writer structure"]
impl crate::Writable for MODECNF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODECNF0 to value 0x0200"]
impl crate::Resettable for MODECNF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
