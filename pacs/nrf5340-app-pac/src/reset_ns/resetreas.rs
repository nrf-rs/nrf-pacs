#[doc = "Register `RESETREAS` reader"]
pub struct R(crate::R<RESETREAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETREAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETREAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETREAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETREAS` writer"]
pub struct W(crate::W<RESETREAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETREAS_SPEC>;
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
impl From<crate::W<RESETREAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETREAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset from pin reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETPIN_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<RESETPIN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETPIN` reader - Reset from pin reset detected"]
pub struct RESETPIN_R(crate::FieldReader<bool, RESETPIN_A>);
impl RESETPIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETPIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETPIN_A {
        match self.bits {
            false => RESETPIN_A::NOTDETECTED,
            true => RESETPIN_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == RESETPIN_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == RESETPIN_A::DETECTED
    }
}
impl core::ops::Deref for RESETPIN_R {
    type Target = crate::FieldReader<bool, RESETPIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETPIN` writer - Reset from pin reset detected"]
pub struct RESETPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETPIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::DETECTED)
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
#[doc = "Reset from application watchdog timer 0 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG0_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG0_A> for bool {
    #[inline(always)]
    fn from(variant: DOG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG0` reader - Reset from application watchdog timer 0 detected"]
pub struct DOG0_R(crate::FieldReader<bool, DOG0_A>);
impl DOG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG0_A {
        match self.bits {
            false => DOG0_A::NOTDETECTED,
            true => DOG0_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == DOG0_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == DOG0_A::DETECTED
    }
}
impl core::ops::Deref for DOG0_R {
    type Target = crate::FieldReader<bool, DOG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOG0` writer - Reset from application watchdog timer 0 detected"]
pub struct DOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG0_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG0_A::DETECTED)
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
#[doc = "Reset from application CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLAP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<CTRLAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLAP` reader - Reset from application CTRL-AP detected"]
pub struct CTRLAP_R(crate::FieldReader<bool, CTRLAP_A>);
impl CTRLAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLAP_A {
        match self.bits {
            false => CTRLAP_A::NOTDETECTED,
            true => CTRLAP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == CTRLAP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == CTRLAP_A::DETECTED
    }
}
impl core::ops::Deref for CTRLAP_R {
    type Target = crate::FieldReader<bool, CTRLAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLAP` writer - Reset from application CTRL-AP detected"]
pub struct CTRLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRLAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::DETECTED)
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
#[doc = "Reset from application soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREQ_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<SREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREQ` reader - Reset from application soft reset detected"]
pub struct SREQ_R(crate::FieldReader<bool, SREQ_A>);
impl SREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREQ_A {
        match self.bits {
            false => SREQ_A::NOTDETECTED,
            true => SREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == SREQ_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == SREQ_A::DETECTED
    }
}
impl core::ops::Deref for SREQ_R {
    type Target = crate::FieldReader<bool, SREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SREQ` writer - Reset from application soft reset detected"]
pub struct SREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQ_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQ_A::DETECTED)
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
#[doc = "Reset from application CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Reset from application CPU lockup detected"]
pub struct LOCKUP_R(crate::FieldReader<bool, LOCKUP_A>);
impl LOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::NOTDETECTED,
            true => LOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == LOCKUP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == LOCKUP_A::DETECTED
    }
}
impl core::ops::Deref for LOCKUP_R {
    type Target = crate::FieldReader<bool, LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUP` writer - Reset from application CPU lockup detected"]
pub struct LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::DETECTED)
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
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub struct OFF_R(crate::FieldReader<bool, OFF_A>);
impl OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFF_A {
        match self.bits {
            false => OFF_A::NOTDETECTED,
            true => OFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == OFF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == OFF_A::DETECTED
    }
}
impl core::ops::Deref for OFF_R {
    type Target = crate::FieldReader<bool, OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFF_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCOMP` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub struct LPCOMP_R(crate::FieldReader<bool, LPCOMP_A>);
impl LPCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPCOMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::NOTDETECTED,
            true => LPCOMP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == LPCOMP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == LPCOMP_A::DETECTED
    }
}
impl core::ops::Deref for LPCOMP_R {
    type Target = crate::FieldReader<bool, LPCOMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCOMP` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub struct LPCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCOMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCOMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub struct DIF_R(crate::FieldReader<bool, DIF_A>);
impl DIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::NOTDETECTED,
            true => DIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == DIF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == DIF_A::DETECTED
    }
}
impl core::ops::Deref for DIF_R {
    type Target = crate::FieldReader<bool, DIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub struct DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIF_A::DETECTED)
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
#[doc = "Reset after wakeup from System OFF mode due to NFC field being detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFC_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<NFC_A> for bool {
    #[inline(always)]
    fn from(variant: NFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFC` reader - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub struct NFC_R(crate::FieldReader<bool, NFC_A>);
impl NFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFC_A {
        match self.bits {
            false => NFC_A::NOTDETECTED,
            true => NFC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == NFC_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == NFC_A::DETECTED
    }
}
impl core::ops::Deref for NFC_R {
    type Target = crate::FieldReader<bool, NFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFC` writer - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub struct NFC_W<'a> {
    w: &'a mut W,
}
impl<'a> NFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(NFC_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NFC_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Reset from application watchdog timer 1 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG1_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG1_A> for bool {
    #[inline(always)]
    fn from(variant: DOG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG1` reader - Reset from application watchdog timer 1 detected"]
pub struct DOG1_R(crate::FieldReader<bool, DOG1_A>);
impl DOG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG1_A {
        match self.bits {
            false => DOG1_A::NOTDETECTED,
            true => DOG1_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == DOG1_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == DOG1_A::DETECTED
    }
}
impl core::ops::Deref for DOG1_R {
    type Target = crate::FieldReader<bool, DOG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOG1` writer - Reset from application watchdog timer 1 detected"]
pub struct DOG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG1_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG1_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS` reader - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub struct VBUS_R(crate::FieldReader<bool, VBUS_A>);
impl VBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_A {
        match self.bits {
            false => VBUS_A::NOTDETECTED,
            true => VBUS_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == VBUS_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == VBUS_A::DETECTED
    }
}
impl core::ops::Deref for VBUS_R {
    type Target = crate::FieldReader<bool, VBUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS` writer - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub struct VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(VBUS_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(VBUS_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> RESETPIN_R {
        RESETPIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&self) -> DOG0_R {
        DOG0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&self) -> CTRLAP_R {
        CTRLAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SREQ_R {
        SREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&self) -> NFC_R {
        NFC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&self) -> DOG1_R {
        DOG1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> RESETPIN_W {
        RESETPIN_W { w: self }
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&mut self) -> DOG0_W {
        DOG0_W { w: self }
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&mut self) -> CTRLAP_W {
        CTRLAP_W { w: self }
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SREQ_W {
        SREQ_W { w: self }
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W {
        LOCKUP_W { w: self }
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LPCOMP_W {
        LPCOMP_W { w: self }
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W {
        DIF_W { w: self }
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&mut self) -> NFC_W {
        NFC_W { w: self }
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&mut self) -> DOG1_W {
        DOG1_W { w: self }
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetreas](index.html) module"]
pub struct RESETREAS_SPEC;
impl crate::RegisterSpec for RESETREAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetreas::R](R) reader structure"]
impl crate::Readable for RESETREAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetreas::W](W) writer structure"]
impl crate::Writable for RESETREAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESETREAS to value 0"]
impl crate::Resettable for RESETREAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
