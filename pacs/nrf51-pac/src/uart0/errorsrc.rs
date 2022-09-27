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
#[doc = "Field `OVERRUN` reader - A start bit is received while the previous data still lies in RXD. (Data loss)."]
pub type OVERRUN_R = crate::BitReader<OVERRUN_A>;
#[doc = "A start bit is received while the previous data still lies in RXD. (Data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_A {
    #[doc = "0: Error not present."]
    NOT_PRESENT = 0,
    #[doc = "1: Error present."]
    PRESENT = 1,
}
impl From<OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_A {
        match self.bits {
            false => OVERRUN_A::NOT_PRESENT,
            true => OVERRUN_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERRUN_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERRUN_A::PRESENT
    }
}
#[doc = "A start bit is received while the previous data still lies in RXD. (Data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_AW {
    #[doc = "1: Clear error on write."]
    CLEAR = 1,
}
impl From<OVERRUN_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` writer - A start bit is received while the previous data still lies in RXD. (Data loss)."]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRORSRC_SPEC, OVERRUN_AW, O>;
impl<'a, const O: u8> OVERRUN_W<'a, O> {
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERRUN_AW::CLEAR)
    }
}
#[doc = "Field `PARITY` reader - A character with bad parity is received. Only checked if HW parity control is enabled."]
pub type PARITY_R = crate::BitReader<PARITY_A>;
#[doc = "A character with bad parity is received. Only checked if HW parity control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_A {
    #[doc = "0: Error not present."]
    NOT_PRESENT = 0,
    #[doc = "1: Error present."]
    PRESENT = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            false => PARITY_A::NOT_PRESENT,
            true => PARITY_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PARITY_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PARITY_A::PRESENT
    }
}
#[doc = "A character with bad parity is received. Only checked if HW parity control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_AW {
    #[doc = "1: Clear error on write."]
    CLEAR = 1,
}
impl From<PARITY_AW> for bool {
    #[inline(always)]
    fn from(variant: PARITY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` writer - A character with bad parity is received. Only checked if HW parity control is enabled."]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRORSRC_SPEC, PARITY_AW, O>;
impl<'a, const O: u8> PARITY_W<'a, O> {
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PARITY_AW::CLEAR)
    }
}
#[doc = "Field `FRAMING` reader - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
pub type FRAMING_R = crate::BitReader<FRAMING_A>;
#[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMING_A {
    #[doc = "0: Error not present."]
    NOT_PRESENT = 0,
    #[doc = "1: Error present."]
    PRESENT = 1,
}
impl From<FRAMING_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMING_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAMING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMING_A {
        match self.bits {
            false => FRAMING_A::NOT_PRESENT,
            true => FRAMING_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == FRAMING_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == FRAMING_A::PRESENT
    }
}
#[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMING_AW {
    #[doc = "1: Clear error on write."]
    CLEAR = 1,
}
impl From<FRAMING_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAMING_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMING` writer - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
pub type FRAMING_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRORSRC_SPEC, FRAMING_AW, O>;
impl<'a, const O: u8> FRAMING_W<'a, O> {
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRAMING_AW::CLEAR)
    }
}
#[doc = "Field `BREAK` reader - The serial data input is '0' for longer than the length of a data frame."]
pub type BREAK_R = crate::BitReader<BREAK_A>;
#[doc = "The serial data input is '0' for longer than the length of a data frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAK_A {
    #[doc = "0: Error not present."]
    NOT_PRESENT = 0,
    #[doc = "1: Error present."]
    PRESENT = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
impl BREAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::NOT_PRESENT,
            true => BREAK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == BREAK_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == BREAK_A::PRESENT
    }
}
#[doc = "The serial data input is '0' for longer than the length of a data frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAK_AW {
    #[doc = "1: Clear error on write."]
    CLEAR = 1,
}
impl From<BREAK_AW> for bool {
    #[inline(always)]
    fn from(variant: BREAK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREAK` writer - The serial data input is '0' for longer than the length of a data frame."]
pub type BREAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRORSRC_SPEC, BREAK_AW, O>;
impl<'a, const O: u8> BREAK_W<'a, O> {
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BREAK_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline(always)]
    pub fn framing(&self) -> FRAMING_R {
        FRAMING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The serial data input is '0' for longer than the length of a data frame."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W<0> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 1 - A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<1> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 2 - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline(always)]
    pub fn framing(&mut self) -> FRAMING_W<2> {
        FRAMING_W::new(self)
    }
    #[doc = "Bit 3 - The serial data input is '0' for longer than the length of a data frame."]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W<3> {
        BREAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error source. Write error field to 1 to clear error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](index.html) module"]
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
