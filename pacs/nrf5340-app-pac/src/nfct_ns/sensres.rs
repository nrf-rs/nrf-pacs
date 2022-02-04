#[doc = "Register `SENSRES` reader"]
pub struct R(crate::R<SENSRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSRES` writer"]
pub struct W(crate::W<SENSRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSRES_SPEC>;
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
impl From<crate::W<SENSRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITFRAMESDD_A {
    #[doc = "0: SDD pattern 00000"]
    SDD00000 = 0,
    #[doc = "1: SDD pattern 00001"]
    SDD00001 = 1,
    #[doc = "2: SDD pattern 00010"]
    SDD00010 = 2,
    #[doc = "4: SDD pattern 00100"]
    SDD00100 = 4,
    #[doc = "8: SDD pattern 01000"]
    SDD01000 = 8,
    #[doc = "16: SDD pattern 10000"]
    SDD10000 = 16,
}
impl From<BITFRAMESDD_A> for u8 {
    #[inline(always)]
    fn from(variant: BITFRAMESDD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BITFRAMESDD` reader - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct BITFRAMESDD_R(crate::FieldReader<u8, BITFRAMESDD_A>);
impl BITFRAMESDD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BITFRAMESDD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITFRAMESDD_A> {
        match self.bits {
            0 => Some(BITFRAMESDD_A::SDD00000),
            1 => Some(BITFRAMESDD_A::SDD00001),
            2 => Some(BITFRAMESDD_A::SDD00010),
            4 => Some(BITFRAMESDD_A::SDD00100),
            8 => Some(BITFRAMESDD_A::SDD01000),
            16 => Some(BITFRAMESDD_A::SDD10000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDD00000`"]
    #[inline(always)]
    pub fn is_sdd00000(&self) -> bool {
        **self == BITFRAMESDD_A::SDD00000
    }
    #[doc = "Checks if the value of the field is `SDD00001`"]
    #[inline(always)]
    pub fn is_sdd00001(&self) -> bool {
        **self == BITFRAMESDD_A::SDD00001
    }
    #[doc = "Checks if the value of the field is `SDD00010`"]
    #[inline(always)]
    pub fn is_sdd00010(&self) -> bool {
        **self == BITFRAMESDD_A::SDD00010
    }
    #[doc = "Checks if the value of the field is `SDD00100`"]
    #[inline(always)]
    pub fn is_sdd00100(&self) -> bool {
        **self == BITFRAMESDD_A::SDD00100
    }
    #[doc = "Checks if the value of the field is `SDD01000`"]
    #[inline(always)]
    pub fn is_sdd01000(&self) -> bool {
        **self == BITFRAMESDD_A::SDD01000
    }
    #[doc = "Checks if the value of the field is `SDD10000`"]
    #[inline(always)]
    pub fn is_sdd10000(&self) -> bool {
        **self == BITFRAMESDD_A::SDD10000
    }
}
impl core::ops::Deref for BITFRAMESDD_R {
    type Target = crate::FieldReader<u8, BITFRAMESDD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITFRAMESDD` writer - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct BITFRAMESDD_W<'a> {
    w: &'a mut W,
}
impl<'a> BITFRAMESDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITFRAMESDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDD pattern 00000"]
    #[inline(always)]
    pub fn sdd00000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00000)
    }
    #[doc = "SDD pattern 00001"]
    #[inline(always)]
    pub fn sdd00001(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00001)
    }
    #[doc = "SDD pattern 00010"]
    #[inline(always)]
    pub fn sdd00010(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00010)
    }
    #[doc = "SDD pattern 00100"]
    #[inline(always)]
    pub fn sdd00100(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00100)
    }
    #[doc = "SDD pattern 01000"]
    #[inline(always)]
    pub fn sdd01000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD01000)
    }
    #[doc = "SDD pattern 10000"]
    #[inline(always)]
    pub fn sdd10000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD10000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `RFU5` reader - Reserved for future use. Shall be 0."]
pub struct RFU5_R(crate::FieldReader<bool, bool>);
impl RFU5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFU5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFU5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFU5` writer - Reserved for future use. Shall be 0."]
pub struct RFU5_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "NFCID1 size. This value is used by the auto collision resolution engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NFCIDSIZE_A {
    #[doc = "0: NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE = 0,
    #[doc = "1: NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE = 1,
    #[doc = "2: NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE = 2,
}
impl From<NFCIDSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCIDSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NFCIDSIZE` reader - NFCID1 size. This value is used by the auto collision resolution engine."]
pub struct NFCIDSIZE_R(crate::FieldReader<u8, NFCIDSIZE_A>);
impl NFCIDSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCIDSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFCIDSIZE_A> {
        match self.bits {
            0 => Some(NFCIDSIZE_A::NFCID1SINGLE),
            1 => Some(NFCIDSIZE_A::NFCID1DOUBLE),
            2 => Some(NFCIDSIZE_A::NFCID1TRIPLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NFCID1SINGLE`"]
    #[inline(always)]
    pub fn is_nfcid1single(&self) -> bool {
        **self == NFCIDSIZE_A::NFCID1SINGLE
    }
    #[doc = "Checks if the value of the field is `NFCID1DOUBLE`"]
    #[inline(always)]
    pub fn is_nfcid1double(&self) -> bool {
        **self == NFCIDSIZE_A::NFCID1DOUBLE
    }
    #[doc = "Checks if the value of the field is `NFCID1TRIPLE`"]
    #[inline(always)]
    pub fn is_nfcid1triple(&self) -> bool {
        **self == NFCIDSIZE_A::NFCID1TRIPLE
    }
}
impl core::ops::Deref for NFCIDSIZE_R {
    type Target = crate::FieldReader<u8, NFCIDSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCIDSIZE` writer - NFCID1 size. This value is used by the auto collision resolution engine."]
pub struct NFCIDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCIDSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NFCIDSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline(always)]
    pub fn nfcid1single(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1SINGLE)
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline(always)]
    pub fn nfcid1double(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1DOUBLE)
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline(always)]
    pub fn nfcid1triple(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1TRIPLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PLATFCONFIG` reader - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct PLATFCONFIG_R(crate::FieldReader<u8, u8>);
impl PLATFCONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLATFCONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLATFCONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLATFCONFIG` writer - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct PLATFCONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PLATFCONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RFU74` reader - Reserved for future use. Shall be 0."]
pub struct RFU74_R(crate::FieldReader<u8, u8>);
impl RFU74_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFU74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFU74_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFU74` writer - Reserved for future use. Shall be 0."]
pub struct RFU74_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU74_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&self) -> BITFRAMESDD_R {
        BITFRAMESDD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&self) -> RFU5_R {
        RFU5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&self) -> NFCIDSIZE_R {
        NFCIDSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&self) -> PLATFCONFIG_R {
        PLATFCONFIG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&self) -> RFU74_R {
        RFU74_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&mut self) -> BITFRAMESDD_W {
        BITFRAMESDD_W { w: self }
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&mut self) -> RFU5_W {
        RFU5_W { w: self }
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&mut self) -> NFCIDSIZE_W {
        NFCIDSIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&mut self) -> PLATFCONFIG_W {
        PLATFCONFIG_W { w: self }
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&mut self) -> RFU74_W {
        RFU74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC-A SENS_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensres](index.html) module"]
pub struct SENSRES_SPEC;
impl crate::RegisterSpec for SENSRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensres::R](R) reader structure"]
impl crate::Readable for SENSRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensres::W](W) writer structure"]
impl crate::Writable for SENSRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSRES to value 0x01"]
impl crate::Resettable for SENSRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
