#[doc = "Register `REFSEL` reader"]
pub type R = crate::R<RefselSpec>;
#[doc = "Register `REFSEL` writer"]
pub type W = crate::W<RefselSpec>;
#[doc = "Reference select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Use supply with a 1/8 prescaler as reference."]
    SupplyOneEighthPrescaling = 0,
    #[doc = "1: Use supply with a 2/8 prescaler as reference."]
    SupplyTwoEighthsPrescaling = 1,
    #[doc = "2: Use supply with a 3/8 prescaler as reference."]
    SupplyThreeEighthsPrescaling = 2,
    #[doc = "3: Use supply with a 4/8 prescaler as reference."]
    SupplyFourEighthsPrescaling = 3,
    #[doc = "4: Use supply with a 5/8 prescaler as reference."]
    SupplyFiveEighthsPrescaling = 4,
    #[doc = "5: Use supply with a 6/8 prescaler as reference."]
    SupplySixEighthsPrescaling = 5,
    #[doc = "6: Use supply with a 7/8 prescaler as reference."]
    SupplySevenEighthsPrescaling = 6,
    #[doc = "7: Use external analog reference as reference."]
    Aref = 7,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference select."]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            0 => Refsel::SupplyOneEighthPrescaling,
            1 => Refsel::SupplyTwoEighthsPrescaling,
            2 => Refsel::SupplyThreeEighthsPrescaling,
            3 => Refsel::SupplyFourEighthsPrescaling,
            4 => Refsel::SupplyFiveEighthsPrescaling,
            5 => Refsel::SupplySixEighthsPrescaling,
            6 => Refsel::SupplySevenEighthsPrescaling,
            7 => Refsel::Aref,
            _ => unreachable!(),
        }
    }
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_one_eighth_prescaling(&self) -> bool {
        *self == Refsel::SupplyOneEighthPrescaling
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_two_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplyTwoEighthsPrescaling
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_three_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplyThreeEighthsPrescaling
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_four_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplyFourEighthsPrescaling
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_five_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplyFiveEighthsPrescaling
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_six_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplySixEighthsPrescaling
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline(always)]
    pub fn is_supply_seven_eighths_prescaling(&self) -> bool {
        *self == Refsel::SupplySevenEighthsPrescaling
    }
    #[doc = "Use external analog reference as reference."]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == Refsel::Aref
    }
}
#[doc = "Field `REFSEL` writer - Reference select."]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refsel, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_one_eighth_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyOneEighthPrescaling)
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_two_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyTwoEighthsPrescaling)
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_three_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyThreeEighthsPrescaling)
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_four_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyFourEighthsPrescaling)
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_five_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyFiveEighthsPrescaling)
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_six_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplySixEighthsPrescaling)
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_seven_eighths_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplySevenEighthsPrescaling)
    }
    #[doc = "Use external analog reference as reference."]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Aref)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, RefselSpec> {
        RefselW::new(self, 0)
    }
}
#[doc = "Reference select.\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefselSpec;
impl crate::RegisterSpec for RefselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refsel::R`](R) reader structure"]
impl crate::Readable for RefselSpec {}
#[doc = "`write(|w| ..)` method takes [`refsel::W`](W) writer structure"]
impl crate::Writable for RefselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFSEL to value 0"]
impl crate::Resettable for RefselSpec {}
