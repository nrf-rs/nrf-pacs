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
#[doc = "Write '1' to disable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Write '1' to disable interrupt for event CTS"]
pub struct CTS_R(crate::FieldReader<bool, CTS_A>);
impl CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::DISABLED,
            true => CTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTS_A::ENABLED
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, CTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTS_AW> for bool {
    #[inline(always)]
    fn from(variant: CTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` writer - Write '1' to disable interrupt for event CTS"]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTS_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<NCTS_A> for bool {
    #[inline(always)]
    fn from(variant: NCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` reader - Write '1' to disable interrupt for event NCTS"]
pub struct NCTS_R(crate::FieldReader<bool, NCTS_A>);
impl NCTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NCTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCTS_A {
        match self.bits {
            false => NCTS_A::DISABLED,
            true => NCTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NCTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NCTS_A::ENABLED
    }
}
impl core::ops::Deref for NCTS_R {
    type Target = crate::FieldReader<bool, NCTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<NCTS_AW> for bool {
    #[inline(always)]
    fn from(variant: NCTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` writer - Write '1' to disable interrupt for event NCTS"]
pub struct NCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> NCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCTS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCTS_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` reader - Write '1' to disable interrupt for event RXDRDY"]
pub struct RXDRDY_R(crate::FieldReader<bool, RXDRDY_A>);
impl RXDRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDRDY_A {
        match self.bits {
            false => RXDRDY_A::DISABLED,
            true => RXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXDRDY_A::ENABLED
    }
}
impl core::ops::Deref for RXDRDY_R {
    type Target = crate::FieldReader<bool, RXDRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` writer - Write '1' to disable interrupt for event RXDRDY"]
pub struct RXDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXDRDY_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` reader - Write '1' to disable interrupt for event TXDRDY"]
pub struct TXDRDY_R(crate::FieldReader<bool, TXDRDY_A>);
impl TXDRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDRDY_A {
        match self.bits {
            false => TXDRDY_A::DISABLED,
            true => TXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXDRDY_A::ENABLED
    }
}
impl core::ops::Deref for TXDRDY_R {
    type Target = crate::FieldReader<bool, TXDRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` writer - Write '1' to disable interrupt for event TXDRDY"]
pub struct TXDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXDRDY_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Write '1' to disable interrupt for event ERROR"]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERROR_A::ENABLED
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to disable interrupt for event ERROR"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERROR_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXTO_A> for bool {
    #[inline(always)]
    fn from(variant: RXTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` reader - Write '1' to disable interrupt for event RXTO"]
pub struct RXTO_R(crate::FieldReader<bool, RXTO_A>);
impl RXTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTO_A {
        match self.bits {
            false => RXTO_A::DISABLED,
            true => RXTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXTO_A::ENABLED
    }
}
impl core::ops::Deref for RXTO_R {
    type Target = crate::FieldReader<bool, RXTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXTO_AW> for bool {
    #[inline(always)]
    fn from(variant: RXTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` writer - Write '1' to disable interrupt for event RXTO"]
pub struct RXTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXTO_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&self) -> NCTS_R {
        NCTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RXDRDY_R {
        RXDRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TXDRDY_R {
        TXDRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&mut self) -> NCTS_W {
        NCTS_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&mut self) -> RXDRDY_W {
        RXDRDY_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&mut self) -> TXDRDY_W {
        TXDRDY_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RXTO_W {
        RXTO_W { w: self }
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
