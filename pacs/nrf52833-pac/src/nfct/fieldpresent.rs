#[doc = "Register `FIELDPRESENT` reader"]
pub struct R(crate::R<FIELDPRESENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIELDPRESENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIELDPRESENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIELDPRESENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates if a valid field is present. Available only in the activated state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDPRESENT_A {
    #[doc = "0: No valid field detected"]
    NOFIELD = 0,
    #[doc = "1: Valid field detected"]
    FIELDPRESENT = 1,
}
impl From<FIELDPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDPRESENT` reader - Indicates if a valid field is present. Available only in the activated state."]
pub struct FIELDPRESENT_R(crate::FieldReader<bool, FIELDPRESENT_A>);
impl FIELDPRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIELDPRESENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDPRESENT_A {
        match self.bits {
            false => FIELDPRESENT_A::NOFIELD,
            true => FIELDPRESENT_A::FIELDPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFIELD`"]
    #[inline(always)]
    pub fn is_no_field(&self) -> bool {
        **self == FIELDPRESENT_A::NOFIELD
    }
    #[doc = "Checks if the value of the field is `FIELDPRESENT`"]
    #[inline(always)]
    pub fn is_field_present(&self) -> bool {
        **self == FIELDPRESENT_A::FIELDPRESENT
    }
}
impl core::ops::Deref for FIELDPRESENT_R {
    type Target = crate::FieldReader<bool, FIELDPRESENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates if the low level has locked to the field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKDETECT_A {
    #[doc = "0: Not locked to field"]
    NOTLOCKED = 0,
    #[doc = "1: Locked to field"]
    LOCKED = 1,
}
impl From<LOCKDETECT_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKDETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKDETECT` reader - Indicates if the low level has locked to the field"]
pub struct LOCKDETECT_R(crate::FieldReader<bool, LOCKDETECT_A>);
impl LOCKDETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKDETECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKDETECT_A {
        match self.bits {
            false => LOCKDETECT_A::NOTLOCKED,
            true => LOCKDETECT_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        **self == LOCKDETECT_A::NOTLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCKDETECT_A::LOCKED
    }
}
impl core::ops::Deref for LOCKDETECT_R {
    type Target = crate::FieldReader<bool, LOCKDETECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if a valid field is present. Available only in the activated state."]
    #[inline(always)]
    pub fn fieldpresent(&self) -> FIELDPRESENT_R {
        FIELDPRESENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the low level has locked to the field"]
    #[inline(always)]
    pub fn lockdetect(&self) -> LOCKDETECT_R {
        LOCKDETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Indicates the presence or not of a valid field\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fieldpresent](index.html) module"]
pub struct FIELDPRESENT_SPEC;
impl crate::RegisterSpec for FIELDPRESENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fieldpresent::R](R) reader structure"]
impl crate::Readable for FIELDPRESENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIELDPRESENT to value 0"]
impl crate::Resettable for FIELDPRESENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
