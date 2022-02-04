#[doc = "Register `DFECTRL1` reader"]
pub struct R(crate::R<DFECTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFECTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFECTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFECTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFECTRL1` writer"]
pub struct W(crate::W<DFECTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFECTRL1_SPEC>;
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
impl From<crate::W<DFECTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFECTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMBEROF8US` reader - Length of the AoA/AoD procedure in number of 8 us units"]
pub struct NUMBEROF8US_R(crate::FieldReader<u8, u8>);
impl NUMBEROF8US_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMBEROF8US_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMBEROF8US_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMBEROF8US` writer - Length of the AoA/AoD procedure in number of 8 us units"]
pub struct NUMBEROF8US_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMBEROF8US_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Add CTE extension and do antenna switching/sampling in this extension\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEINEXTENSION_A {
    #[doc = "1: AoA/AoD procedure triggered at end of CRC"]
    CRC = 1,
    #[doc = "0: Antenna switching/sampling is done in the packet payload"]
    PAYLOAD = 0,
}
impl From<DFEINEXTENSION_A> for bool {
    #[inline(always)]
    fn from(variant: DFEINEXTENSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEINEXTENSION` reader - Add CTE extension and do antenna switching/sampling in this extension"]
pub struct DFEINEXTENSION_R(crate::FieldReader<bool, DFEINEXTENSION_A>);
impl DFEINEXTENSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFEINEXTENSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEINEXTENSION_A {
        match self.bits {
            true => DFEINEXTENSION_A::CRC,
            false => DFEINEXTENSION_A::PAYLOAD,
        }
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        **self == DFEINEXTENSION_A::CRC
    }
    #[doc = "Checks if the value of the field is `PAYLOAD`"]
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        **self == DFEINEXTENSION_A::PAYLOAD
    }
}
impl core::ops::Deref for DFEINEXTENSION_R {
    type Target = crate::FieldReader<bool, DFEINEXTENSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFEINEXTENSION` writer - Add CTE extension and do antenna switching/sampling in this extension"]
pub struct DFEINEXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEINEXTENSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEINEXTENSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AoA/AoD procedure triggered at end of CRC"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(DFEINEXTENSION_A::CRC)
    }
    #[doc = "Antenna switching/sampling is done in the packet payload"]
    #[inline(always)]
    pub fn payload(self) -> &'a mut W {
        self.variant(DFEINEXTENSION_A::PAYLOAD)
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
#[doc = "Interval between every time the antenna is changed in the SWITCHING state\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSWITCHSPACING_A {
    #[doc = "1: 4us"]
    _4US = 1,
    #[doc = "2: 2us"]
    _2US = 2,
    #[doc = "3: 1us"]
    _1US = 3,
}
impl From<TSWITCHSPACING_A> for u8 {
    #[inline(always)]
    fn from(variant: TSWITCHSPACING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSWITCHSPACING` reader - Interval between every time the antenna is changed in the SWITCHING state"]
pub struct TSWITCHSPACING_R(crate::FieldReader<u8, TSWITCHSPACING_A>);
impl TSWITCHSPACING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSWITCHSPACING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSWITCHSPACING_A> {
        match self.bits {
            1 => Some(TSWITCHSPACING_A::_4US),
            2 => Some(TSWITCHSPACING_A::_2US),
            3 => Some(TSWITCHSPACING_A::_1US),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        **self == TSWITCHSPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        **self == TSWITCHSPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        **self == TSWITCHSPACING_A::_1US
    }
}
impl core::ops::Deref for TSWITCHSPACING_R {
    type Target = crate::FieldReader<u8, TSWITCHSPACING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSWITCHSPACING` writer - Interval between every time the antenna is changed in the SWITCHING state"]
pub struct TSWITCHSPACING_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWITCHSPACING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSWITCHSPACING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_1US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Interval between samples in the REFERENCE period\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSAMPLESPACINGREF_A {
    #[doc = "1: 4us"]
    _4US = 1,
    #[doc = "2: 2us"]
    _2US = 2,
    #[doc = "3: 1us"]
    _1US = 3,
    #[doc = "4: 0.5us"]
    _500NS = 4,
    #[doc = "5: 0.25us"]
    _250NS = 5,
    #[doc = "6: 0.125us"]
    _125NS = 6,
}
impl From<TSAMPLESPACINGREF_A> for u8 {
    #[inline(always)]
    fn from(variant: TSAMPLESPACINGREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSAMPLESPACINGREF` reader - Interval between samples in the REFERENCE period"]
pub struct TSAMPLESPACINGREF_R(crate::FieldReader<u8, TSAMPLESPACINGREF_A>);
impl TSAMPLESPACINGREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSAMPLESPACINGREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSAMPLESPACINGREF_A> {
        match self.bits {
            1 => Some(TSAMPLESPACINGREF_A::_4US),
            2 => Some(TSAMPLESPACINGREF_A::_2US),
            3 => Some(TSAMPLESPACINGREF_A::_1US),
            4 => Some(TSAMPLESPACINGREF_A::_500NS),
            5 => Some(TSAMPLESPACINGREF_A::_250NS),
            6 => Some(TSAMPLESPACINGREF_A::_125NS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        **self == TSAMPLESPACINGREF_A::_125NS
    }
}
impl core::ops::Deref for TSAMPLESPACINGREF_R {
    type Target = crate::FieldReader<u8, TSAMPLESPACINGREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAMPLESPACINGREF` writer - Interval between samples in the REFERENCE period"]
pub struct TSAMPLESPACINGREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLESPACINGREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSAMPLESPACINGREF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Whether to sample I/Q or magnitude/phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLETYPE_A {
    #[doc = "0: Complex samples in I and Q"]
    IQ = 0,
    #[doc = "1: Complex samples as magnitude and phase"]
    MAGPHASE = 1,
}
impl From<SAMPLETYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLETYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLETYPE` reader - Whether to sample I/Q or magnitude/phase"]
pub struct SAMPLETYPE_R(crate::FieldReader<bool, SAMPLETYPE_A>);
impl SAMPLETYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLETYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLETYPE_A {
        match self.bits {
            false => SAMPLETYPE_A::IQ,
            true => SAMPLETYPE_A::MAGPHASE,
        }
    }
    #[doc = "Checks if the value of the field is `IQ`"]
    #[inline(always)]
    pub fn is_iq(&self) -> bool {
        **self == SAMPLETYPE_A::IQ
    }
    #[doc = "Checks if the value of the field is `MAGPHASE`"]
    #[inline(always)]
    pub fn is_mag_phase(&self) -> bool {
        **self == SAMPLETYPE_A::MAGPHASE
    }
}
impl core::ops::Deref for SAMPLETYPE_R {
    type Target = crate::FieldReader<bool, SAMPLETYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLETYPE` writer - Whether to sample I/Q or magnitude/phase"]
pub struct SAMPLETYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLETYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLETYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Complex samples in I and Q"]
    #[inline(always)]
    pub fn iq(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::IQ)
    }
    #[doc = "Complex samples as magnitude and phase"]
    #[inline(always)]
    pub fn mag_phase(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::MAGPHASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSAMPLESPACING_A {
    #[doc = "1: 4us"]
    _4US = 1,
    #[doc = "2: 2us"]
    _2US = 2,
    #[doc = "3: 1us"]
    _1US = 3,
    #[doc = "4: 0.5us"]
    _500NS = 4,
    #[doc = "5: 0.25us"]
    _250NS = 5,
    #[doc = "6: 0.125us"]
    _125NS = 6,
}
impl From<TSAMPLESPACING_A> for u8 {
    #[inline(always)]
    fn from(variant: TSAMPLESPACING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSAMPLESPACING` reader - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub struct TSAMPLESPACING_R(crate::FieldReader<u8, TSAMPLESPACING_A>);
impl TSAMPLESPACING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSAMPLESPACING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSAMPLESPACING_A> {
        match self.bits {
            1 => Some(TSAMPLESPACING_A::_4US),
            2 => Some(TSAMPLESPACING_A::_2US),
            3 => Some(TSAMPLESPACING_A::_1US),
            4 => Some(TSAMPLESPACING_A::_500NS),
            5 => Some(TSAMPLESPACING_A::_250NS),
            6 => Some(TSAMPLESPACING_A::_125NS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        **self == TSAMPLESPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        **self == TSAMPLESPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        **self == TSAMPLESPACING_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        **self == TSAMPLESPACING_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        **self == TSAMPLESPACING_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        **self == TSAMPLESPACING_A::_125NS
    }
}
impl core::ops::Deref for TSAMPLESPACING_R {
    type Target = crate::FieldReader<u8, TSAMPLESPACING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAMPLESPACING` writer - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub struct TSAMPLESPACING_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLESPACING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSAMPLESPACING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REPEATPATTERN_A {
    #[doc = "0: Do not repeat (1 time in total)"]
    NOREPEAT = 0,
}
impl From<REPEATPATTERN_A> for u8 {
    #[inline(always)]
    fn from(variant: REPEATPATTERN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REPEATPATTERN` reader - Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
pub struct REPEATPATTERN_R(crate::FieldReader<u8, REPEATPATTERN_A>);
impl REPEATPATTERN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REPEATPATTERN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REPEATPATTERN_A> {
        match self.bits {
            0 => Some(REPEATPATTERN_A::NOREPEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPEAT`"]
    #[inline(always)]
    pub fn is_no_repeat(&self) -> bool {
        **self == REPEATPATTERN_A::NOREPEAT
    }
}
impl core::ops::Deref for REPEATPATTERN_R {
    type Target = crate::FieldReader<u8, REPEATPATTERN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPEATPATTERN` writer - Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
pub struct REPEATPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEATPATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPEATPATTERN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not repeat (1 time in total)"]
    #[inline(always)]
    pub fn no_repeat(self) -> &'a mut W {
        self.variant(REPEATPATTERN_A::NOREPEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `AGCBACKOFFGAIN` reader - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub struct AGCBACKOFFGAIN_R(crate::FieldReader<u8, u8>);
impl AGCBACKOFFGAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AGCBACKOFFGAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGCBACKOFFGAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGCBACKOFFGAIN` writer - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub struct AGCBACKOFFGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGCBACKOFFGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&self) -> NUMBEROF8US_R {
        NUMBEROF8US_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&self) -> DFEINEXTENSION_R {
        DFEINEXTENSION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&self) -> TSWITCHSPACING_R {
        TSWITCHSPACING_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&self) -> TSAMPLESPACINGREF_R {
        TSAMPLESPACINGREF_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&self) -> SAMPLETYPE_R {
        SAMPLETYPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&self) -> TSAMPLESPACING_R {
        TSAMPLESPACING_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
    #[inline(always)]
    pub fn repeatpattern(&self) -> REPEATPATTERN_R {
        REPEATPATTERN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&self) -> AGCBACKOFFGAIN_R {
        AGCBACKOFFGAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&mut self) -> NUMBEROF8US_W {
        NUMBEROF8US_W { w: self }
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&mut self) -> DFEINEXTENSION_W {
        DFEINEXTENSION_W { w: self }
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&mut self) -> TSWITCHSPACING_W {
        TSWITCHSPACING_W { w: self }
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&mut self) -> TSAMPLESPACINGREF_W {
        TSAMPLESPACINGREF_W { w: self }
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&mut self) -> SAMPLETYPE_W {
        SAMPLETYPE_W { w: self }
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&mut self) -> TSAMPLESPACING_W {
        TSAMPLESPACING_W { w: self }
    }
    #[doc = "Bits 20:23 - Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
    #[inline(always)]
    pub fn repeatpattern(&mut self) -> REPEATPATTERN_W {
        REPEATPATTERN_W { w: self }
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&mut self) -> AGCBACKOFFGAIN_W {
        AGCBACKOFFGAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various configuration for Direction finding\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfectrl1](index.html) module"]
pub struct DFECTRL1_SPEC;
impl crate::RegisterSpec for DFECTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfectrl1::R](R) reader structure"]
impl crate::Readable for DFECTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfectrl1::W](W) writer structure"]
impl crate::Writable for DFECTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFECTRL1 to value 0x0002_3282"]
impl crate::Resettable for DFECTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_3282
    }
}
