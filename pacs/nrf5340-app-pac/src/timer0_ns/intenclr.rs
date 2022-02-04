#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub struct COMPARE0_R(crate::FieldReader<bool, COMPARE0_A>);
impl COMPARE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_A {
        match self.bits {
            false => COMPARE0_A::DISABLED,
            true => COMPARE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE0_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE0_R {
    type Target = crate::FieldReader<bool, COMPARE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE0_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` writer - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub struct COMPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub struct COMPARE1_R(crate::FieldReader<bool, COMPARE1_A>);
impl COMPARE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_A {
        match self.bits {
            false => COMPARE1_A::DISABLED,
            true => COMPARE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE1_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE1_R {
    type Target = crate::FieldReader<bool, COMPARE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE1_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` writer - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub struct COMPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub struct COMPARE2_R(crate::FieldReader<bool, COMPARE2_A>);
impl COMPARE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_A {
        match self.bits {
            false => COMPARE2_A::DISABLED,
            true => COMPARE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE2_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE2_R {
    type Target = crate::FieldReader<bool, COMPARE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE2_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` writer - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub struct COMPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub struct COMPARE3_R(crate::FieldReader<bool, COMPARE3_A>);
impl COMPARE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_A {
        match self.bits {
            false => COMPARE3_A::DISABLED,
            true => COMPARE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE3_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE3_R {
    type Target = crate::FieldReader<bool, COMPARE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE3_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` writer - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub struct COMPARE3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE4_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` reader - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
pub struct COMPARE4_R(crate::FieldReader<bool, COMPARE4_A>);
impl COMPARE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_A {
        match self.bits {
            false => COMPARE4_A::DISABLED,
            true => COMPARE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE4_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE4_R {
    type Target = crate::FieldReader<bool, COMPARE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE4_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` writer - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
pub struct COMPARE4_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE4_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE5_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` reader - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
pub struct COMPARE5_R(crate::FieldReader<bool, COMPARE5_A>);
impl COMPARE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_A {
        match self.bits {
            false => COMPARE5_A::DISABLED,
            true => COMPARE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE5_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE5_R {
    type Target = crate::FieldReader<bool, COMPARE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE5_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` writer - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
pub struct COMPARE5_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE5_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&self) -> COMPARE4_R {
        COMPARE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&self) -> COMPARE5_R {
        COMPARE5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&mut self) -> COMPARE0_W {
        COMPARE0_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&mut self) -> COMPARE1_W {
        COMPARE1_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&mut self) -> COMPARE2_W {
        COMPARE2_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&mut self) -> COMPARE3_W {
        COMPARE3_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&mut self) -> COMPARE4_W {
        COMPARE4_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&mut self) -> COMPARE5_W {
        COMPARE5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
