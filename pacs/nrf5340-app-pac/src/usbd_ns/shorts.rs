#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPIN0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_STARTEPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_STARTEPIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPIN0` reader - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
pub struct EP0DATADONE_STARTEPIN0_R(crate::FieldReader<bool, EP0DATADONE_STARTEPIN0_A>);
impl EP0DATADONE_STARTEPIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0DATADONE_STARTEPIN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_STARTEPIN0_A {
        match self.bits {
            false => EP0DATADONE_STARTEPIN0_A::DISABLED,
            true => EP0DATADONE_STARTEPIN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EP0DATADONE_STARTEPIN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EP0DATADONE_STARTEPIN0_A::ENABLED
    }
}
impl core::ops::Deref for EP0DATADONE_STARTEPIN0_R {
    type Target = crate::FieldReader<bool, EP0DATADONE_STARTEPIN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPIN0` writer - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
pub struct EP0DATADONE_STARTEPIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_STARTEPIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_STARTEPIN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0_A::ENABLED)
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
#[doc = "Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPOUT0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_STARTEPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_STARTEPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPOUT0` reader - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
pub struct EP0DATADONE_STARTEPOUT0_R(crate::FieldReader<bool, EP0DATADONE_STARTEPOUT0_A>);
impl EP0DATADONE_STARTEPOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0DATADONE_STARTEPOUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_STARTEPOUT0_A {
        match self.bits {
            false => EP0DATADONE_STARTEPOUT0_A::DISABLED,
            true => EP0DATADONE_STARTEPOUT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EP0DATADONE_STARTEPOUT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EP0DATADONE_STARTEPOUT0_A::ENABLED
    }
}
impl core::ops::Deref for EP0DATADONE_STARTEPOUT0_R {
    type Target = crate::FieldReader<bool, EP0DATADONE_STARTEPOUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0DATADONE_STARTEPOUT0` writer - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
pub struct EP0DATADONE_STARTEPOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_STARTEPOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_STARTEPOUT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0_A::ENABLED)
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
#[doc = "Shortcut between event EP0DATADONE and task EP0STATUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_EP0STATUS_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_EP0STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_EP0STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE_EP0STATUS` reader - Shortcut between event EP0DATADONE and task EP0STATUS"]
pub struct EP0DATADONE_EP0STATUS_R(crate::FieldReader<bool, EP0DATADONE_EP0STATUS_A>);
impl EP0DATADONE_EP0STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0DATADONE_EP0STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_EP0STATUS_A {
        match self.bits {
            false => EP0DATADONE_EP0STATUS_A::DISABLED,
            true => EP0DATADONE_EP0STATUS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EP0DATADONE_EP0STATUS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EP0DATADONE_EP0STATUS_A::ENABLED
    }
}
impl core::ops::Deref for EP0DATADONE_EP0STATUS_R {
    type Target = crate::FieldReader<bool, EP0DATADONE_EP0STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0DATADONE_EP0STATUS` writer - Shortcut between event EP0DATADONE and task EP0STATUS"]
pub struct EP0DATADONE_EP0STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_EP0STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_EP0STATUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUS_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUS_A::ENABLED)
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
#[doc = "Shortcut between event ENDEPOUT\\[0\\]
and task EP0STATUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0STATUS_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_EP0STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_EP0STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0_EP0STATUS` reader - Shortcut between event ENDEPOUT\\[0\\]
and task EP0STATUS"]
pub struct ENDEPOUT0_EP0STATUS_R(crate::FieldReader<bool, ENDEPOUT0_EP0STATUS_A>);
impl ENDEPOUT0_EP0STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT0_EP0STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_EP0STATUS_A {
        match self.bits {
            false => ENDEPOUT0_EP0STATUS_A::DISABLED,
            true => ENDEPOUT0_EP0STATUS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT0_EP0STATUS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT0_EP0STATUS_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT0_EP0STATUS_R {
    type Target = crate::FieldReader<bool, ENDEPOUT0_EP0STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDEPOUT0_EP0STATUS` writer - Shortcut between event ENDEPOUT\\[0\\]
and task EP0STATUS"]
pub struct ENDEPOUT0_EP0STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_EP0STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_EP0STATUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUS_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Shortcut between event ENDEPOUT\\[0\\]
and task EP0RCVOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0RCVOUT_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_EP0RCVOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_EP0RCVOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0_EP0RCVOUT` reader - Shortcut between event ENDEPOUT\\[0\\]
and task EP0RCVOUT"]
pub struct ENDEPOUT0_EP0RCVOUT_R(crate::FieldReader<bool, ENDEPOUT0_EP0RCVOUT_A>);
impl ENDEPOUT0_EP0RCVOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT0_EP0RCVOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_EP0RCVOUT_A {
        match self.bits {
            false => ENDEPOUT0_EP0RCVOUT_A::DISABLED,
            true => ENDEPOUT0_EP0RCVOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT0_EP0RCVOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT0_EP0RCVOUT_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT0_EP0RCVOUT_R {
    type Target = crate::FieldReader<bool, ENDEPOUT0_EP0RCVOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDEPOUT0_EP0RCVOUT` writer - Shortcut between event ENDEPOUT\\[0\\]
and task EP0RCVOUT"]
pub struct ENDEPOUT0_EP0RCVOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_EP0RCVOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_EP0RCVOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUT_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&self) -> EP0DATADONE_STARTEPIN0_R {
        EP0DATADONE_STARTEPIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&self) -> EP0DATADONE_STARTEPOUT0_R {
        EP0DATADONE_STARTEPOUT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&self) -> EP0DATADONE_EP0STATUS_R {
        EP0DATADONE_EP0STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event ENDEPOUT\\[0\\]
and task EP0STATUS"]
    #[inline(always)]
    pub fn endepout0_ep0status(&self) -> ENDEPOUT0_EP0STATUS_R {
        ENDEPOUT0_EP0STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event ENDEPOUT\\[0\\]
and task EP0RCVOUT"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&self) -> ENDEPOUT0_EP0RCVOUT_R {
        ENDEPOUT0_EP0RCVOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&mut self) -> EP0DATADONE_STARTEPIN0_W {
        EP0DATADONE_STARTEPIN0_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&mut self) -> EP0DATADONE_STARTEPOUT0_W {
        EP0DATADONE_STARTEPOUT0_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&mut self) -> EP0DATADONE_EP0STATUS_W {
        EP0DATADONE_EP0STATUS_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between event ENDEPOUT\\[0\\]
and task EP0STATUS"]
    #[inline(always)]
    pub fn endepout0_ep0status(&mut self) -> ENDEPOUT0_EP0STATUS_W {
        ENDEPOUT0_EP0STATUS_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event ENDEPOUT\\[0\\]
and task EP0RCVOUT"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&mut self) -> ENDEPOUT0_EP0RCVOUT_W {
        ENDEPOUT0_EP0RCVOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
