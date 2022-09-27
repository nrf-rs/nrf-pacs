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
#[doc = "Field `WRITE` reader - Configure write and erase permissions for region n. Write '0' has no effect."]
pub type WRITE_R = crate::BitReader<WRITE_A>;
#[doc = "Configure write and erase permissions for region n. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "0: Allow write and erase instructions to region n"]
    ENABLE = 0,
    #[doc = "1: Block write and erase instructions to region n"]
    DISABLE = 1,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::ENABLE,
            true => WRITE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WRITE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WRITE_A::DISABLE
    }
}
#[doc = "Field `WRITE` writer - Configure write and erase permissions for region n. Write '0' has no effect."]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, WRITE_A, O>;
impl<'a, const O: u8> WRITE_W<'a, O> {
    #[doc = "Allow write and erase instructions to region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLE)
    }
    #[doc = "Block write and erase instructions to region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLE)
    }
}
#[doc = "Field `READ` reader - Configure read permissions for region n. Write '0' has no effect."]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "Configure read permissions for region n. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Allow read instructions to region n"]
    ENABLE = 0,
    #[doc = "1: Block read instructions to region n"]
    DISABLE = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::ENABLE,
            true => READ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == READ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == READ_A::DISABLE
    }
}
#[doc = "Field `READ` writer - Configure read permissions for region n. Write '0' has no effect."]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, READ_A, O>;
impl<'a, const O: u8> READ_W<'a, O> {
    #[doc = "Allow read instructions to region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(READ_A::ENABLE)
    }
    #[doc = "Block read instructions to region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(READ_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Configure write and erase permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Configure write and erase permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Configure read permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<2> {
        READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
#[doc = "`reset()` method sets PERM to value 0"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
