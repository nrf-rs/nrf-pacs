#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<Pidr1Spec>;
#[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number.\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Part1 {
    #[doc = "13: Indicates bits\\[11:8\\] of the part number of the component."]
    PartnumberH = 13,
}
impl From<Part1> for u8 {
    #[inline(always)]
    fn from(variant: Part1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Part1 {
    type Ux = u8;
}
impl crate::IsEnum for Part1 {}
#[doc = "Field `PART_1` reader - Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub type Part1R = crate::FieldReader<Part1>;
impl Part1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Part1> {
        match self.bits {
            13 => Some(Part1::PartnumberH),
            _ => None,
        }
    }
    #[doc = "Indicates bits\\[11:8\\] of the part number of the component."]
    #[inline(always)]
    pub fn is_partnumber_h(&self) -> bool {
        *self == Part1::PartnumberH
    }
}
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Des0 {
    #[doc = "11: Arm. Bits\\[3:0\\] of the JEDEC JEP106 Identity Code"]
    Arm = 11,
}
impl From<Des0> for u8 {
    #[inline(always)]
    fn from(variant: Des0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Des0 {
    type Ux = u8;
}
impl crate::IsEnum for Des0 {}
#[doc = "Field `DES_0` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type Des0R = crate::FieldReader<Des0>;
impl Des0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Des0> {
        match self.bits {
            11 => Some(Des0::Arm),
            _ => None,
        }
    }
    #[doc = "Arm. Bits\\[3:0\\] of the JEDEC JEP106 Identity Code"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == Des0::Arm
    }
}
impl R {
    #[doc = "Bits 0:3 - Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr1Spec;
impl crate::RegisterSpec for Pidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for Pidr1Spec {}
#[doc = "`reset()` method sets PIDR1 to value 0xbd"]
impl crate::Resettable for Pidr1Spec {
    const RESET_VALUE: u32 = 0xbd;
}
