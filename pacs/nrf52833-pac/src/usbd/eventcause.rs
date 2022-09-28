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
#[doc = "Field `ISOOUTCRC` reader - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub type ISOOUTCRC_R = crate::BitReader<ISOOUTCRC_A>;
#[doc = "CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOOUTCRC_A {
    #[doc = "0: No error detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Error detected"]
    DETECTED = 1,
}
impl From<ISOOUTCRC_A> for bool {
    #[inline(always)]
    fn from(variant: ISOOUTCRC_A) -> Self {
        variant as u8 != 0
    }
}
impl ISOOUTCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOOUTCRC_A {
        match self.bits {
            false => ISOOUTCRC_A::NOT_DETECTED,
            true => ISOOUTCRC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == ISOOUTCRC_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == ISOOUTCRC_A::DETECTED
    }
}
#[doc = "Field `ISOOUTCRC` writer - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub type ISOOUTCRC_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EVENTCAUSE_SPEC, ISOOUTCRC_A, O>;
impl<'a, const O: u8> ISOOUTCRC_W<'a, O> {
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(ISOOUTCRC_A::NOT_DETECTED)
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(ISOOUTCRC_A::DETECTED)
    }
}
#[doc = "Field `SUSPEND` reader - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub type SUSPEND_R = crate::BitReader<SUSPEND_A>;
#[doc = "Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_A {
    #[doc = "0: Suspend not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Suspend detected"]
    DETECTED = 1,
}
impl From<SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPEND_A {
        match self.bits {
            false => SUSPEND_A::NOT_DETECTED,
            true => SUSPEND_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == SUSPEND_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SUSPEND_A::DETECTED
    }
}
#[doc = "Field `SUSPEND` writer - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EVENTCAUSE_SPEC, SUSPEND_A, O>;
impl<'a, const O: u8> SUSPEND_W<'a, O> {
    #[doc = "Suspend not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SUSPEND_A::NOT_DETECTED)
    }
    #[doc = "Suspend detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SUSPEND_A::DETECTED)
    }
}
#[doc = "Field `RESUME` reader - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub type RESUME_R = crate::BitReader<RESUME_A>;
#[doc = "Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUME_A {
    #[doc = "0: Resume not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Resume detected"]
    DETECTED = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUME_A {
        match self.bits {
            false => RESUME_A::NOT_DETECTED,
            true => RESUME_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RESUME_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == RESUME_A::DETECTED
    }
}
#[doc = "Field `RESUME` writer - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EVENTCAUSE_SPEC, RESUME_A, O>;
impl<'a, const O: u8> RESUME_W<'a, O> {
    #[doc = "Resume not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESUME_A::NOT_DETECTED)
    }
    #[doc = "Resume detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESUME_A::DETECTED)
    }
}
#[doc = "Field `USBWUALLOWED` reader - USB MAC has been woken up and operational. Write '1' to clear."]
pub type USBWUALLOWED_R = crate::BitReader<USBWUALLOWED_A>;
#[doc = "USB MAC has been woken up and operational. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBWUALLOWED_A {
    #[doc = "0: Wake up not allowed"]
    NOT_ALLOWED = 0,
    #[doc = "1: Wake up allowed"]
    ALLOWED = 1,
}
impl From<USBWUALLOWED_A> for bool {
    #[inline(always)]
    fn from(variant: USBWUALLOWED_A) -> Self {
        variant as u8 != 0
    }
}
impl USBWUALLOWED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBWUALLOWED_A {
        match self.bits {
            false => USBWUALLOWED_A::NOT_ALLOWED,
            true => USBWUALLOWED_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == USBWUALLOWED_A::NOT_ALLOWED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == USBWUALLOWED_A::ALLOWED
    }
}
#[doc = "Field `USBWUALLOWED` writer - USB MAC has been woken up and operational. Write '1' to clear."]
pub type USBWUALLOWED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, EVENTCAUSE_SPEC, USBWUALLOWED_A, O>;
impl<'a, const O: u8> USBWUALLOWED_W<'a, O> {
    #[doc = "Wake up not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWED_A::NOT_ALLOWED)
    }
    #[doc = "Wake up allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWED_A::ALLOWED)
    }
}
#[doc = "Field `READY` reader - USB device is ready for normal operation. Write '1' to clear."]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "USB device is ready for normal operation. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: USBEVENT was not issued due to USBD peripheral ready"]
    NOT_DETECTED = 0,
    #[doc = "1: USBD peripheral is ready"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOT_DETECTED,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == READY_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
}
#[doc = "Field `READY` writer - USB device is ready for normal operation. Write '1' to clear."]
pub type READY_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EVENTCAUSE_SPEC, READY_A, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(READY_A::NOT_DETECTED)
    }
    #[doc = "USBD peripheral is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(READY_A::READY)
    }
}
impl R {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&self) -> ISOOUTCRC_R {
        ISOOUTCRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&self) -> USBWUALLOWED_R {
        USBWUALLOWED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&mut self) -> ISOOUTCRC_W<0> {
        ISOOUTCRC_W::new(self)
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W<8> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<9> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&mut self) -> USBWUALLOWED_W<10> {
        USBWUALLOWED_W::new(self)
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<11> {
        READY_W::new(self)
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
