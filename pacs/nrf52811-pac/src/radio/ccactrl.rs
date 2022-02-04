#[doc = "Register `CCACTRL` reader"]
pub struct R(crate::R<CCACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCACTRL` writer"]
pub struct W(crate::W<CCACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCACTRL_SPEC>;
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
impl From<crate::W<CCACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CCA mode of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCAMODE_A {
    #[doc = "0: Energy above threshold"]
    EDMODE = 0,
    #[doc = "1: Carrier seen"]
    CARRIERMODE = 1,
    #[doc = "2: Energy above threshold AND carrier seen"]
    CARRIERANDEDMODE = 2,
    #[doc = "3: Energy above threshold OR carrier seen"]
    CARRIEROREDMODE = 3,
    #[doc = "4: Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EDMODETEST1 = 4,
}
impl From<CCAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCAMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CCAMODE` reader - CCA mode of operation"]
pub struct CCAMODE_R(crate::FieldReader<u8, CCAMODE_A>);
impl CCAMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCAMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCAMODE_A> {
        match self.bits {
            0 => Some(CCAMODE_A::EDMODE),
            1 => Some(CCAMODE_A::CARRIERMODE),
            2 => Some(CCAMODE_A::CARRIERANDEDMODE),
            3 => Some(CCAMODE_A::CARRIEROREDMODE),
            4 => Some(CCAMODE_A::EDMODETEST1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EDMODE`"]
    #[inline(always)]
    pub fn is_ed_mode(&self) -> bool {
        **self == CCAMODE_A::EDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERMODE`"]
    #[inline(always)]
    pub fn is_carrier_mode(&self) -> bool {
        **self == CCAMODE_A::CARRIERMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERANDEDMODE`"]
    #[inline(always)]
    pub fn is_carrier_and_ed_mode(&self) -> bool {
        **self == CCAMODE_A::CARRIERANDEDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIEROREDMODE`"]
    #[inline(always)]
    pub fn is_carrier_or_ed_mode(&self) -> bool {
        **self == CCAMODE_A::CARRIEROREDMODE
    }
    #[doc = "Checks if the value of the field is `EDMODETEST1`"]
    #[inline(always)]
    pub fn is_ed_mode_test1(&self) -> bool {
        **self == CCAMODE_A::EDMODETEST1
    }
}
impl core::ops::Deref for CCAMODE_R {
    type Target = crate::FieldReader<u8, CCAMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCAMODE` writer - CCA mode of operation"]
pub struct CCAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCAMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Energy above threshold"]
    #[inline(always)]
    pub fn ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::EDMODE)
    }
    #[doc = "Carrier seen"]
    #[inline(always)]
    pub fn carrier_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIERMODE)
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline(always)]
    pub fn carrier_and_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIERANDEDMODE)
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline(always)]
    pub fn carrier_or_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIEROREDMODE)
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline(always)]
    pub fn ed_mode_test1(self) -> &'a mut W {
        self.variant(CCAMODE_A::EDMODETEST1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `CCAEDTHRES` reader - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub struct CCAEDTHRES_R(crate::FieldReader<u8, u8>);
impl CCAEDTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCAEDTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCAEDTHRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCAEDTHRES` writer - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub struct CCAEDTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAEDTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CCACORRTHRES` reader - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub struct CCACORRTHRES_R(crate::FieldReader<u8, u8>);
impl CCACORRTHRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCACORRTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCACORRTHRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCACORRTHRES` writer - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub struct CCACORRTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACORRTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CCACORRCNT` reader - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub struct CCACORRCNT_R(crate::FieldReader<u8, u8>);
impl CCACORRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCACORRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCACORRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCACORRCNT` writer - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub struct CCACORRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACORRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&self) -> CCAMODE_R {
        CCAMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&self) -> CCAEDTHRES_R {
        CCAEDTHRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&self) -> CCACORRTHRES_R {
        CCACORRTHRES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&self) -> CCACORRCNT_R {
        CCACORRCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&mut self) -> CCAMODE_W {
        CCAMODE_W { w: self }
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&mut self) -> CCAEDTHRES_W {
        CCAEDTHRES_W { w: self }
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&mut self) -> CCACORRTHRES_W {
        CCACORRTHRES_W { w: self }
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&mut self) -> CCACORRCNT_W {
        CCACORRCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 clear channel assessment control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccactrl](index.html) module"]
pub struct CCACTRL_SPEC;
impl crate::RegisterSpec for CCACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccactrl::R](R) reader structure"]
impl crate::Readable for CCACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccactrl::W](W) writer structure"]
impl crate::Writable for CCACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCACTRL to value 0x052d_0000"]
impl crate::Resettable for CCACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x052d_0000
    }
}
