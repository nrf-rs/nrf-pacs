#[doc = "Register `PCNF0` reader"]
pub struct R(crate::R<PCNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF0` writer"]
pub struct W(crate::W<PCNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF0_SPEC>;
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
impl From<crate::W<PCNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFLEN` reader - Length on air of LENGTH field in number of bits."]
pub struct LFLEN_R(crate::FieldReader<u8, u8>);
impl LFLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LFLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFLEN` writer - Length on air of LENGTH field in number of bits."]
pub struct LFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `S0LEN` reader - Length on air of S0 field in number of bytes."]
pub struct S0LEN_R(crate::FieldReader<bool, bool>);
impl S0LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S0LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S0LEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S0LEN` writer - Length on air of S0 field in number of bytes."]
pub struct S0LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0LEN_W<'a> {
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
#[doc = "Field `S1LEN` reader - Length on air of S1 field in number of bits."]
pub struct S1LEN_R(crate::FieldReader<u8, u8>);
impl S1LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        S1LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S1LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S1LEN` writer - Length on air of S1 field in number of bits."]
pub struct S1LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Include or exclude S1 field in RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INCL_A {
    #[doc = "0: Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC = 0,
    #[doc = "1: Always include S1 field in RAM independent of S1LEN"]
    INCLUDE = 1,
}
impl From<S1INCL_A> for bool {
    #[inline(always)]
    fn from(variant: S1INCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1INCL` reader - Include or exclude S1 field in RAM"]
pub struct S1INCL_R(crate::FieldReader<bool, S1INCL_A>);
impl S1INCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S1INCL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1INCL_A {
        match self.bits {
            false => S1INCL_A::AUTOMATIC,
            true => S1INCL_A::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == S1INCL_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        **self == S1INCL_A::INCLUDE
    }
}
impl core::ops::Deref for S1INCL_R {
    type Target = crate::FieldReader<bool, S1INCL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S1INCL` writer - Include or exclude S1 field in RAM"]
pub struct S1INCL_W<'a> {
    w: &'a mut W,
}
impl<'a> S1INCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1INCL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(S1INCL_A::AUTOMATIC)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(S1INCL_A::INCLUDE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CILEN` reader - Length of code indicator - long range"]
pub struct CILEN_R(crate::FieldReader<u8, u8>);
impl CILEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CILEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CILEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CILEN` writer - Length of code indicator - long range"]
pub struct CILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CILEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Length of preamble on air. Decision point: TASKS_START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLEN_A {
    #[doc = "0: 8-bit preamble"]
    _8BIT = 0,
    #[doc = "1: 16-bit preamble"]
    _16BIT = 1,
    #[doc = "2: 32-bit zero preamble - used for IEEE 802.15.4"]
    _32BITZERO = 2,
    #[doc = "3: Preamble - used for BLE long range"]
    LONGRANGE = 3,
}
impl From<PLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLEN` reader - Length of preamble on air. Decision point: TASKS_START task"]
pub struct PLEN_R(crate::FieldReader<u8, PLEN_A>);
impl PLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLEN_A {
        match self.bits {
            0 => PLEN_A::_8BIT,
            1 => PLEN_A::_16BIT,
            2 => PLEN_A::_32BITZERO,
            3 => PLEN_A::LONGRANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        **self == PLEN_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        **self == PLEN_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BITZERO`"]
    #[inline(always)]
    pub fn is_32bit_zero(&self) -> bool {
        **self == PLEN_A::_32BITZERO
    }
    #[doc = "Checks if the value of the field is `LONGRANGE`"]
    #[inline(always)]
    pub fn is_long_range(&self) -> bool {
        **self == PLEN_A::LONGRANGE
    }
}
impl core::ops::Deref for PLEN_R {
    type Target = crate::FieldReader<u8, PLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLEN` writer - Length of preamble on air. Decision point: TASKS_START task"]
pub struct PLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-bit preamble"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PLEN_A::_8BIT)
    }
    #[doc = "16-bit preamble"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PLEN_A::_16BIT)
    }
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    #[inline(always)]
    pub fn _32bit_zero(self) -> &'a mut W {
        self.variant(PLEN_A::_32BITZERO)
    }
    #[doc = "Preamble - used for BLE long range"]
    #[inline(always)]
    pub fn long_range(self) -> &'a mut W {
        self.variant(PLEN_A::LONGRANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Indicates if LENGTH field contains CRC or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCINC_A {
    #[doc = "0: LENGTH does not contain CRC"]
    EXCLUDE = 0,
    #[doc = "1: LENGTH includes CRC"]
    INCLUDE = 1,
}
impl From<CRCINC_A> for bool {
    #[inline(always)]
    fn from(variant: CRCINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCINC` reader - Indicates if LENGTH field contains CRC or not"]
pub struct CRCINC_R(crate::FieldReader<bool, CRCINC_A>);
impl CRCINC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCINC_A {
        match self.bits {
            false => CRCINC_A::EXCLUDE,
            true => CRCINC_A::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        **self == CRCINC_A::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        **self == CRCINC_A::INCLUDE
    }
}
impl core::ops::Deref for CRCINC_R {
    type Target = crate::FieldReader<bool, CRCINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCINC` writer - Indicates if LENGTH field contains CRC or not"]
pub struct CRCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LENGTH does not contain CRC"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut W {
        self.variant(CRCINC_A::EXCLUDE)
    }
    #[doc = "LENGTH includes CRC"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(CRCINC_A::INCLUDE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TERMLEN` reader - Length of TERM field in Long Range operation"]
pub struct TERMLEN_R(crate::FieldReader<u8, u8>);
impl TERMLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TERMLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERMLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERMLEN` writer - Length of TERM field in Long Range operation"]
pub struct TERMLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TERMLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&self) -> S1INCL_R {
        S1INCL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline(always)]
    pub fn cilen(&self) -> CILEN_R {
        CILEN_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn crcinc(&self) -> CRCINC_R {
        CRCINC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn termlen(&self) -> TERMLEN_R {
        TERMLEN_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LFLEN_W {
        LFLEN_W { w: self }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0LEN_W {
        S0LEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1LEN_W {
        S1LEN_W { w: self }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&mut self) -> S1INCL_W {
        S1INCL_W { w: self }
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline(always)]
    pub fn cilen(&mut self) -> CILEN_W {
        CILEN_W { w: self }
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&mut self) -> PLEN_W {
        PLEN_W { w: self }
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn crcinc(&mut self) -> CRCINC_W {
        CRCINC_W { w: self }
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn termlen(&mut self) -> TERMLEN_W {
        TERMLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf0](index.html) module"]
pub struct PCNF0_SPEC;
impl crate::RegisterSpec for PCNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf0::R](R) reader structure"]
impl crate::Readable for PCNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf0::W](W) writer structure"]
impl crate::Writable for PCNF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for PCNF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
