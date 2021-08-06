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
#[doc = "RADIO output power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPOWER_A {
    #[doc = "0: 0 dBm"]
    _0DBM = 0,
    #[doc = "255: -1 dBm"]
    NEG1DBM = 255,
    #[doc = "254: -2 dBm"]
    NEG2DBM = 254,
    #[doc = "253: -3 dBm"]
    NEG3DBM = 253,
    #[doc = "252: -4 dBm"]
    NEG4DBM = 252,
    #[doc = "251: -5 dBm"]
    NEG5DBM = 251,
    #[doc = "250: -6 dBm"]
    NEG6DBM = 250,
    #[doc = "249: -7 dBm"]
    NEG7DBM = 249,
    #[doc = "248: -8 dBm"]
    NEG8DBM = 248,
    #[doc = "244: -12 dBm"]
    NEG12DBM = 244,
    #[doc = "240: -16 dBm"]
    NEG16DBM = 240,
    #[doc = "236: -20 dBm"]
    NEG20DBM = 236,
    #[doc = "226: Deprecated enumerator -  -40 dBm"]
    NEG30DBM = 226,
    #[doc = "216: -40 dBm"]
    NEG40DBM = 216,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXPOWER` reader - RADIO output power"]
pub struct TXPOWER_R(crate::FieldReader<u8, TXPOWER_A>);
impl TXPOWER_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXPOWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXPOWER_A> {
        match self.bits {
            0 => Some(TXPOWER_A::_0DBM),
            255 => Some(TXPOWER_A::NEG1DBM),
            254 => Some(TXPOWER_A::NEG2DBM),
            253 => Some(TXPOWER_A::NEG3DBM),
            252 => Some(TXPOWER_A::NEG4DBM),
            251 => Some(TXPOWER_A::NEG5DBM),
            250 => Some(TXPOWER_A::NEG6DBM),
            249 => Some(TXPOWER_A::NEG7DBM),
            248 => Some(TXPOWER_A::NEG8DBM),
            244 => Some(TXPOWER_A::NEG12DBM),
            240 => Some(TXPOWER_A::NEG16DBM),
            236 => Some(TXPOWER_A::NEG20DBM),
            226 => Some(TXPOWER_A::NEG30DBM),
            216 => Some(TXPOWER_A::NEG40DBM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        **self == TXPOWER_A::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG1DBM`"]
    #[inline(always)]
    pub fn is_neg1d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG1DBM
    }
    #[doc = "Checks if the value of the field is `NEG2DBM`"]
    #[inline(always)]
    pub fn is_neg2d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG2DBM
    }
    #[doc = "Checks if the value of the field is `NEG3DBM`"]
    #[inline(always)]
    pub fn is_neg3d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG3DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG5DBM`"]
    #[inline(always)]
    pub fn is_neg5d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG5DBM
    }
    #[doc = "Checks if the value of the field is `NEG6DBM`"]
    #[inline(always)]
    pub fn is_neg6d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG6DBM
    }
    #[doc = "Checks if the value of the field is `NEG7DBM`"]
    #[inline(always)]
    pub fn is_neg7d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG7DBM
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
    #[doc = "Checks if the value of the field is `NEG30DBM`"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        **self == TXPOWER_A::NEG30DBM
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
#[doc = "Field `TXPOWER` writer - RADIO output power"]
pub struct TXPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOWER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0DBM)
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn neg1d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG1DBM)
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn neg2d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG2DBM)
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn neg3d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG3DBM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4DBM)
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn neg5d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG5DBM)
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn neg6d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG6DBM)
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn neg7d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG7DBM)
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
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG30DBM)
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
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power"]
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
