#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<Pidr0Spec>;
#[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number.\n\nValue on reset: 33"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Part0 {
    #[doc = "33: Indicates bits\\[7:0\\] of the part number of the component."]
    PartnumberL = 33,
}
impl From<Part0> for u8 {
    #[inline(always)]
    fn from(variant: Part0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Part0 {
    type Ux = u8;
}
impl crate::IsEnum for Part0 {}
#[doc = "Field `PART_0` reader - Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub type Part0R = crate::FieldReader<Part0>;
impl Part0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Part0> {
        match self.bits {
            33 => Some(Part0::PartnumberL),
            _ => None,
        }
    }
    #[doc = "Indicates bits\\[7:0\\] of the part number of the component."]
    #[inline(always)]
    pub fn is_partnumber_l(&self) -> bool {
        *self == Part0::PartnumberL
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr0Spec;
impl crate::RegisterSpec for Pidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for Pidr0Spec {}
#[doc = "`reset()` method sets PIDR0 to value 0x21"]
impl crate::Resettable for Pidr0Spec {
    const RESET_VALUE: u32 = 0x21;
}
