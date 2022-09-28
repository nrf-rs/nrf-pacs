#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRESET` reader - Enable or disable interrupt for event USBRESET"]
pub type USBRESET_R = crate::BitReader<USBRESET_A>;
#[doc = "Enable or disable interrupt for event USBRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBRESET_A> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRESET_A {
        match self.bits {
            false => USBRESET_A::DISABLED,
            true => USBRESET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBRESET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBRESET_A::ENABLED
    }
}
#[doc = "Field `USBRESET` writer - Enable or disable interrupt for event USBRESET"]
pub type USBRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, USBRESET_A, O>;
impl<'a, const O: u8> USBRESET_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBRESET_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBRESET_A::ENABLED)
    }
}
#[doc = "Field `STARTED` reader - Enable or disable interrupt for event STARTED"]
pub type STARTED_R = crate::BitReader<STARTED_A>;
#[doc = "Enable or disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STARTED_A::ENABLED
    }
}
#[doc = "Field `STARTED` writer - Enable or disable interrupt for event STARTED"]
pub type STARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, STARTED_A, O>;
impl<'a, const O: u8> STARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STARTED_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN0` reader - Enable or disable interrupt for event ENDEPIN\\[0\\]"]
pub type ENDEPIN0_R = crate::BitReader<ENDEPIN0_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN0_A {
        match self.bits {
            false => ENDEPIN0_A::DISABLED,
            true => ENDEPIN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN0_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN0` writer - Enable or disable interrupt for event ENDEPIN\\[0\\]"]
pub type ENDEPIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN0_A, O>;
impl<'a, const O: u8> ENDEPIN0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN0_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN1` reader - Enable or disable interrupt for event ENDEPIN\\[1\\]"]
pub type ENDEPIN1_R = crate::BitReader<ENDEPIN1_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN1_A {
        match self.bits {
            false => ENDEPIN1_A::DISABLED,
            true => ENDEPIN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN1_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN1` writer - Enable or disable interrupt for event ENDEPIN\\[1\\]"]
pub type ENDEPIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN1_A, O>;
impl<'a, const O: u8> ENDEPIN1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN1_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN2` reader - Enable or disable interrupt for event ENDEPIN\\[2\\]"]
pub type ENDEPIN2_R = crate::BitReader<ENDEPIN2_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN2_A {
        match self.bits {
            false => ENDEPIN2_A::DISABLED,
            true => ENDEPIN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN2_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN2` writer - Enable or disable interrupt for event ENDEPIN\\[2\\]"]
pub type ENDEPIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN2_A, O>;
impl<'a, const O: u8> ENDEPIN2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN2_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN3` reader - Enable or disable interrupt for event ENDEPIN\\[3\\]"]
pub type ENDEPIN3_R = crate::BitReader<ENDEPIN3_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN3_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN3_A {
        match self.bits {
            false => ENDEPIN3_A::DISABLED,
            true => ENDEPIN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN3_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN3` writer - Enable or disable interrupt for event ENDEPIN\\[3\\]"]
pub type ENDEPIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN3_A, O>;
impl<'a, const O: u8> ENDEPIN3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN3_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN4` reader - Enable or disable interrupt for event ENDEPIN\\[4\\]"]
pub type ENDEPIN4_R = crate::BitReader<ENDEPIN4_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN4_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN4_A {
        match self.bits {
            false => ENDEPIN4_A::DISABLED,
            true => ENDEPIN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN4_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN4` writer - Enable or disable interrupt for event ENDEPIN\\[4\\]"]
pub type ENDEPIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN4_A, O>;
impl<'a, const O: u8> ENDEPIN4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN4_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN5` reader - Enable or disable interrupt for event ENDEPIN\\[5\\]"]
pub type ENDEPIN5_R = crate::BitReader<ENDEPIN5_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN5_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN5_A {
        match self.bits {
            false => ENDEPIN5_A::DISABLED,
            true => ENDEPIN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN5_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN5` writer - Enable or disable interrupt for event ENDEPIN\\[5\\]"]
pub type ENDEPIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN5_A, O>;
impl<'a, const O: u8> ENDEPIN5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN5_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN6` reader - Enable or disable interrupt for event ENDEPIN\\[6\\]"]
pub type ENDEPIN6_R = crate::BitReader<ENDEPIN6_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN6_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN6_A {
        match self.bits {
            false => ENDEPIN6_A::DISABLED,
            true => ENDEPIN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN6_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN6` writer - Enable or disable interrupt for event ENDEPIN\\[6\\]"]
pub type ENDEPIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN6_A, O>;
impl<'a, const O: u8> ENDEPIN6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN6_A::ENABLED)
    }
}
#[doc = "Field `ENDEPIN7` reader - Enable or disable interrupt for event ENDEPIN\\[7\\]"]
pub type ENDEPIN7_R = crate::BitReader<ENDEPIN7_A>;
#[doc = "Enable or disable interrupt for event ENDEPIN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN7_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPIN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN7_A {
        match self.bits {
            false => ENDEPIN7_A::DISABLED,
            true => ENDEPIN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN7_A::ENABLED
    }
}
#[doc = "Field `ENDEPIN7` writer - Enable or disable interrupt for event ENDEPIN\\[7\\]"]
pub type ENDEPIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPIN7_A, O>;
impl<'a, const O: u8> ENDEPIN7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN7_A::ENABLED)
    }
}
#[doc = "Field `EP0DATADONE` reader - Enable or disable interrupt for event EP0DATADONE"]
pub type EP0DATADONE_R = crate::BitReader<EP0DATADONE_A>;
#[doc = "Enable or disable interrupt for event EP0DATADONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EP0DATADONE_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_A) -> Self {
        variant as u8 != 0
    }
}
impl EP0DATADONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_A {
        match self.bits {
            false => EP0DATADONE_A::DISABLED,
            true => EP0DATADONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_A::ENABLED
    }
}
#[doc = "Field `EP0DATADONE` writer - Enable or disable interrupt for event EP0DATADONE"]
pub type EP0DATADONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, EP0DATADONE_A, O>;
impl<'a, const O: u8> EP0DATADONE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_A::ENABLED)
    }
}
#[doc = "Field `ENDISOIN` reader - Enable or disable interrupt for event ENDISOIN"]
pub type ENDISOIN_R = crate::BitReader<ENDISOIN_A>;
#[doc = "Enable or disable interrupt for event ENDISOIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOIN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOIN_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDISOIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOIN_A {
        match self.bits {
            false => ENDISOIN_A::DISABLED,
            true => ENDISOIN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOIN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOIN_A::ENABLED
    }
}
#[doc = "Field `ENDISOIN` writer - Enable or disable interrupt for event ENDISOIN"]
pub type ENDISOIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDISOIN_A, O>;
impl<'a, const O: u8> ENDISOIN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDISOIN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDISOIN_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT0` reader - Enable or disable interrupt for event ENDEPOUT\\[0\\]"]
pub type ENDEPOUT0_R = crate::BitReader<ENDEPOUT0_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_A {
        match self.bits {
            false => ENDEPOUT0_A::DISABLED,
            true => ENDEPOUT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT0` writer - Enable or disable interrupt for event ENDEPOUT\\[0\\]"]
pub type ENDEPOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT0_A, O>;
impl<'a, const O: u8> ENDEPOUT0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT1` reader - Enable or disable interrupt for event ENDEPOUT\\[1\\]"]
pub type ENDEPOUT1_R = crate::BitReader<ENDEPOUT1_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT1_A {
        match self.bits {
            false => ENDEPOUT1_A::DISABLED,
            true => ENDEPOUT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT1_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT1` writer - Enable or disable interrupt for event ENDEPOUT\\[1\\]"]
pub type ENDEPOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT1_A, O>;
impl<'a, const O: u8> ENDEPOUT1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT1_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT2` reader - Enable or disable interrupt for event ENDEPOUT\\[2\\]"]
pub type ENDEPOUT2_R = crate::BitReader<ENDEPOUT2_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT2_A {
        match self.bits {
            false => ENDEPOUT2_A::DISABLED,
            true => ENDEPOUT2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT2_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT2` writer - Enable or disable interrupt for event ENDEPOUT\\[2\\]"]
pub type ENDEPOUT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT2_A, O>;
impl<'a, const O: u8> ENDEPOUT2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT2_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT3` reader - Enable or disable interrupt for event ENDEPOUT\\[3\\]"]
pub type ENDEPOUT3_R = crate::BitReader<ENDEPOUT3_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT3_A {
        match self.bits {
            false => ENDEPOUT3_A::DISABLED,
            true => ENDEPOUT3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT3_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT3` writer - Enable or disable interrupt for event ENDEPOUT\\[3\\]"]
pub type ENDEPOUT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT3_A, O>;
impl<'a, const O: u8> ENDEPOUT3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT3_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT4` reader - Enable or disable interrupt for event ENDEPOUT\\[4\\]"]
pub type ENDEPOUT4_R = crate::BitReader<ENDEPOUT4_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT4_A {
        match self.bits {
            false => ENDEPOUT4_A::DISABLED,
            true => ENDEPOUT4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT4_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT4` writer - Enable or disable interrupt for event ENDEPOUT\\[4\\]"]
pub type ENDEPOUT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT4_A, O>;
impl<'a, const O: u8> ENDEPOUT4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT4_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT5` reader - Enable or disable interrupt for event ENDEPOUT\\[5\\]"]
pub type ENDEPOUT5_R = crate::BitReader<ENDEPOUT5_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT5_A {
        match self.bits {
            false => ENDEPOUT5_A::DISABLED,
            true => ENDEPOUT5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT5_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT5` writer - Enable or disable interrupt for event ENDEPOUT\\[5\\]"]
pub type ENDEPOUT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT5_A, O>;
impl<'a, const O: u8> ENDEPOUT5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT5_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT6` reader - Enable or disable interrupt for event ENDEPOUT\\[6\\]"]
pub type ENDEPOUT6_R = crate::BitReader<ENDEPOUT6_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT6_A {
        match self.bits {
            false => ENDEPOUT6_A::DISABLED,
            true => ENDEPOUT6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT6_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT6` writer - Enable or disable interrupt for event ENDEPOUT\\[6\\]"]
pub type ENDEPOUT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT6_A, O>;
impl<'a, const O: u8> ENDEPOUT6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT6_A::ENABLED)
    }
}
#[doc = "Field `ENDEPOUT7` reader - Enable or disable interrupt for event ENDEPOUT\\[7\\]"]
pub type ENDEPOUT7_R = crate::BitReader<ENDEPOUT7_A>;
#[doc = "Enable or disable interrupt for event ENDEPOUT\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEPOUT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT7_A {
        match self.bits {
            false => ENDEPOUT7_A::DISABLED,
            true => ENDEPOUT7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT7_A::ENABLED
    }
}
#[doc = "Field `ENDEPOUT7` writer - Enable or disable interrupt for event ENDEPOUT\\[7\\]"]
pub type ENDEPOUT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDEPOUT7_A, O>;
impl<'a, const O: u8> ENDEPOUT7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT7_A::ENABLED)
    }
}
#[doc = "Field `ENDISOOUT` reader - Enable or disable interrupt for event ENDISOOUT"]
pub type ENDISOOUT_R = crate::BitReader<ENDISOOUT_A>;
#[doc = "Enable or disable interrupt for event ENDISOOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOOUT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDISOOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDISOOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOOUT_A {
        match self.bits {
            false => ENDISOOUT_A::DISABLED,
            true => ENDISOOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOOUT_A::ENABLED
    }
}
#[doc = "Field `ENDISOOUT` writer - Enable or disable interrupt for event ENDISOOUT"]
pub type ENDISOOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ENDISOOUT_A, O>;
impl<'a, const O: u8> ENDISOOUT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDISOOUT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDISOOUT_A::ENABLED)
    }
}
#[doc = "Field `SOF` reader - Enable or disable interrupt for event SOF"]
pub type SOF_R = crate::BitReader<SOF_A>;
#[doc = "Enable or disable interrupt for event SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::DISABLED,
            true => SOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOF_A::ENABLED
    }
}
#[doc = "Field `SOF` writer - Enable or disable interrupt for event SOF"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SOF_A, O>;
impl<'a, const O: u8> SOF_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOF_A::ENABLED)
    }
}
#[doc = "Field `USBEVENT` reader - Enable or disable interrupt for event USBEVENT"]
pub type USBEVENT_R = crate::BitReader<USBEVENT_A>;
#[doc = "Enable or disable interrupt for event USBEVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEVENT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBEVENT_A> for bool {
    #[inline(always)]
    fn from(variant: USBEVENT_A) -> Self {
        variant as u8 != 0
    }
}
impl USBEVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEVENT_A {
        match self.bits {
            false => USBEVENT_A::DISABLED,
            true => USBEVENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBEVENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBEVENT_A::ENABLED
    }
}
#[doc = "Field `USBEVENT` writer - Enable or disable interrupt for event USBEVENT"]
pub type USBEVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, USBEVENT_A, O>;
impl<'a, const O: u8> USBEVENT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBEVENT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBEVENT_A::ENABLED)
    }
}
#[doc = "Field `EP0SETUP` reader - Enable or disable interrupt for event EP0SETUP"]
pub type EP0SETUP_R = crate::BitReader<EP0SETUP_A>;
#[doc = "Enable or disable interrupt for event EP0SETUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0SETUP_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EP0SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: EP0SETUP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP0SETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0SETUP_A {
        match self.bits {
            false => EP0SETUP_A::DISABLED,
            true => EP0SETUP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0SETUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0SETUP_A::ENABLED
    }
}
#[doc = "Field `EP0SETUP` writer - Enable or disable interrupt for event EP0SETUP"]
pub type EP0SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, EP0SETUP_A, O>;
impl<'a, const O: u8> EP0SETUP_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0SETUP_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0SETUP_A::ENABLED)
    }
}
#[doc = "Field `EPDATA` reader - Enable or disable interrupt for event EPDATA"]
pub type EPDATA_R = crate::BitReader<EPDATA_A>;
#[doc = "Enable or disable interrupt for event EPDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDATA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EPDATA_A> for bool {
    #[inline(always)]
    fn from(variant: EPDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl EPDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDATA_A {
        match self.bits {
            false => EPDATA_A::DISABLED,
            true => EPDATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPDATA_A::ENABLED
    }
}
#[doc = "Field `EPDATA` writer - Enable or disable interrupt for event EPDATA"]
pub type EPDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, EPDATA_A, O>;
impl<'a, const O: u8> EPDATA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPDATA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EPDATA_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&self) -> USBRESET_R {
        USBRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&self) -> ENDEPIN0_R {
        ENDEPIN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&self) -> ENDEPIN1_R {
        ENDEPIN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&self) -> ENDEPIN2_R {
        ENDEPIN2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&self) -> ENDEPIN3_R {
        ENDEPIN3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&self) -> ENDEPIN4_R {
        ENDEPIN4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&self) -> ENDEPIN5_R {
        ENDEPIN5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&self) -> ENDEPIN6_R {
        ENDEPIN6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&self) -> ENDEPIN7_R {
        ENDEPIN7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&self) -> EP0DATADONE_R {
        EP0DATADONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&self) -> ENDISOIN_R {
        ENDISOIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&self) -> ENDEPOUT0_R {
        ENDEPOUT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&self) -> ENDEPOUT1_R {
        ENDEPOUT1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&self) -> ENDEPOUT2_R {
        ENDEPOUT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&self) -> ENDEPOUT3_R {
        ENDEPOUT3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&self) -> ENDEPOUT4_R {
        ENDEPOUT4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&self) -> ENDEPOUT5_R {
        ENDEPOUT5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&self) -> ENDEPOUT6_R {
        ENDEPOUT6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&self) -> ENDEPOUT7_R {
        ENDEPOUT7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&self) -> ENDISOOUT_R {
        ENDISOOUT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&self) -> USBEVENT_R {
        USBEVENT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable or disable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&self) -> EP0SETUP_R {
        EP0SETUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&self) -> EPDATA_R {
        EPDATA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> USBRESET_W<0> {
        USBRESET_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W<1> {
        STARTED_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&mut self) -> ENDEPIN0_W<2> {
        ENDEPIN0_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&mut self) -> ENDEPIN1_W<3> {
        ENDEPIN1_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&mut self) -> ENDEPIN2_W<4> {
        ENDEPIN2_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&mut self) -> ENDEPIN3_W<5> {
        ENDEPIN3_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&mut self) -> ENDEPIN4_W<6> {
        ENDEPIN4_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&mut self) -> ENDEPIN5_W<7> {
        ENDEPIN5_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&mut self) -> ENDEPIN6_W<8> {
        ENDEPIN6_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&mut self) -> ENDEPIN7_W<9> {
        ENDEPIN7_W::new(self)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&mut self) -> EP0DATADONE_W<10> {
        EP0DATADONE_W::new(self)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&mut self) -> ENDISOIN_W<11> {
        ENDISOIN_W::new(self)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&mut self) -> ENDEPOUT0_W<12> {
        ENDEPOUT0_W::new(self)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&mut self) -> ENDEPOUT1_W<13> {
        ENDEPOUT1_W::new(self)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&mut self) -> ENDEPOUT2_W<14> {
        ENDEPOUT2_W::new(self)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&mut self) -> ENDEPOUT3_W<15> {
        ENDEPOUT3_W::new(self)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&mut self) -> ENDEPOUT4_W<16> {
        ENDEPOUT4_W::new(self)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&mut self) -> ENDEPOUT5_W<17> {
        ENDEPOUT5_W::new(self)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&mut self) -> ENDEPOUT6_W<18> {
        ENDEPOUT6_W::new(self)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&mut self) -> ENDEPOUT7_W<19> {
        ENDEPOUT7_W::new(self)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&mut self) -> ENDISOOUT_W<20> {
        ENDISOOUT_W::new(self)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<21> {
        SOF_W::new(self)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&mut self) -> USBEVENT_W<22> {
        USBEVENT_W::new(self)
    }
    #[doc = "Bit 23 - Enable or disable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&mut self) -> EP0SETUP_W<23> {
        EP0SETUP_W::new(self)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&mut self) -> EPDATA_W<24> {
        EPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
