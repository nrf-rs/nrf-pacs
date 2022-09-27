#[doc = "Register `DTOGGLE` reader"]
pub struct R(crate::R<DTOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTOGGLE` writer"]
pub struct W(crate::W<DTOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTOGGLE_SPEC>;
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
impl From<crate::W<DTOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP` reader - Select bulk endpoint number"]
pub type EP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP` writer - Select bulk endpoint number"]
pub type EP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTOGGLE_SPEC, u8, u8, 3, O>;
#[doc = "Field `IO` reader - Selects IN or OUT endpoint"]
pub type IO_R = crate::BitReader<IO_A>;
#[doc = "Selects IN or OUT endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_A {
    #[doc = "0: Selects OUT endpoint"]
    OUT = 0,
    #[doc = "1: Selects IN endpoint"]
    IN = 1,
}
impl From<IO_A> for bool {
    #[inline(always)]
    fn from(variant: IO_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_A {
        match self.bits {
            false => IO_A::OUT,
            true => IO_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == IO_A::IN
    }
}
#[doc = "Field `IO` writer - Selects IN or OUT endpoint"]
pub type IO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTOGGLE_SPEC, IO_A, O>;
impl<'a, const O: u8> IO_W<'a, O> {
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO_A::OUT)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO_A::IN)
    }
}
#[doc = "Field `VALUE` reader - Data toggle value"]
pub type VALUE_R = crate::FieldReader<u8, VALUE_A>;
#[doc = "Data toggle value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VALUE_A {
    #[doc = "0: No action on data toggle when writing the register with this value"]
    NOP = 0,
    #[doc = "1: Data toggle is DATA0 on endpoint set by EP and IO"]
    DATA0 = 1,
    #[doc = "2: Data toggle is DATA1 on endpoint set by EP and IO"]
    DATA1 = 2,
}
impl From<VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as _
    }
}
impl VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VALUE_A> {
        match self.bits {
            0 => Some(VALUE_A::NOP),
            1 => Some(VALUE_A::DATA0),
            2 => Some(VALUE_A::DATA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == VALUE_A::NOP
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == VALUE_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == VALUE_A::DATA1
    }
}
#[doc = "Field `VALUE` writer - Data toggle value"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTOGGLE_SPEC, u8, VALUE_A, 2, O>;
impl<'a, const O: u8> VALUE_W<'a, O> {
    #[doc = "No action on data toggle when writing the register with this value"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(VALUE_A::NOP)
    }
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut W {
        self.variant(VALUE_A::DATA0)
    }
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut W {
        self.variant(VALUE_A::DATA1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&self) -> IO_R {
        IO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<0> {
        EP_W::new(self)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IO_W<7> {
        IO_W::new(self)
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W<8> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data toggle control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtoggle](index.html) module"]
pub struct DTOGGLE_SPEC;
impl crate::RegisterSpec for DTOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtoggle::R](R) reader structure"]
impl crate::Readable for DTOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtoggle::W](W) writer structure"]
impl crate::Writable for DTOGGLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTOGGLE to value 0x0100"]
impl crate::Resettable for DTOGGLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
