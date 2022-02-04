#[doc = "Register `LIST` reader"]
pub struct R(crate::R<LIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIST` writer"]
pub struct W(crate::W<LIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIST_SPEC>;
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
impl From<crate::W<LIST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "List type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LIST_A {
    #[doc = "0: Disable EasyDMA list"]
    DISABLED = 0,
    #[doc = "1: Use array list"]
    ARRAYLIST = 1,
}
impl From<LIST_A> for u8 {
    #[inline(always)]
    fn from(variant: LIST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LIST` reader - List type"]
pub struct LIST_R(crate::FieldReader<u8, LIST_A>);
impl LIST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LIST_A> {
        match self.bits {
            0 => Some(LIST_A::DISABLED),
            1 => Some(LIST_A::ARRAYLIST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LIST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ARRAYLIST`"]
    #[inline(always)]
    pub fn is_array_list(&self) -> bool {
        **self == LIST_A::ARRAYLIST
    }
}
impl core::ops::Deref for LIST_R {
    type Target = crate::FieldReader<u8, LIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIST` writer - List type"]
pub struct LIST_W<'a> {
    w: &'a mut W,
}
impl<'a> LIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIST_A::DISABLED)
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn array_list(self) -> &'a mut W {
        self.variant(LIST_A::ARRAYLIST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn list(&self) -> LIST_R {
        LIST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn list(&mut self) -> LIST_W {
        LIST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EasyDMA list type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [list](index.html) module"]
pub struct LIST_SPEC;
impl crate::RegisterSpec for LIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [list::R](R) reader structure"]
impl crate::Readable for LIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [list::W](W) writer structure"]
impl crate::Writable for LIST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIST to value 0"]
impl crate::Resettable for LIST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
