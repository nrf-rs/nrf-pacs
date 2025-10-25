#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<Cidr1Spec>;
#[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmbl1 {
    #[doc = "0: Bits\\[11:8\\] of the identification code."]
    Value = 0,
}
impl From<Prmbl1> for u8 {
    #[inline(always)]
    fn from(variant: Prmbl1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmbl1 {
    type Ux = u8;
}
impl crate::IsEnum for Prmbl1 {}
#[doc = "Field `PRMBL_1` reader - Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
pub type Prmbl1R = crate::FieldReader<Prmbl1>;
impl Prmbl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmbl1> {
        match self.bits {
            0 => Some(Prmbl1::Value),
            _ => None,
        }
    }
    #[doc = "Bits\\[11:8\\] of the identification code."]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == Prmbl1::Value
    }
}
#[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Class {
    #[doc = "9: Indicates that the component is a CoreSight component."]
    Coresight = 9,
}
impl From<Class> for u8 {
    #[inline(always)]
    fn from(variant: Class) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Class {
    type Ux = u8;
}
impl crate::IsEnum for Class {}
#[doc = "Field `CLASS` reader - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
pub type ClassR = crate::FieldReader<Class>;
impl ClassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Class> {
        match self.bits {
            9 => Some(Class::Coresight),
            _ => None,
        }
    }
    #[doc = "Indicates that the component is a CoreSight component."]
    #[inline(always)]
    pub fn is_coresight(&self) -> bool {
        *self == Class::Coresight
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
    #[inline(always)]
    pub fn prmbl_1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr1Spec;
impl crate::RegisterSpec for Cidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for Cidr1Spec {}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for Cidr1Spec {
    const RESET_VALUE: u32 = 0x90;
}
