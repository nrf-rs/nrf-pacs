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
#[doc = "Field `REFSEL` reader - Reference select."]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Use supply with a 1/8 prescaler as reference."]
    SUPPLY_ONE_EIGHTH_PRESCALING = 0,
    #[doc = "1: Use supply with a 2/8 prescaler as reference."]
    SUPPLY_TWO_EIGHTHS_PRESCALING = 1,
    #[doc = "2: Use supply with a 3/8 prescaler as reference."]
    SUPPLY_THREE_EIGHTHS_PRESCALING = 2,
    #[doc = "3: Use supply with a 4/8 prescaler as reference."]
    SUPPLY_FOUR_EIGHTHS_PRESCALING = 3,
    #[doc = "4: Use supply with a 5/8 prescaler as reference."]
    SUPPLY_FIVE_EIGHTHS_PRESCALING = 4,
    #[doc = "5: Use supply with a 6/8 prescaler as reference."]
    SUPPLY_SIX_EIGHTHS_PRESCALING = 5,
    #[doc = "6: Use supply with a 7/8 prescaler as reference."]
    SUPPLY_SEVEN_EIGHTHS_PRESCALING = 6,
    #[doc = "7: Use external analog reference as reference."]
    AREF = 7,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::SUPPLY_ONE_EIGHTH_PRESCALING,
            1 => REFSEL_A::SUPPLY_TWO_EIGHTHS_PRESCALING,
            2 => REFSEL_A::SUPPLY_THREE_EIGHTHS_PRESCALING,
            3 => REFSEL_A::SUPPLY_FOUR_EIGHTHS_PRESCALING,
            4 => REFSEL_A::SUPPLY_FIVE_EIGHTHS_PRESCALING,
            5 => REFSEL_A::SUPPLY_SIX_EIGHTHS_PRESCALING,
            6 => REFSEL_A::SUPPLY_SEVEN_EIGHTHS_PRESCALING,
            7 => REFSEL_A::AREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLY_ONE_EIGHTH_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_eighth_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_ONE_EIGHTH_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_TWO_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_two_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_TWO_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_THREE_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_three_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_THREE_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_FOUR_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_four_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_FOUR_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_FIVE_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_five_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_FIVE_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_SIX_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_six_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_SIX_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_SEVEN_EIGHTHS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_seven_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_SEVEN_EIGHTHS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
}
#[doc = "Field `REFSEL` writer - Reference select."]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REFSEL_SPEC, u8, REFSEL_A, 3, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_one_eighth_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_ONE_EIGHTH_PRESCALING)
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_two_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_TWO_EIGHTHS_PRESCALING)
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_three_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_THREE_EIGHTHS_PRESCALING)
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_four_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_FOUR_EIGHTHS_PRESCALING)
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_five_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_FIVE_EIGHTHS_PRESCALING)
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_six_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_SIX_EIGHTHS_PRESCALING)
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_seven_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_SEVEN_EIGHTHS_PRESCALING)
    }
    #[doc = "Use external analog reference as reference."]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
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
