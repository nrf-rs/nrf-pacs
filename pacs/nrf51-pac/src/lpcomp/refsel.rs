#[doc = "Register `REFSEL` reader"]
pub struct R(crate::R<REFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFSEL` writer"]
pub struct W(crate::W<REFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFSEL_SPEC>;
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
impl From<crate::W<REFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reference select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Use supply with a 1/8 prescaler as reference."]
    SUPPLYONEEIGHTHPRESCALING = 0,
    #[doc = "1: Use supply with a 2/8 prescaler as reference."]
    SUPPLYTWOEIGHTHSPRESCALING = 1,
    #[doc = "2: Use supply with a 3/8 prescaler as reference."]
    SUPPLYTHREEEIGHTHSPRESCALING = 2,
    #[doc = "3: Use supply with a 4/8 prescaler as reference."]
    SUPPLYFOUREIGHTHSPRESCALING = 3,
    #[doc = "4: Use supply with a 5/8 prescaler as reference."]
    SUPPLYFIVEEIGHTHSPRESCALING = 4,
    #[doc = "5: Use supply with a 6/8 prescaler as reference."]
    SUPPLYSIXEIGHTHSPRESCALING = 5,
    #[doc = "6: Use supply with a 7/8 prescaler as reference."]
    SUPPLYSEVENEIGHTHSPRESCALING = 6,
    #[doc = "7: Use external analog reference as reference."]
    AREF = 7,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Reference select."]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::SUPPLYONEEIGHTHPRESCALING,
            1 => REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING,
            2 => REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING,
            3 => REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING,
            4 => REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING,
            5 => REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING,
            6 => REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING,
            7 => REFSEL_A::AREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLYONEEIGHTHPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_eighth_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYONEEIGHTHPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTWOEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_two_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTHREEEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_three_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFOUREIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_four_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFIVEEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_five_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSIXEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_six_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSEVENEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_seven_eighths_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        **self == REFSEL_A::AREF
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Reference select."]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_one_eighth_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYONEEIGHTHPRESCALING)
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_two_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_three_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_four_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_five_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_six_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_seven_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING)
    }
    #[doc = "Use external analog reference as reference."]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refsel](index.html) module"]
pub struct REFSEL_SPEC;
impl crate::RegisterSpec for REFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refsel::R](R) reader structure"]
impl crate::Readable for REFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refsel::W](W) writer structure"]
impl crate::Writable for REFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFSEL to value 0"]
impl crate::Resettable for REFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
