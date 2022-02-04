#[doc = "Register `SELRES` reader"]
pub struct R(crate::R<SELRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELRES` writer"]
pub struct W(crate::W<SELRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELRES_SPEC>;
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
impl From<crate::W<SELRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFU10` reader - Reserved for future use. Shall be 0."]
pub struct RFU10_R(crate::FieldReader<u8, u8>);
impl RFU10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFU10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFU10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFU10` writer - Reserved for future use. Shall be 0."]
pub struct RFU10_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CASCADE` reader - Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
pub struct CASCADE_R(crate::FieldReader<bool, bool>);
impl CASCADE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CASCADE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CASCADE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CASCADE` writer - Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
pub struct CASCADE_W<'a> {
    w: &'a mut W,
}
impl<'a> CASCADE_W<'a> {
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
#[doc = "Field `RFU43` reader - Reserved for future use. Shall be 0."]
pub struct RFU43_R(crate::FieldReader<u8, u8>);
impl RFU43_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFU43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFU43_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFU43` writer - Reserved for future use. Shall be 0."]
pub struct RFU43_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `PROTOCOL` reader - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct PROTOCOL_R(crate::FieldReader<u8, u8>);
impl PROTOCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROTOCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTOCOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTOCOL` writer - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub struct PROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTOCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `RFU7` reader - Reserved for future use. Shall be 0."]
pub struct RFU7_R(crate::FieldReader<bool, bool>);
impl RFU7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFU7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFU7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFU7` writer - Reserved for future use. Shall be 0."]
pub struct RFU7_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&self) -> RFU10_R {
        RFU10_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
    #[inline(always)]
    pub fn cascade(&self) -> CASCADE_R {
        CASCADE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&self) -> RFU43_R {
        RFU43_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&self) -> RFU7_R {
        RFU7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&mut self) -> RFU10_W {
        RFU10_W { w: self }
    }
    #[doc = "Bit 2 - Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
    #[inline(always)]
    pub fn cascade(&mut self) -> CASCADE_W {
        CASCADE_W { w: self }
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&mut self) -> RFU43_W {
        RFU43_W { w: self }
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&mut self) -> PROTOCOL_W {
        PROTOCOL_W { w: self }
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&mut self) -> RFU7_W {
        RFU7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC-A SEL_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selres](index.html) module"]
pub struct SELRES_SPEC;
impl crate::RegisterSpec for SELRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [selres::R](R) reader structure"]
impl crate::Readable for SELRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selres::W](W) writer structure"]
impl crate::Writable for SELRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SELRES to value 0"]
impl crate::Resettable for SELRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
