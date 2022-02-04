#[doc = "Register `CRCCNF` reader"]
pub struct R(crate::R<CRCCNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCNF` writer"]
pub struct W(crate::W<CRCCNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCNF_SPEC>;
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
impl From<crate::W<CRCCNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC length in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: CRC length is zero and CRC calculation is disabled"]
    DISABLED = 0,
    #[doc = "1: CRC length is one byte and CRC calculation is enabled"]
    ONE = 1,
    #[doc = "2: CRC length is two bytes and CRC calculation is enabled"]
    TWO = 2,
    #[doc = "3: CRC length is three bytes and CRC calculation is enabled"]
    THREE = 3,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEN` reader - CRC length in number of bytes."]
pub struct LEN_R(crate::FieldReader<u8, LEN_A>);
impl LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            0 => LEN_A::DISABLED,
            1 => LEN_A::ONE,
            2 => LEN_A::TWO,
            3 => LEN_A::THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == LEN_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == LEN_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        **self == LEN_A::THREE
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u8, LEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - CRC length in number of bytes."]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LEN_A::DISABLED)
    }
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(LEN_A::ONE)
    }
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(LEN_A::TWO)
    }
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(LEN_A::THREE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Include or exclude packet address field out of CRC calculation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SKIPADDR_A {
    #[doc = "0: CRC calculation includes address field"]
    INCLUDE = 0,
    #[doc = "1: CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    SKIP = 1,
    #[doc = "2: CRC calculation as per 802.15.4 standard. Starting at first byte after length field."]
    IEEE802154 = 2,
}
impl From<SKIPADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: SKIPADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SKIPADDR` reader - Include or exclude packet address field out of CRC calculation."]
pub struct SKIPADDR_R(crate::FieldReader<u8, SKIPADDR_A>);
impl SKIPADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SKIPADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SKIPADDR_A> {
        match self.bits {
            0 => Some(SKIPADDR_A::INCLUDE),
            1 => Some(SKIPADDR_A::SKIP),
            2 => Some(SKIPADDR_A::IEEE802154),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        **self == SKIPADDR_A::INCLUDE
    }
    #[doc = "Checks if the value of the field is `SKIP`"]
    #[inline(always)]
    pub fn is_skip(&self) -> bool {
        **self == SKIPADDR_A::SKIP
    }
    #[doc = "Checks if the value of the field is `IEEE802154`"]
    #[inline(always)]
    pub fn is_ieee802154(&self) -> bool {
        **self == SKIPADDR_A::IEEE802154
    }
}
impl core::ops::Deref for SKIPADDR_R {
    type Target = crate::FieldReader<u8, SKIPADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIPADDR` writer - Include or exclude packet address field out of CRC calculation."]
pub struct SKIPADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIPADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKIPADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CRC calculation includes address field"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(SKIPADDR_A::INCLUDE)
    }
    #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    #[inline(always)]
    pub fn skip(self) -> &'a mut W {
        self.variant(SKIPADDR_A::SKIP)
    }
    #[doc = "CRC calculation as per 802.15.4 standard. Starting at first byte after length field."]
    #[inline(always)]
    pub fn ieee802154(self) -> &'a mut W {
        self.variant(SKIPADDR_A::IEEE802154)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn skipaddr(&self) -> SKIPADDR_R {
        SKIPADDR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn skipaddr(&mut self) -> SKIPADDR_W {
        SKIPADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccnf](index.html) module"]
pub struct CRCCNF_SPEC;
impl crate::RegisterSpec for CRCCNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crccnf::R](R) reader structure"]
impl crate::Readable for CRCCNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccnf::W](W) writer structure"]
impl crate::Writable for CRCCNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCCNF to value 0"]
impl crate::Resettable for CRCCNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
