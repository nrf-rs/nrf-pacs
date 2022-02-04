#[doc = "Register `FRAMECONFIG` reader"]
pub struct R(crate::R<FRAMECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMECONFIG` writer"]
pub struct W(crate::W<FRAMECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMECONFIG_SPEC>;
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
impl From<crate::W<FRAMECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Indicates if parity expected in RX frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_A {
    #[doc = "0: Parity is not expected in RX frames"]
    NOPARITY = 0,
    #[doc = "1: Parity is expected in RX frames"]
    PARITY = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - Indicates if parity expected in RX frame"]
pub struct PARITY_R(crate::FieldReader<bool, PARITY_A>);
impl PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            false => PARITY_A::NOPARITY,
            true => PARITY_A::PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NOPARITY`"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        **self == PARITY_A::NOPARITY
    }
    #[doc = "Checks if the value of the field is `PARITY`"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        **self == PARITY_A::PARITY
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<bool, PARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` writer - Indicates if parity expected in RX frame"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Parity is not expected in RX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITY_A::NOPARITY)
    }
    #[doc = "Parity is expected in RX frames"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut W {
        self.variant(PARITY_A::PARITY)
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
#[doc = "SoF expected or not in RX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: SoF symbol is not expected in RX frames"]
    NOSOF = 0,
    #[doc = "1: SoF symbol is expected in RX frames"]
    SOF = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - SoF expected or not in RX frames"]
pub struct SOF_R(crate::FieldReader<bool, SOF_A>);
impl SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::NOSOF,
            true => SOF_A::SOF,
        }
    }
    #[doc = "Checks if the value of the field is `NOSOF`"]
    #[inline(always)]
    pub fn is_no_so_f(&self) -> bool {
        **self == SOF_A::NOSOF
    }
    #[doc = "Checks if the value of the field is `SOF`"]
    #[inline(always)]
    pub fn is_so_f(&self) -> bool {
        **self == SOF_A::SOF
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, SOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` writer - SoF expected or not in RX frames"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SoF symbol is not expected in RX frames"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOF_A::NOSOF)
    }
    #[doc = "SoF symbol is expected in RX frames"]
    #[inline(always)]
    pub fn so_f(self) -> &'a mut W {
        self.variant(SOF_A::SOF)
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
#[doc = "CRC mode for incoming frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCMODERX_A {
    #[doc = "0: CRC is not expected in RX frames"]
    NOCRCRX = 0,
    #[doc = "1: Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    CRC16RX = 1,
}
impl From<CRCMODERX_A> for bool {
    #[inline(always)]
    fn from(variant: CRCMODERX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCMODERX` reader - CRC mode for incoming frames"]
pub struct CRCMODERX_R(crate::FieldReader<bool, CRCMODERX_A>);
impl CRCMODERX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCMODERX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCMODERX_A {
        match self.bits {
            false => CRCMODERX_A::NOCRCRX,
            true => CRCMODERX_A::CRC16RX,
        }
    }
    #[doc = "Checks if the value of the field is `NOCRCRX`"]
    #[inline(always)]
    pub fn is_no_crcrx(&self) -> bool {
        **self == CRCMODERX_A::NOCRCRX
    }
    #[doc = "Checks if the value of the field is `CRC16RX`"]
    #[inline(always)]
    pub fn is_crc16rx(&self) -> bool {
        **self == CRCMODERX_A::CRC16RX
    }
}
impl core::ops::Deref for CRCMODERX_R {
    type Target = crate::FieldReader<bool, CRCMODERX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCMODERX` writer - CRC mode for incoming frames"]
pub struct CRCMODERX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCMODERX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCMODERX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CRC is not expected in RX frames"]
    #[inline(always)]
    pub fn no_crcrx(self) -> &'a mut W {
        self.variant(CRCMODERX_A::NOCRCRX)
    }
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    #[inline(always)]
    pub fn crc16rx(self) -> &'a mut W {
        self.variant(CRCMODERX_A::CRC16RX)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    pub fn crcmoderx(&self) -> CRCMODERX_R {
        CRCMODERX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    pub fn crcmoderx(&mut self) -> CRCMODERX_W {
        CRCMODERX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of incoming frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameconfig](index.html) module"]
pub struct FRAMECONFIG_SPEC;
impl crate::RegisterSpec for FRAMECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameconfig::R](R) reader structure"]
impl crate::Readable for FRAMECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameconfig::W](W) writer structure"]
impl crate::Writable for FRAMECONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMECONFIG to value 0x15"]
impl crate::Resettable for FRAMECONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
