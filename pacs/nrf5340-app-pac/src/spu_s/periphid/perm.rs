#[doc = "Register `PERM` reader"]
pub struct R(crate::R<PERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERM` writer"]
pub struct W(crate::W<PERM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERM_SPEC>;
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
impl From<crate::W<PERM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECUREMAPPING` reader - Define configuration capabilities for Arm TrustZone Cortex-M secure attribute"]
pub type SECUREMAPPING_R = crate::FieldReader<u8, SECUREMAPPING_A>;
#[doc = "Define configuration capabilities for Arm TrustZone Cortex-M secure attribute\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECUREMAPPING_A {
    #[doc = "0: This peripheral is always accessible as a non-secure peripheral"]
    NON_SECURE = 0,
    #[doc = "1: This peripheral is always accessible as a secure peripheral"]
    SECURE = 1,
    #[doc = "2: Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    USER_SELECTABLE = 2,
    #[doc = "3: This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    SPLIT = 3,
}
impl From<SECUREMAPPING_A> for u8 {
    #[inline(always)]
    fn from(variant: SECUREMAPPING_A) -> Self {
        variant as _
    }
}
impl SECUREMAPPING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREMAPPING_A {
        match self.bits {
            0 => SECUREMAPPING_A::NON_SECURE,
            1 => SECUREMAPPING_A::SECURE,
            2 => SECUREMAPPING_A::USER_SELECTABLE,
            3 => SECUREMAPPING_A::SPLIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECUREMAPPING_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECUREMAPPING_A::SECURE
    }
    #[doc = "Checks if the value of the field is `USER_SELECTABLE`"]
    #[inline(always)]
    pub fn is_user_selectable(&self) -> bool {
        *self == SECUREMAPPING_A::USER_SELECTABLE
    }
    #[doc = "Checks if the value of the field is `SPLIT`"]
    #[inline(always)]
    pub fn is_split(&self) -> bool {
        *self == SECUREMAPPING_A::SPLIT
    }
}
#[doc = "Field `DMA` reader - Indicates if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
pub type DMA_R = crate::FieldReader<u8, DMA_A>;
#[doc = "Indicates if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "0: Peripheral has no DMA capability"]
    NO_DMA = 0,
    #[doc = "1: Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    NO_SEPARATE_ATTRIBUTE = 1,
    #[doc = "2: Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    SEPARATE_ATTRIBUTE = 2,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA_A> {
        match self.bits {
            0 => Some(DMA_A::NO_DMA),
            1 => Some(DMA_A::NO_SEPARATE_ATTRIBUTE),
            2 => Some(DMA_A::SEPARATE_ATTRIBUTE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DMA`"]
    #[inline(always)]
    pub fn is_no_dma(&self) -> bool {
        *self == DMA_A::NO_DMA
    }
    #[doc = "Checks if the value of the field is `NO_SEPARATE_ATTRIBUTE`"]
    #[inline(always)]
    pub fn is_no_separate_attribute(&self) -> bool {
        *self == DMA_A::NO_SEPARATE_ATTRIBUTE
    }
    #[doc = "Checks if the value of the field is `SEPARATE_ATTRIBUTE`"]
    #[inline(always)]
    pub fn is_separate_attribute(&self) -> bool {
        *self == DMA_A::SEPARATE_ATTRIBUTE
    }
}
#[doc = "Field `SECATTR` reader - Peripheral security mapping"]
pub type SECATTR_R = crate::BitReader<SECATTR_A>;
#[doc = "Peripheral security mapping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "1: Peripheral is mapped in secure peripheral address space"]
    SECURE = 1,
    #[doc = "0: If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NON_SECURE = 0,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SECATTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECATTR_A {
        match self.bits {
            true => SECATTR_A::SECURE,
            false => SECATTR_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECATTR_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTR_A::NON_SECURE
    }
}
#[doc = "Field `SECATTR` writer - Peripheral security mapping"]
pub type SECATTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, SECATTR_A, O>;
impl<'a, const O: u8> SECATTR_W<'a, O> {
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTR_A::SECURE)
    }
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NON_SECURE)
    }
}
#[doc = "Field `DMASEC` reader - Security attribution for the DMA transfer"]
pub type DMASEC_R = crate::BitReader<DMASEC_A>;
#[doc = "Security attribution for the DMA transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASEC_A {
    #[doc = "1: DMA transfers initiated by this peripheral have the secure attribute set"]
    SECURE = 1,
    #[doc = "0: DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<DMASEC_A> for bool {
    #[inline(always)]
    fn from(variant: DMASEC_A) -> Self {
        variant as u8 != 0
    }
}
impl DMASEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASEC_A {
        match self.bits {
            true => DMASEC_A::SECURE,
            false => DMASEC_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == DMASEC_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == DMASEC_A::NON_SECURE
    }
}
#[doc = "Field `DMASEC` writer - Security attribution for the DMA transfer"]
pub type DMASEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, DMASEC_A, O>;
impl<'a, const O: u8> DMASEC_W<'a, O> {
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(DMASEC_A::SECURE)
    }
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(DMASEC_A::NON_SECURE)
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
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, LOCK_A, O>;
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
#[doc = "Field `PRESENT` reader - Indicate if a peripheral is present with ID n"]
pub type PRESENT_R = crate::BitReader<PRESENT_A>;
#[doc = "Indicate if a peripheral is present with ID n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_A {
    #[doc = "0: Peripheral is not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Peripheral is present"]
    IS_PRESENT = 1,
}
impl From<PRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl PRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_A {
        match self.bits {
            false => PRESENT_A::NOT_PRESENT,
            true => PRESENT_A::IS_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `IS_PRESENT`"]
    #[inline(always)]
    pub fn is_is_present(&self) -> bool {
        *self == PRESENT_A::IS_PRESENT
    }
}
impl R {
    #[doc = "Bits 0:1 - Define configuration capabilities for Arm TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SECUREMAPPING_R {
        SECUREMAPPING_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicates if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SECATTR_R {
        SECATTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn dmasec(&self) -> DMASEC_R {
        DMASEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicate if a peripheral is present with ID n"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SECATTR_W<4> {
        SECATTR_W::new(self)
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn dmasec(&mut self) -> DMASEC_W<5> {
        DMASEC_W::new(self)
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
#[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
pub struct PERM_SPEC;
impl crate::RegisterSpec for PERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perm::R](R) reader structure"]
impl crate::Readable for PERM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perm::W](W) writer structure"]
impl crate::Writable for PERM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERM to value 0x12"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12
    }
}
