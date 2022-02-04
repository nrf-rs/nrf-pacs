#[doc = "Register `ERRORSRC` reader"]
pub struct R(crate::R<ERRORSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRORSRC` writer"]
pub struct W(crate::W<ERRORSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRORSRC_SPEC>;
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
impl From<crate::W<ERRORSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRORSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_A {
    #[doc = "0: Read: no overrun occured"]
    NOTPRESENT = 0,
    #[doc = "1: Read: overrun occured"]
    PRESENT = 1,
}
impl From<OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Overrun error"]
pub struct OVERRUN_R(crate::FieldReader<bool, OVERRUN_A>);
impl OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_A {
        match self.bits {
            false => OVERRUN_A::NOTPRESENT,
            true => OVERRUN_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == OVERRUN_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == OVERRUN_A::PRESENT
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<bool, OVERRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN` writer - Overrun error"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERRUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read: no overrun occured"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(OVERRUN_A::NOTPRESENT)
    }
    #[doc = "Read: overrun occured"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(OVERRUN_A::PRESENT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACK_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - NACK received after sending the address (write '1' to clear)"]
pub struct ANACK_R(crate::FieldReader<bool, ANACK_A>);
impl ANACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::NOTPRESENT,
            true => ANACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == ANACK_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == ANACK_A::PRESENT
    }
}
impl core::ops::Deref for ANACK_R {
    type Target = crate::FieldReader<bool, ANACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANACK` writer - NACK received after sending the address (write '1' to clear)"]
pub struct ANACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(ANACK_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(ANACK_A::PRESENT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACK_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<DNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - NACK received after sending a data byte (write '1' to clear)"]
pub struct DNACK_R(crate::FieldReader<bool, DNACK_A>);
impl DNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNACK_A {
        match self.bits {
            false => DNACK_A::NOTPRESENT,
            true => DNACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == DNACK_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == DNACK_A::PRESENT
    }
}
impl core::ops::Deref for DNACK_R {
    type Target = crate::FieldReader<bool, DNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNACK` writer - NACK received after sending a data byte (write '1' to clear)"]
pub struct DNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(DNACK_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(DNACK_A::PRESENT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&mut self) -> ANACK_W {
        ANACK_W { w: self }
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DNACK_W {
        DNACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](index.html) module"]
pub struct ERRORSRC_SPEC;
impl crate::RegisterSpec for ERRORSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorsrc::R](R) reader structure"]
impl crate::Readable for ERRORSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](W) writer structure"]
impl crate::Writable for ERRORSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ERRORSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
