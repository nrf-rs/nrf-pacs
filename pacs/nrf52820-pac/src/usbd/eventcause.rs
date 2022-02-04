#[doc = "Register `EVENTCAUSE` reader"]
pub struct R(crate::R<EVENTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTCAUSE` writer"]
pub struct W(crate::W<EVENTCAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTCAUSE_SPEC>;
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
impl From<crate::W<EVENTCAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTCAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOOUTCRC_A {
    #[doc = "0: No error detected"]
    NOTDETECTED = 0,
    #[doc = "1: Error detected"]
    DETECTED = 1,
}
impl From<ISOOUTCRC_A> for bool {
    #[inline(always)]
    fn from(variant: ISOOUTCRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOOUTCRC` reader - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub struct ISOOUTCRC_R(crate::FieldReader<bool, ISOOUTCRC_A>);
impl ISOOUTCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOOUTCRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOOUTCRC_A {
        match self.bits {
            false => ISOOUTCRC_A::NOTDETECTED,
            true => ISOOUTCRC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == ISOOUTCRC_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == ISOOUTCRC_A::DETECTED
    }
}
impl core::ops::Deref for ISOOUTCRC_R {
    type Target = crate::FieldReader<bool, ISOOUTCRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOOUTCRC` writer - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub struct ISOOUTCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOUTCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOOUTCRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(ISOOUTCRC_A::NOTDETECTED)
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(ISOOUTCRC_A::DETECTED)
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
#[doc = "Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_A {
    #[doc = "0: Suspend not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Suspend detected"]
    DETECTED = 1,
}
impl From<SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND` reader - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub struct SUSPEND_R(crate::FieldReader<bool, SUSPEND_A>);
impl SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPEND_A {
        match self.bits {
            false => SUSPEND_A::NOTDETECTED,
            true => SUSPEND_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == SUSPEND_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == SUSPEND_A::DETECTED
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, SUSPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` writer - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Suspend not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SUSPEND_A::NOTDETECTED)
    }
    #[doc = "Suspend detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SUSPEND_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUME_A {
    #[doc = "0: Resume not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Resume detected"]
    DETECTED = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub struct RESUME_R(crate::FieldReader<bool, RESUME_A>);
impl RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUME_A {
        match self.bits {
            false => RESUME_A::NOTDETECTED,
            true => RESUME_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == RESUME_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == RESUME_A::DETECTED
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, RESUME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESUME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Resume not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESUME_A::NOTDETECTED)
    }
    #[doc = "Resume detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESUME_A::DETECTED)
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
#[doc = "USB MAC has been woken up and operational. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBWUALLOWED_A {
    #[doc = "0: Wake up not allowed"]
    NOTALLOWED = 0,
    #[doc = "1: Wake up allowed"]
    ALLOWED = 1,
}
impl From<USBWUALLOWED_A> for bool {
    #[inline(always)]
    fn from(variant: USBWUALLOWED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBWUALLOWED` reader - USB MAC has been woken up and operational. Write '1' to clear."]
pub struct USBWUALLOWED_R(crate::FieldReader<bool, USBWUALLOWED_A>);
impl USBWUALLOWED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBWUALLOWED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBWUALLOWED_A {
        match self.bits {
            false => USBWUALLOWED_A::NOTALLOWED,
            true => USBWUALLOWED_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == USBWUALLOWED_A::NOTALLOWED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == USBWUALLOWED_A::ALLOWED
    }
}
impl core::ops::Deref for USBWUALLOWED_R {
    type Target = crate::FieldReader<bool, USBWUALLOWED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBWUALLOWED` writer - USB MAC has been woken up and operational. Write '1' to clear."]
pub struct USBWUALLOWED_W<'a> {
    w: &'a mut W,
}
impl<'a> USBWUALLOWED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBWUALLOWED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake up not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWED_A::NOTALLOWED)
    }
    #[doc = "Wake up allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWED_A::ALLOWED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "USB device is ready for normal operation. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: USBEVENT was not issued due to USBD peripheral ready"]
    NOTDETECTED = 0,
    #[doc = "1: USBD peripheral is ready"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - USB device is ready for normal operation. Write '1' to clear."]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOTDETECTED,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == READY_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == READY_A::READY
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` writer - USB device is ready for normal operation. Write '1' to clear."]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(READY_A::NOTDETECTED)
    }
    #[doc = "USBD peripheral is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(READY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&self) -> ISOOUTCRC_R {
        ISOOUTCRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&self) -> USBWUALLOWED_R {
        USBWUALLOWED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&mut self) -> ISOOUTCRC_W {
        ISOOUTCRC_W { w: self }
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&mut self) -> USBWUALLOWED_W {
        USBWUALLOWED_W { w: self }
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Details on what caused the USBEVENT event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventcause](index.html) module"]
pub struct EVENTCAUSE_SPEC;
impl crate::RegisterSpec for EVENTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eventcause::R](R) reader structure"]
impl crate::Readable for EVENTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eventcause::W](W) writer structure"]
impl crate::Writable for EVENTCAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTCAUSE to value 0"]
impl crate::Resettable for EVENTCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
