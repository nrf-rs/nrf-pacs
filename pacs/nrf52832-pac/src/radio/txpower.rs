#[doc = "Register `TXPOWER` reader"]
pub struct R(crate::R<TXPOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPOWER` writer"]
pub struct W(crate::W<TXPOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPOWER_SPEC>;
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
impl From<crate::W<TXPOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RADIO output power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPOWER_A {
    #[doc = "4: +4 dBm"]
    POS4DBM = 4,
    #[doc = "3: +3 dBm"]
    POS3DBM = 3,
    #[doc = "0: 0 dBm"]
    _0DBM = 0,
    #[doc = "252: -4 dBm"]
    NEG4DBM = 252,
    #[doc = "248: -8 dBm"]
    NEG8DBM = 248,
    #[doc = "244: -12 dBm"]
    NEG12DBM = 244,
    #[doc = "240: -16 dBm"]
    NEG16DBM = 240,
    #[doc = "236: -20 dBm"]
    NEG20DBM = 236,
    #[doc = "216: -40 dBm"]
    NEG40DBM = 216,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXPOWER` reader - RADIO output power."]
pub struct TXPOWER_R(crate::FieldReader<u8, TXPOWER_A>);
impl TXPOWER_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXPOWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXPOWER_A> {
        match self.bits {
            4 => Some(TXPOWER_A::POS4DBM),
            3 => Some(TXPOWER_A::POS3DBM),
            0 => Some(TXPOWER_A::_0DBM),
            252 => Some(TXPOWER_A::NEG4DBM),
            248 => Some(TXPOWER_A::NEG8DBM),
            244 => Some(TXPOWER_A::NEG12DBM),
            240 => Some(TXPOWER_A::NEG16DBM),
            236 => Some(TXPOWER_A::NEG20DBM),
            216 => Some(TXPOWER_A::NEG40DBM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POS4DBM`"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        **self == TXPOWER_A::POS4DBM
    }
    #[doc = "Checks if the value of the field is `POS3DBM`"]
    #[inline(always)]
    pub fn is_pos3d_bm(&self) -> bool {
        **self == TXPOWER_A::POS3DBM
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        **self == TXPOWER_A::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG8DBM`"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG8DBM
    }
    #[doc = "Checks if the value of the field is `NEG12DBM`"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG12DBM
    }
    #[doc = "Checks if the value of the field is `NEG16DBM`"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG16DBM
    }
    #[doc = "Checks if the value of the field is `NEG20DBM`"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG20DBM
    }
    #[doc = "Checks if the value of the field is `NEG40DBM`"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG40DBM
    }
}
impl core::ops::Deref for TXPOWER_R {
    type Target = crate::FieldReader<u8, TXPOWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPOWER` writer - RADIO output power."]
pub struct TXPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOWER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS4DBM)
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn pos3d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS3DBM)
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0DBM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4DBM)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG8DBM)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG12DBM)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG16DBM)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG20DBM)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG40DBM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    pub fn txpower(&mut self) -> TXPOWER_W {
        TXPOWER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpower](index.html) module"]
pub struct TXPOWER_SPEC;
impl crate::RegisterSpec for TXPOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpower::R](R) reader structure"]
impl crate::Readable for TXPOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpower::W](W) writer structure"]
impl crate::Writable for TXPOWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TXPOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
