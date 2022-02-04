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
#[doc = "Indicates if parity is added to the frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_A {
    #[doc = "0: Parity is not added to TX frames"]
    NOPARITY = 0,
    #[doc = "1: Parity is added to TX frames"]
    PARITY = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - Indicates if parity is added to the frame"]
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
#[doc = "Field `PARITY` writer - Indicates if parity is added to the frame"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Parity is not added to TX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITY_A::NOPARITY)
    }
    #[doc = "Parity is added to TX frames"]
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
#[doc = "Discarding unused bits at start or end of a frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCARDMODE_A {
    #[doc = "0: Unused bits are discarded at end of frame (EoF)"]
    DISCARDEND = 0,
    #[doc = "1: Unused bits are discarded at start of frame (SoF)"]
    DISCARDSTART = 1,
}
impl From<DISCARDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DISCARDMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCARDMODE` reader - Discarding unused bits at start or end of a frame"]
pub struct DISCARDMODE_R(crate::FieldReader<bool, DISCARDMODE_A>);
impl DISCARDMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCARDMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCARDMODE_A {
        match self.bits {
            false => DISCARDMODE_A::DISCARDEND,
            true => DISCARDMODE_A::DISCARDSTART,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARDEND`"]
    #[inline(always)]
    pub fn is_discard_end(&self) -> bool {
        **self == DISCARDMODE_A::DISCARDEND
    }
    #[doc = "Checks if the value of the field is `DISCARDSTART`"]
    #[inline(always)]
    pub fn is_discard_start(&self) -> bool {
        **self == DISCARDMODE_A::DISCARDSTART
    }
}
impl core::ops::Deref for DISCARDMODE_R {
    type Target = crate::FieldReader<bool, DISCARDMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCARDMODE` writer - Discarding unused bits at start or end of a frame"]
pub struct DISCARDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCARDMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCARDMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    #[inline(always)]
    pub fn discard_end(self) -> &'a mut W {
        self.variant(DISCARDMODE_A::DISCARDEND)
    }
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    #[inline(always)]
    pub fn discard_start(self) -> &'a mut W {
        self.variant(DISCARDMODE_A::DISCARDSTART)
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
#[doc = "Adding SoF or not in TX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: SoF symbol not added"]
    NOSOF = 0,
    #[doc = "1: SoF symbol added"]
    SOF = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Adding SoF or not in TX frames"]
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
#[doc = "Field `SOF` writer - Adding SoF or not in TX frames"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SoF symbol not added"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOF_A::NOSOF)
    }
    #[doc = "SoF symbol added"]
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
#[doc = "CRC mode for outgoing frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCMODETX_A {
    #[doc = "0: CRC is not added to the frame"]
    NOCRCTX = 0,
    #[doc = "1: 16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    CRC16TX = 1,
}
impl From<CRCMODETX_A> for bool {
    #[inline(always)]
    fn from(variant: CRCMODETX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCMODETX` reader - CRC mode for outgoing frames"]
pub struct CRCMODETX_R(crate::FieldReader<bool, CRCMODETX_A>);
impl CRCMODETX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCMODETX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCMODETX_A {
        match self.bits {
            false => CRCMODETX_A::NOCRCTX,
            true => CRCMODETX_A::CRC16TX,
        }
    }
    #[doc = "Checks if the value of the field is `NOCRCTX`"]
    #[inline(always)]
    pub fn is_no_crctx(&self) -> bool {
        **self == CRCMODETX_A::NOCRCTX
    }
    #[doc = "Checks if the value of the field is `CRC16TX`"]
    #[inline(always)]
    pub fn is_crc16tx(&self) -> bool {
        **self == CRCMODETX_A::CRC16TX
    }
}
impl core::ops::Deref for CRCMODETX_R {
    type Target = crate::FieldReader<bool, CRCMODETX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCMODETX` writer - CRC mode for outgoing frames"]
pub struct CRCMODETX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCMODETX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCMODETX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CRC is not added to the frame"]
    #[inline(always)]
    pub fn no_crctx(self) -> &'a mut W {
        self.variant(CRCMODETX_A::NOCRCTX)
    }
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    #[inline(always)]
    pub fn crc16tx(self) -> &'a mut W {
        self.variant(CRCMODETX_A::CRC16TX)
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
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub fn discardmode(&self) -> DISCARDMODE_R {
        DISCARDMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn crcmodetx(&self) -> CRCMODETX_R {
        CRCMODETX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub fn discardmode(&mut self) -> DISCARDMODE_W {
        DISCARDMODE_W { w: self }
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn crcmodetx(&mut self) -> CRCMODETX_W {
        CRCMODETX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of outgoing frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameconfig](index.html) module"]
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
#[doc = "`reset()` method sets FRAMECONFIG to value 0x17"]
impl crate::Resettable for FRAMECONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
