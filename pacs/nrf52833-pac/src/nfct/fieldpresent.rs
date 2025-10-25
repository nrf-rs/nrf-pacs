#[doc = "Register `FIELDPRESENT` reader"]
pub type R = crate::R<FieldpresentSpec>;
#[doc = "Indicates if a valid field is present. Available only in the activated state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fieldpresent {
    #[doc = "0: No valid field detected"]
    NoField = 0,
    #[doc = "1: Valid field detected"]
    FieldPresent = 1,
}
impl From<Fieldpresent> for bool {
    #[inline(always)]
    fn from(variant: Fieldpresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDPRESENT` reader - Indicates if a valid field is present. Available only in the activated state."]
pub type FieldpresentR = crate::BitReader<Fieldpresent>;
impl FieldpresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fieldpresent {
        match self.bits {
            false => Fieldpresent::NoField,
            true => Fieldpresent::FieldPresent,
        }
    }
    #[doc = "No valid field detected"]
    #[inline(always)]
    pub fn is_no_field(&self) -> bool {
        *self == Fieldpresent::NoField
    }
    #[doc = "Valid field detected"]
    #[inline(always)]
    pub fn is_field_present(&self) -> bool {
        *self == Fieldpresent::FieldPresent
    }
}
#[doc = "Indicates if the low level has locked to the field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockdetect {
    #[doc = "0: Not locked to field"]
    NotLocked = 0,
    #[doc = "1: Locked to field"]
    Locked = 1,
}
impl From<Lockdetect> for bool {
    #[inline(always)]
    fn from(variant: Lockdetect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKDETECT` reader - Indicates if the low level has locked to the field"]
pub type LockdetectR = crate::BitReader<Lockdetect>;
impl LockdetectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockdetect {
        match self.bits {
            false => Lockdetect::NotLocked,
            true => Lockdetect::Locked,
        }
    }
    #[doc = "Not locked to field"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == Lockdetect::NotLocked
    }
    #[doc = "Locked to field"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockdetect::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if a valid field is present. Available only in the activated state."]
    #[inline(always)]
    pub fn fieldpresent(&self) -> FieldpresentR {
        FieldpresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the low level has locked to the field"]
    #[inline(always)]
    pub fn lockdetect(&self) -> LockdetectR {
        LockdetectR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Indicates the presence or not of a valid field\n\nYou can [`read`](crate::Reg::read) this register and get [`fieldpresent::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FieldpresentSpec;
impl crate::RegisterSpec for FieldpresentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fieldpresent::R`](R) reader structure"]
impl crate::Readable for FieldpresentSpec {}
#[doc = "`reset()` method sets FIELDPRESENT to value 0"]
impl crate::Resettable for FieldpresentSpec {}
