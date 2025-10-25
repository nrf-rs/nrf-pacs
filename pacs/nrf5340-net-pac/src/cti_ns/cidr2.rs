#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<Cidr2Spec>;
#[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmbl2 {
    #[doc = "5: Bits\\[23:16\\] of the identification code."]
    Value = 5,
}
impl From<Prmbl2> for u8 {
    #[inline(always)]
    fn from(variant: Prmbl2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmbl2 {
    type Ux = u8;
}
impl crate::IsEnum for Prmbl2 {}
#[doc = "Field `PRMBL_2` reader - Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
pub type Prmbl2R = crate::FieldReader<Prmbl2>;
impl Prmbl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmbl2> {
        match self.bits {
            5 => Some(Prmbl2::Value),
            _ => None,
        }
    }
    #[doc = "Bits\\[23:16\\] of the identification code."]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == Prmbl2::Value
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
    #[inline(always)]
    pub fn prmbl_2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr2Spec;
impl crate::RegisterSpec for Cidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for Cidr2Spec {}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for Cidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
