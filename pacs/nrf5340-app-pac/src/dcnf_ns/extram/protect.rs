#[doc = "Register `PROTECT` reader"]
pub struct R(crate::R<PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECT` writer"]
pub struct W(crate::W<PROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECT_SPEC>;
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
impl From<crate::W<PROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE0` reader - Control access to slave 0 of master EXTRAM\\[n\\]"]
pub type SLAVE0_R = crate::BitReader<SLAVE0_A>;
#[doc = "Control access to slave 0 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE0_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::ALLOWED,
            true => SLAVE0_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE0_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE0_A::BLOCKED
    }
}
#[doc = "Field `SLAVE0` writer - Control access to slave 0 of master EXTRAM\\[n\\]"]
pub type SLAVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE0_A, O>;
impl<'a, const O: u8> SLAVE0_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE0_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE0_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE1` reader - Control access to slave 1 of master EXTRAM\\[n\\]"]
pub type SLAVE1_R = crate::BitReader<SLAVE1_A>;
#[doc = "Control access to slave 1 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE1_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE1_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE1_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE1_A {
        match self.bits {
            false => SLAVE1_A::ALLOWED,
            true => SLAVE1_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE1_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE1_A::BLOCKED
    }
}
#[doc = "Field `SLAVE1` writer - Control access to slave 1 of master EXTRAM\\[n\\]"]
pub type SLAVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE1_A, O>;
impl<'a, const O: u8> SLAVE1_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE1_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE1_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE2` reader - Control access to slave 2 of master EXTRAM\\[n\\]"]
pub type SLAVE2_R = crate::BitReader<SLAVE2_A>;
#[doc = "Control access to slave 2 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE2_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE2_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE2_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE2_A {
        match self.bits {
            false => SLAVE2_A::ALLOWED,
            true => SLAVE2_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE2_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE2_A::BLOCKED
    }
}
#[doc = "Field `SLAVE2` writer - Control access to slave 2 of master EXTRAM\\[n\\]"]
pub type SLAVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE2_A, O>;
impl<'a, const O: u8> SLAVE2_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE2_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE2_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE3` reader - Control access to slave 3 of master EXTRAM\\[n\\]"]
pub type SLAVE3_R = crate::BitReader<SLAVE3_A>;
#[doc = "Control access to slave 3 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE3_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE3_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE3_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE3_A {
        match self.bits {
            false => SLAVE3_A::ALLOWED,
            true => SLAVE3_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE3_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE3_A::BLOCKED
    }
}
#[doc = "Field `SLAVE3` writer - Control access to slave 3 of master EXTRAM\\[n\\]"]
pub type SLAVE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE3_A, O>;
impl<'a, const O: u8> SLAVE3_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE3_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE3_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE4` reader - Control access to slave 4 of master EXTRAM\\[n\\]"]
pub type SLAVE4_R = crate::BitReader<SLAVE4_A>;
#[doc = "Control access to slave 4 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE4_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE4_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE4_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE4_A {
        match self.bits {
            false => SLAVE4_A::ALLOWED,
            true => SLAVE4_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE4_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE4_A::BLOCKED
    }
}
#[doc = "Field `SLAVE4` writer - Control access to slave 4 of master EXTRAM\\[n\\]"]
pub type SLAVE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE4_A, O>;
impl<'a, const O: u8> SLAVE4_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE4_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE4_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE5` reader - Control access to slave 5 of master EXTRAM\\[n\\]"]
pub type SLAVE5_R = crate::BitReader<SLAVE5_A>;
#[doc = "Control access to slave 5 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE5_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE5_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE5_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE5_A {
        match self.bits {
            false => SLAVE5_A::ALLOWED,
            true => SLAVE5_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE5_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE5_A::BLOCKED
    }
}
#[doc = "Field `SLAVE5` writer - Control access to slave 5 of master EXTRAM\\[n\\]"]
pub type SLAVE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE5_A, O>;
impl<'a, const O: u8> SLAVE5_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE5_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE5_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE6` reader - Control access to slave 6 of master EXTRAM\\[n\\]"]
pub type SLAVE6_R = crate::BitReader<SLAVE6_A>;
#[doc = "Control access to slave 6 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE6_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE6_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE6_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE6_A {
        match self.bits {
            false => SLAVE6_A::ALLOWED,
            true => SLAVE6_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE6_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE6_A::BLOCKED
    }
}
#[doc = "Field `SLAVE6` writer - Control access to slave 6 of master EXTRAM\\[n\\]"]
pub type SLAVE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE6_A, O>;
impl<'a, const O: u8> SLAVE6_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE6_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE6_A::BLOCKED)
    }
}
#[doc = "Field `SLAVE7` reader - Control access to slave 7 of master EXTRAM\\[n\\]"]
pub type SLAVE7_R = crate::BitReader<SLAVE7_A>;
#[doc = "Control access to slave 7 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE7_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED = 0,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED = 1,
}
impl From<SLAVE7_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE7_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE7_A {
        match self.bits {
            false => SLAVE7_A::ALLOWED,
            true => SLAVE7_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE7_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE7_A::BLOCKED
    }
}
#[doc = "Field `SLAVE7` writer - Control access to slave 7 of master EXTRAM\\[n\\]"]
pub type SLAVE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROTECT_SPEC, SLAVE7_A, O>;
impl<'a, const O: u8> SLAVE7_W<'a, O> {
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE7_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE7_A::BLOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&self) -> SLAVE1_R {
        SLAVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&self) -> SLAVE2_R {
        SLAVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&self) -> SLAVE3_R {
        SLAVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&self) -> SLAVE4_R {
        SLAVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&self) -> SLAVE5_R {
        SLAVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&self) -> SLAVE6_R {
        SLAVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&self) -> SLAVE7_R {
        SLAVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> SLAVE0_W<0> {
        SLAVE0_W::new(self)
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&mut self) -> SLAVE1_W<1> {
        SLAVE1_W::new(self)
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&mut self) -> SLAVE2_W<2> {
        SLAVE2_W::new(self)
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&mut self) -> SLAVE3_W<3> {
        SLAVE3_W::new(self)
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&mut self) -> SLAVE4_W<4> {
        SLAVE4_W::new(self)
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&mut self) -> SLAVE5_W<5> {
        SLAVE5_W::new(self)
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&mut self) -> SLAVE6_W<6> {
        SLAVE6_W::new(self)
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&mut self) -> SLAVE7_W<7> {
        SLAVE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protect](index.html) module"]
pub struct PROTECT_SPEC;
impl crate::RegisterSpec for PROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protect::R](R) reader structure"]
impl crate::Readable for PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protect::W](W) writer structure"]
impl crate::Writable for PROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for PROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
