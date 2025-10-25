#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<Cidr0Spec>;
#[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code.\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmbl0 {
    #[doc = "13: Bits\\[7:0\\] of the identification code."]
    Value = 13,
}
impl From<Prmbl0> for u8 {
    #[inline(always)]
    fn from(variant: Prmbl0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmbl0 {
    type Ux = u8;
}
impl crate::IsEnum for Prmbl0 {}
#[doc = "Field `PRMBL_0` reader - Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
pub type Prmbl0R = crate::FieldReader<Prmbl0>;
impl Prmbl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmbl0> {
        match self.bits {
            13 => Some(Prmbl0::Value),
            _ => None,
        }
    }
    #[doc = "Bits\\[7:0\\] of the identification code."]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == Prmbl0::Value
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
    #[inline(always)]
    pub fn prmbl_0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr0Spec;
impl crate::RegisterSpec for Cidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for Cidr0Spec {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for Cidr0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
