#[doc = "Register `FREQUENCY` reader"]
pub struct R(crate::R<FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQUENCY` writer"]
pub struct W(crate::W<FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQUENCY_SPEC>;
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
impl From<crate::W<FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI master data rate\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FREQUENCY_A {
    #[doc = "33554432: 125 kbps"]
    K125 = 33554432,
    #[doc = "67108864: 250 kbps"]
    K250 = 67108864,
    #[doc = "134217728: 500 kbps"]
    K500 = 134217728,
    #[doc = "268435456: 1 Mbps"]
    M1 = 268435456,
    #[doc = "536870912: 2 Mbps"]
    M2 = 536870912,
    #[doc = "1073741824: 4 Mbps"]
    M4 = 1073741824,
    #[doc = "2147483648: 8 Mbps"]
    M8 = 2147483648,
}
impl From<FREQUENCY_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQUENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQUENCY` reader - SPI master data rate"]
pub struct FREQUENCY_R(crate::FieldReader<u32, FREQUENCY_A>);
impl FREQUENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FREQUENCY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQUENCY_A> {
        match self.bits {
            33554432 => Some(FREQUENCY_A::K125),
            67108864 => Some(FREQUENCY_A::K250),
            134217728 => Some(FREQUENCY_A::K500),
            268435456 => Some(FREQUENCY_A::M1),
            536870912 => Some(FREQUENCY_A::M2),
            1073741824 => Some(FREQUENCY_A::M4),
            2147483648 => Some(FREQUENCY_A::M8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K125`"]
    #[inline(always)]
    pub fn is_k125(&self) -> bool {
        **self == FREQUENCY_A::K125
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        **self == FREQUENCY_A::K250
    }
    #[doc = "Checks if the value of the field is `K500`"]
    #[inline(always)]
    pub fn is_k500(&self) -> bool {
        **self == FREQUENCY_A::K500
    }
    #[doc = "Checks if the value of the field is `M1`"]
    #[inline(always)]
    pub fn is_m1(&self) -> bool {
        **self == FREQUENCY_A::M1
    }
    #[doc = "Checks if the value of the field is `M2`"]
    #[inline(always)]
    pub fn is_m2(&self) -> bool {
        **self == FREQUENCY_A::M2
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        **self == FREQUENCY_A::M4
    }
    #[doc = "Checks if the value of the field is `M8`"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        **self == FREQUENCY_A::M8
    }
}
impl core::ops::Deref for FREQUENCY_R {
    type Target = crate::FieldReader<u32, FREQUENCY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQUENCY` writer - SPI master data rate"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQUENCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "125 kbps"]
    #[inline(always)]
    pub fn k125(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K125)
    }
    #[doc = "250 kbps"]
    #[inline(always)]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K250)
    }
    #[doc = "500 kbps"]
    #[inline(always)]
    pub fn k500(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K500)
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn m1(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M1)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn m2(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M2)
    }
    #[doc = "4 Mbps"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M4)
    }
    #[doc = "8 Mbps"]
    #[inline(always)]
    pub fn m8(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI master data rate"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI master data rate"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](index.html) module"]
pub struct FREQUENCY_SPEC;
impl crate::RegisterSpec for FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frequency::R](R) reader structure"]
impl crate::Readable for FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frequency::W](W) writer structure"]
impl crate::Writable for FREQUENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x0400_0000"]
impl crate::Resettable for FREQUENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
