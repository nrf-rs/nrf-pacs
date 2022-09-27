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
#[doc = "Field `CCAMODE` reader - CCA mode of operation"]
pub type CCAMODE_R = crate::FieldReader<u8, CCAMODE_A>;
#[doc = "CCA mode of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCAMODE_A {
    #[doc = "0: Energy above threshold"]
    ED_MODE = 0,
    #[doc = "1: Carrier seen"]
    CARRIER_MODE = 1,
    #[doc = "2: Energy above threshold AND carrier seen"]
    CARRIER_AND_ED_MODE = 2,
    #[doc = "3: Energy above threshold OR carrier seen"]
    CARRIER_OR_ED_MODE = 3,
    #[doc = "4: Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    ED_MODE_TEST1 = 4,
}
impl From<CCAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCAMODE_A) -> Self {
        variant as _
    }
}
impl CCAMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCAMODE_A> {
        match self.bits {
            0 => Some(CCAMODE_A::ED_MODE),
            1 => Some(CCAMODE_A::CARRIER_MODE),
            2 => Some(CCAMODE_A::CARRIER_AND_ED_MODE),
            3 => Some(CCAMODE_A::CARRIER_OR_ED_MODE),
            4 => Some(CCAMODE_A::ED_MODE_TEST1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ED_MODE`"]
    #[inline(always)]
    pub fn is_ed_mode(&self) -> bool {
        *self == CCAMODE_A::ED_MODE
    }
    #[doc = "Checks if the value of the field is `CARRIER_MODE`"]
    #[inline(always)]
    pub fn is_carrier_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIER_MODE
    }
    #[doc = "Checks if the value of the field is `CARRIER_AND_ED_MODE`"]
    #[inline(always)]
    pub fn is_carrier_and_ed_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIER_AND_ED_MODE
    }
    #[doc = "Checks if the value of the field is `CARRIER_OR_ED_MODE`"]
    #[inline(always)]
    pub fn is_carrier_or_ed_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIER_OR_ED_MODE
    }
    #[doc = "Checks if the value of the field is `ED_MODE_TEST1`"]
    #[inline(always)]
    pub fn is_ed_mode_test1(&self) -> bool {
        *self == CCAMODE_A::ED_MODE_TEST1
    }
}
#[doc = "Field `CCAMODE` writer - CCA mode of operation"]
pub type CCAMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCACTRL_SPEC, u8, CCAMODE_A, 3, O>;
impl<'a, const O: u8> CCAMODE_W<'a, O> {
    #[doc = "Energy above threshold"]
    #[inline(always)]
    pub fn ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::ED_MODE)
    }
    #[doc = "Carrier seen"]
    #[inline(always)]
    pub fn carrier_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIER_MODE)
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline(always)]
    pub fn carrier_and_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIER_AND_ED_MODE)
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline(always)]
    pub fn carrier_or_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIER_OR_ED_MODE)
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline(always)]
    pub fn ed_mode_test1(self) -> &'a mut W {
        self.variant(CCAMODE_A::ED_MODE_TEST1)
    }
}
#[doc = "Field `CCAEDTHRES` reader - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub type CCAEDTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCAEDTHRES` writer - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub type CCAEDTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCACTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CCACORRTHRES` reader - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub type CCACORRTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCACORRTHRES` writer - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub type CCACORRTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCACTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CCACORRCNT` reader - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub type CCACORRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCACORRCNT` writer - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub type CCACORRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCACTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&self) -> CCAMODE_R {
        CCAMODE_R::new((self.bits & 7) as u8)
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
    pub fn ccamode(&mut self) -> CCAMODE_W<0> {
        CCAMODE_W::new(self)
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&mut self) -> CCAEDTHRES_W<8> {
        CCAEDTHRES_W::new(self)
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&mut self) -> CCACORRTHRES_W<16> {
        CCACORRTHRES_W::new(self)
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&mut self) -> CCACORRCNT_W<24> {
        CCACORRCNT_W::new(self)
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
