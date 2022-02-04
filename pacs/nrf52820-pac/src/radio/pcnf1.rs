#[doc = "Register `PCNF1` reader"]
pub struct R(crate::R<PCNF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF1` writer"]
pub struct W(crate::W<PCNF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF1_SPEC>;
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
impl From<crate::W<PCNF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXLEN` reader - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub struct MAXLEN_R(crate::FieldReader<u8, u8>);
impl MAXLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXLEN` writer - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub struct MAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `STATLEN` reader - Static length in number of bytes"]
pub struct STATLEN_R(crate::FieldReader<u8, u8>);
impl STATLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATLEN` writer - Static length in number of bytes"]
pub struct STATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BALEN` reader - Base address length in number of bytes"]
pub struct BALEN_R(crate::FieldReader<u8, u8>);
impl BALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BALEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BALEN` writer - Base address length in number of bytes"]
pub struct BALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIAN_A {
    #[doc = "0: Least significant bit on air first"]
    LITTLE = 0,
    #[doc = "1: Most significant bit on air first"]
    BIG = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIAN` reader - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
pub struct ENDIAN_R(crate::FieldReader<bool, ENDIAN_A>);
impl ENDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDIAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LITTLE,
            true => ENDIAN_A::BIG,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        **self == ENDIAN_A::LITTLE
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        **self == ENDIAN_A::BIG
    }
}
impl core::ops::Deref for ENDIAN_R {
    type Target = crate::FieldReader<bool, ENDIAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDIAN` writer - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Least significant bit on air first"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = "Most significant bit on air first"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIAN_A::BIG)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable or disable packet whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEEN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<WHITEEN_A> for bool {
    #[inline(always)]
    fn from(variant: WHITEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHITEEN` reader - Enable or disable packet whitening"]
pub struct WHITEEN_R(crate::FieldReader<bool, WHITEEN_A>);
impl WHITEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WHITEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHITEEN_A {
        match self.bits {
            false => WHITEEN_A::DISABLED,
            true => WHITEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WHITEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WHITEEN_A::ENABLED
    }
}
impl core::ops::Deref for WHITEEN_R {
    type Target = crate::FieldReader<bool, WHITEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHITEEN` writer - Enable or disable packet whitening"]
pub struct WHITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WHITEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WHITEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&self) -> STATLEN_R {
        STATLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&self) -> BALEN_R {
        BALEN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&self) -> WHITEEN_R {
        WHITEEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MAXLEN_W {
        MAXLEN_W { w: self }
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&mut self) -> STATLEN_W {
        STATLEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&mut self) -> BALEN_W {
        BALEN_W { w: self }
    }
    #[doc = "Bit 24 - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&mut self) -> WHITEEN_W {
        WHITEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf1](index.html) module"]
pub struct PCNF1_SPEC;
impl crate::RegisterSpec for PCNF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf1::R](R) reader structure"]
impl crate::Readable for PCNF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf1::W](W) writer structure"]
impl crate::Writable for PCNF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCNF1 to value 0"]
impl crate::Resettable for PCNF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
