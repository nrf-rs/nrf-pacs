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
#[doc = "Field `RESETPIN` reader - Reset from pin reset detected"]
pub type RESETPIN_R = crate::BitReader<RESETPIN_A>;
#[doc = "Reset from pin reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETPIN_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<RESETPIN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETPIN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETPIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETPIN_A {
        match self.bits {
            false => RESETPIN_A::NOT_DETECTED,
            true => RESETPIN_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RESETPIN_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == RESETPIN_A::DETECTED
    }
}
#[doc = "Field `RESETPIN` writer - Reset from pin reset detected"]
pub type RESETPIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, RESETPIN_A, O>;
impl<'a, const O: u8> RESETPIN_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::DETECTED)
    }
}
#[doc = "Field `DOG0` reader - Reset from application watchdog timer 0 detected"]
pub type DOG0_R = crate::BitReader<DOG0_A>;
#[doc = "Reset from application watchdog timer 0 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG0_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG0_A> for bool {
    #[inline(always)]
    fn from(variant: DOG0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG0_A {
        match self.bits {
            false => DOG0_A::NOT_DETECTED,
            true => DOG0_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG0_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG0_A::DETECTED
    }
}
#[doc = "Field `DOG0` writer - Reset from application watchdog timer 0 detected"]
pub type DOG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, DOG0_A, O>;
impl<'a, const O: u8> DOG0_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG0_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG0_A::DETECTED)
    }
}
#[doc = "Field `CTRLAP` reader - Reset from application CTRL-AP detected"]
pub type CTRLAP_R = crate::BitReader<CTRLAP_A>;
#[doc = "Reset from application CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLAP_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<CTRLAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLAP_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRLAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLAP_A {
        match self.bits {
            false => CTRLAP_A::NOT_DETECTED,
            true => CTRLAP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == CTRLAP_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == CTRLAP_A::DETECTED
    }
}
#[doc = "Field `CTRLAP` writer - Reset from application CTRL-AP detected"]
pub type CTRLAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, CTRLAP_A, O>;
impl<'a, const O: u8> CTRLAP_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::DETECTED)
    }
}
#[doc = "Field `SREQ` reader - Reset from application soft reset detected"]
pub type SREQ_R = crate::BitReader<SREQ_A>;
#[doc = "Reset from application soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREQ_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<SREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREQ_A {
        match self.bits {
            false => SREQ_A::NOT_DETECTED,
            true => SREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == SREQ_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SREQ_A::DETECTED
    }
}
#[doc = "Field `SREQ` writer - Reset from application soft reset detected"]
pub type SREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, SREQ_A, O>;
impl<'a, const O: u8> SREQ_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQ_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQ_A::DETECTED)
    }
}
#[doc = "Field `LOCKUP` reader - Reset from application CPU lockup detected"]
pub type LOCKUP_R = crate::BitReader<LOCKUP_A>;
#[doc = "Reset from application CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::NOT_DETECTED,
            true => LOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LOCKUP_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LOCKUP_A::DETECTED
    }
}
#[doc = "Field `LOCKUP` writer - Reset from application CPU lockup detected"]
pub type LOCKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, LOCKUP_A, O>;
impl<'a, const O: u8> LOCKUP_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::DETECTED)
    }
}
#[doc = "Field `OFF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OFF_R = crate::BitReader<OFF_A>;
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFF_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFF_A {
        match self.bits {
            false => OFF_A::NOT_DETECTED,
            true => OFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OFF_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OFF_A::DETECTED
    }
}
#[doc = "Field `OFF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, OFF_A, O>;
impl<'a, const O: u8> OFF_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFF_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFF_A::DETECTED)
    }
}
#[doc = "Field `LPCOMP` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LPCOMP_R = crate::BitReader<LPCOMP_A>;
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMP_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::NOT_DETECTED,
            true => LPCOMP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LPCOMP_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LPCOMP_A::DETECTED
    }
}
#[doc = "Field `LPCOMP` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LPCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, LPCOMP_A, O>;
impl<'a, const O: u8> LPCOMP_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::DETECTED)
    }
}
#[doc = "Field `DIF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub type DIF_R = crate::BitReader<DIF_A>;
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIF_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::NOT_DETECTED,
            true => DIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DIF_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DIF_A::DETECTED
    }
}
#[doc = "Field `DIF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub type DIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, DIF_A, O>;
impl<'a, const O: u8> DIF_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIF_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIF_A::DETECTED)
    }
}
#[doc = "Field `NFC` reader - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NFC_R = crate::BitReader<NFC_A>;
#[doc = "Reset after wakeup from System OFF mode due to NFC field being detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFC_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<NFC_A> for bool {
    #[inline(always)]
    fn from(variant: NFC_A) -> Self {
        variant as u8 != 0
    }
}
impl NFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFC_A {
        match self.bits {
            false => NFC_A::NOT_DETECTED,
            true => NFC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == NFC_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == NFC_A::DETECTED
    }
}
#[doc = "Field `NFC` writer - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, NFC_A, O>;
impl<'a, const O: u8> NFC_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(NFC_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NFC_A::DETECTED)
    }
}
#[doc = "Field `DOG1` reader - Reset from application watchdog timer 1 detected"]
pub type DOG1_R = crate::BitReader<DOG1_A>;
#[doc = "Reset from application watchdog timer 1 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG1_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG1_A> for bool {
    #[inline(always)]
    fn from(variant: DOG1_A) -> Self {
        variant as u8 != 0
    }
}
impl DOG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG1_A {
        match self.bits {
            false => DOG1_A::NOT_DETECTED,
            true => DOG1_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG1_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG1_A::DETECTED
    }
}
#[doc = "Field `DOG1` writer - Reset from application watchdog timer 1 detected"]
pub type DOG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, DOG1_A, O>;
impl<'a, const O: u8> DOG1_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG1_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG1_A::DETECTED)
    }
}
#[doc = "Field `VBUS` reader - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub type VBUS_R = crate::BitReader<VBUS_A>;
#[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_A {
        match self.bits {
            false => VBUS_A::NOT_DETECTED,
            true => VBUS_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == VBUS_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == VBUS_A::DETECTED
    }
}
#[doc = "Field `VBUS` writer - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub type VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, VBUS_A, O>;
impl<'a, const O: u8> VBUS_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(VBUS_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(VBUS_A::DETECTED)
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> RESETPIN_R {
        RESETPIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&self) -> DOG0_R {
        DOG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&self) -> CTRLAP_R {
        CTRLAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SREQ_R {
        SREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&self) -> NFC_R {
        NFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&self) -> DOG1_R {
        DOG1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> RESETPIN_W<0> {
        RESETPIN_W::new(self)
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&mut self) -> DOG0_W<1> {
        DOG0_W::new(self)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&mut self) -> CTRLAP_W<2> {
        CTRLAP_W::new(self)
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SREQ_W<3> {
        SREQ_W::new(self)
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W<4> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W<5> {
        OFF_W::new(self)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LPCOMP_W<6> {
        LPCOMP_W::new(self)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W<7> {
        DIF_W::new(self)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&mut self) -> NFC_W<24> {
        NFC_W::new(self)
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&mut self) -> DOG1_W<25> {
        DOG1_W::new(self)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W<26> {
        VBUS_W::new(self)
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
