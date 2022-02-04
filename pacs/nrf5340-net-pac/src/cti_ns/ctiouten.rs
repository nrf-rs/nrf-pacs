#[doc = "Register `CTIOUTEN[%s]` reader"]
pub struct R(crate::R<CTIOUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIOUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIOUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIOUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIOUTEN[%s]` writer"]
pub struct W(crate::W<CTIOUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIOUTEN_SPEC>;
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
impl From<crate::W<CTIOUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIOUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_0_A {
    #[doc = "0: Channel 0 is ignored by output trigger n."]
    DISABLED = 0,
    #[doc = "1: When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 1,
}
impl From<TRIGOUTEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_0` reader - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
pub struct TRIGOUTEN_0_R(crate::FieldReader<bool, TRIGOUTEN_0_A>);
impl TRIGOUTEN_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGOUTEN_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_0_A {
        match self.bits {
            false => TRIGOUTEN_0_A::DISABLED,
            true => TRIGOUTEN_0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGOUTEN_0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGOUTEN_0_A::ENABLED
    }
}
impl core::ops::Deref for TRIGOUTEN_0_R {
    type Target = crate::FieldReader<bool, TRIGOUTEN_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGOUTEN_0` writer - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
pub struct TRIGOUTEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 0 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_0_A::DISABLED)
    }
    #[doc = "When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_0_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_1_A {
    #[doc = "0: Channel 1 is ignored by output trigger n."]
    DISABLED = 0,
    #[doc = "1: When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 1,
}
impl From<TRIGOUTEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_1` reader - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
pub struct TRIGOUTEN_1_R(crate::FieldReader<bool, TRIGOUTEN_1_A>);
impl TRIGOUTEN_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGOUTEN_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_1_A {
        match self.bits {
            false => TRIGOUTEN_1_A::DISABLED,
            true => TRIGOUTEN_1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGOUTEN_1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGOUTEN_1_A::ENABLED
    }
}
impl core::ops::Deref for TRIGOUTEN_1_R {
    type Target = crate::FieldReader<bool, TRIGOUTEN_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGOUTEN_1` writer - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
pub struct TRIGOUTEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 1 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_1_A::DISABLED)
    }
    #[doc = "When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_1_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_2_A {
    #[doc = "0: Channel 2 is ignored by output trigger n."]
    DISABLED = 0,
    #[doc = "1: When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 1,
}
impl From<TRIGOUTEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_2` reader - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
pub struct TRIGOUTEN_2_R(crate::FieldReader<bool, TRIGOUTEN_2_A>);
impl TRIGOUTEN_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGOUTEN_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_2_A {
        match self.bits {
            false => TRIGOUTEN_2_A::DISABLED,
            true => TRIGOUTEN_2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGOUTEN_2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGOUTEN_2_A::ENABLED
    }
}
impl core::ops::Deref for TRIGOUTEN_2_R {
    type Target = crate::FieldReader<bool, TRIGOUTEN_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGOUTEN_2` writer - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
pub struct TRIGOUTEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 2 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_2_A::DISABLED)
    }
    #[doc = "When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_2_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_3_A {
    #[doc = "0: Channel 3 is ignored by output trigger n."]
    DISABLED = 0,
    #[doc = "1: When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 1,
}
impl From<TRIGOUTEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_3` reader - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
pub struct TRIGOUTEN_3_R(crate::FieldReader<bool, TRIGOUTEN_3_A>);
impl TRIGOUTEN_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGOUTEN_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_3_A {
        match self.bits {
            false => TRIGOUTEN_3_A::DISABLED,
            true => TRIGOUTEN_3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGOUTEN_3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGOUTEN_3_A::ENABLED
    }
}
impl core::ops::Deref for TRIGOUTEN_3_R {
    type Target = crate::FieldReader<bool, TRIGOUTEN_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGOUTEN_3` writer - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
pub struct TRIGOUTEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 3 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_3_A::DISABLED)
    }
    #[doc = "When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_3_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub fn trigouten_0(&self) -> TRIGOUTEN_0_R {
        TRIGOUTEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub fn trigouten_1(&self) -> TRIGOUTEN_1_R {
        TRIGOUTEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub fn trigouten_2(&self) -> TRIGOUTEN_2_R {
        TRIGOUTEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub fn trigouten_3(&self) -> TRIGOUTEN_3_R {
        TRIGOUTEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub fn trigouten_0(&mut self) -> TRIGOUTEN_0_W {
        TRIGOUTEN_0_W { w: self }
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub fn trigouten_1(&mut self) -> TRIGOUTEN_1_W {
        TRIGOUTEN_1_W { w: self }
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub fn trigouten_2(&mut self) -> TRIGOUTEN_2_W {
        TRIGOUTEN_2_W { w: self }
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub fn trigouten_3(&mut self) -> TRIGOUTEN_3_W {
        TRIGOUTEN_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: CTI Trigger output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiouten](index.html) module"]
pub struct CTIOUTEN_SPEC;
impl crate::RegisterSpec for CTIOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctiouten::R](R) reader structure"]
impl crate::Readable for CTIOUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctiouten::W](W) writer structure"]
impl crate::Writable for CTIOUTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIOUTEN[%s]
to value 0"]
impl crate::Resettable for CTIOUTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
