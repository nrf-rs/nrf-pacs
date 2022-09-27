#[doc = "Register `SIZE` reader"]
pub struct R(crate::R<SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZE` writer"]
pub struct W(crate::W<SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZE_SPEC>;
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
impl From<crate::W<SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Size of the non-secure callable (NSC) region n"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Size of the non-secure callable (NSC) region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED = 0,
    #[doc = "1: The region n is defined as non-secure callable with size 32 bytes"]
    _32 = 1,
    #[doc = "2: The region n is defined as non-secure callable with size 64 bytes"]
    _64 = 2,
    #[doc = "3: The region n is defined as non-secure callable with size 128 bytes"]
    _128 = 3,
    #[doc = "4: The region n is defined as non-secure callable with size 256 bytes"]
    _256 = 4,
    #[doc = "5: The region n is defined as non-secure callable with size 512 bytes"]
    _512 = 5,
    #[doc = "6: The region n is defined as non-secure callable with size 1024 bytes"]
    _1024 = 6,
    #[doc = "7: The region n is defined as non-secure callable with size 2048 bytes"]
    _2048 = 7,
    #[doc = "8: The region n is defined as non-secure callable with size 4096 bytes"]
    _4096 = 8,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::DISABLED),
            1 => Some(SIZE_A::_32),
            2 => Some(SIZE_A::_64),
            3 => Some(SIZE_A::_128),
            4 => Some(SIZE_A::_256),
            5 => Some(SIZE_A::_512),
            6 => Some(SIZE_A::_1024),
            7 => Some(SIZE_A::_2048),
            8 => Some(SIZE_A::_4096),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SIZE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SIZE_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == SIZE_A::_2048
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == SIZE_A::_4096
    }
}
#[doc = "Field `SIZE` writer - Size of the non-secure callable (NSC) region n"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIZE_SPEC, u8, SIZE_A, 4, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SIZE_A::DISABLED)
    }
    #[doc = "The region n is defined as non-secure callable with size 32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SIZE_A::_32)
    }
    #[doc = "The region n is defined as non-secure callable with size 64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SIZE_A::_64)
    }
    #[doc = "The region n is defined as non-secure callable with size 128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SIZE_A::_128)
    }
    #[doc = "The region n is defined as non-secure callable with size 256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SIZE_A::_256)
    }
    #[doc = "The region n is defined as non-secure callable with size 512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SIZE_A::_512)
    }
    #[doc = "The region n is defined as non-secure callable with size 1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SIZE_A::_1024)
    }
    #[doc = "The region n is defined as non-secure callable with size 2048 bytes"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SIZE_A::_2048)
    }
    #[doc = "The region n is defined as non-secure callable with size 4096 bytes"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(SIZE_A::_4096)
    }
}
#[doc = "Field `LOCK` reader - "]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This register can be updated"]
    UNLOCKED = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCK` writer - "]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIZE_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<0> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size](index.html) module"]
pub struct SIZE_SPEC;
impl crate::RegisterSpec for SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [size::R](R) reader structure"]
impl crate::Readable for SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [size::W](W) writer structure"]
impl crate::Writable for SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIZE to value 0"]
impl crate::Resettable for SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
