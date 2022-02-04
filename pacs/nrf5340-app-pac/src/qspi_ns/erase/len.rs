#[doc = "Register `LEN` reader"]
pub struct R(crate::R<LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEN` writer"]
pub struct W(crate::W<LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEN_SPEC>;
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
impl From<crate::W<LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: Erase 4 kB block (flash command 0x20)"]
    _4KB = 0,
    #[doc = "1: Erase 64 kB block (flash command 0xD8)"]
    _64KB = 1,
    #[doc = "2: Erase all (flash command 0xC7)"]
    ALL = 2,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEN` reader - LEN"]
pub struct LEN_R(crate::FieldReader<u8, LEN_A>);
impl LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEN_A> {
        match self.bits {
            0 => Some(LEN_A::_4KB),
            1 => Some(LEN_A::_64KB),
            2 => Some(LEN_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        **self == LEN_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        **self == LEN_A::_64KB
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == LEN_A::ALL
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u8, LEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - LEN"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(LEN_A::_4KB)
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(LEN_A::_64KB)
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(LEN_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of block to be erased.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [len](index.html) module"]
pub struct LEN_SPEC;
impl crate::RegisterSpec for LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [len::R](R) reader structure"]
impl crate::Readable for LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [len::W](W) writer structure"]
impl crate::Writable for LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEN to value 0"]
impl crate::Resettable for LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
