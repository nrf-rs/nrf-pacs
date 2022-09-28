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
pub type NUMBEROF8US_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMBEROF8US` writer - Length of the AoA/AoD procedure in number of 8 us units"]
pub type NUMBEROF8US_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFECTRL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `DFEINEXTENSION` reader - Add CTE extension and do antenna switching/sampling in this extension"]
pub type DFEINEXTENSION_R = crate::BitReader<DFEINEXTENSION_A>;
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
impl DFEINEXTENSION_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DFEINEXTENSION_A::CRC
    }
    #[doc = "Checks if the value of the field is `PAYLOAD`"]
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == DFEINEXTENSION_A::PAYLOAD
    }
}
#[doc = "Field `DFEINEXTENSION` writer - Add CTE extension and do antenna switching/sampling in this extension"]
pub type DFEINEXTENSION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFECTRL1_SPEC, DFEINEXTENSION_A, O>;
impl<'a, const O: u8> DFEINEXTENSION_W<'a, O> {
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
}
#[doc = "Field `TSWITCHSPACING` reader - Interval between every time the antenna is changed in the SWITCHING state"]
pub type TSWITCHSPACING_R = crate::FieldReader<u8, TSWITCHSPACING_A>;
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
impl TSWITCHSPACING_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSWITCHSPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSWITCHSPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSWITCHSPACING_A::_1US
    }
}
#[doc = "Field `TSWITCHSPACING` writer - Interval between every time the antenna is changed in the SWITCHING state"]
pub type TSWITCHSPACING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL1_SPEC, u8, TSWITCHSPACING_A, 3, O>;
impl<'a, const O: u8> TSWITCHSPACING_W<'a, O> {
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
}
#[doc = "Field `TSAMPLESPACINGREF` reader - Interval between samples in the REFERENCE period"]
pub type TSAMPLESPACINGREF_R = crate::FieldReader<u8, TSAMPLESPACINGREF_A>;
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
impl TSAMPLESPACINGREF_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSAMPLESPACINGREF_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_125NS
    }
}
#[doc = "Field `TSAMPLESPACINGREF` writer - Interval between samples in the REFERENCE period"]
pub type TSAMPLESPACINGREF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL1_SPEC, u8, TSAMPLESPACINGREF_A, 3, O>;
impl<'a, const O: u8> TSAMPLESPACINGREF_W<'a, O> {
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
}
#[doc = "Field `SAMPLETYPE` reader - Whether to sample I/Q or magnitude/phase"]
pub type SAMPLETYPE_R = crate::BitReader<SAMPLETYPE_A>;
#[doc = "Whether to sample I/Q or magnitude/phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLETYPE_A {
    #[doc = "0: Complex samples in I and Q"]
    IQ = 0,
    #[doc = "1: Complex samples as magnitude and phase"]
    MAG_PHASE = 1,
}
impl From<SAMPLETYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLETYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPLETYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLETYPE_A {
        match self.bits {
            false => SAMPLETYPE_A::IQ,
            true => SAMPLETYPE_A::MAG_PHASE,
        }
    }
    #[doc = "Checks if the value of the field is `IQ`"]
    #[inline(always)]
    pub fn is_iq(&self) -> bool {
        *self == SAMPLETYPE_A::IQ
    }
    #[doc = "Checks if the value of the field is `MAG_PHASE`"]
    #[inline(always)]
    pub fn is_mag_phase(&self) -> bool {
        *self == SAMPLETYPE_A::MAG_PHASE
    }
}
#[doc = "Field `SAMPLETYPE` writer - Whether to sample I/Q or magnitude/phase"]
pub type SAMPLETYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFECTRL1_SPEC, SAMPLETYPE_A, O>;
impl<'a, const O: u8> SAMPLETYPE_W<'a, O> {
    #[doc = "Complex samples in I and Q"]
    #[inline(always)]
    pub fn iq(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::IQ)
    }
    #[doc = "Complex samples as magnitude and phase"]
    #[inline(always)]
    pub fn mag_phase(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::MAG_PHASE)
    }
}
#[doc = "Field `TSAMPLESPACING` reader - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub type TSAMPLESPACING_R = crate::FieldReader<u8, TSAMPLESPACING_A>;
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
impl TSAMPLESPACING_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TSAMPLESPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSAMPLESPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSAMPLESPACING_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_125NS
    }
}
#[doc = "Field `TSAMPLESPACING` writer - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub type TSAMPLESPACING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL1_SPEC, u8, TSAMPLESPACING_A, 3, O>;
impl<'a, const O: u8> TSAMPLESPACING_W<'a, O> {
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
}
#[doc = "Field `AGCBACKOFFGAIN` reader - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub type AGCBACKOFFGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGCBACKOFFGAIN` writer - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub type AGCBACKOFFGAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&self) -> NUMBEROF8US_R {
        NUMBEROF8US_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&self) -> DFEINEXTENSION_R {
        DFEINEXTENSION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&self) -> TSWITCHSPACING_R {
        TSWITCHSPACING_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&self) -> TSAMPLESPACINGREF_R {
        TSAMPLESPACINGREF_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&self) -> SAMPLETYPE_R {
        SAMPLETYPE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&self) -> TSAMPLESPACING_R {
        TSAMPLESPACING_R::new(((self.bits >> 16) & 7) as u8)
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
    pub fn numberof8us(&mut self) -> NUMBEROF8US_W<0> {
        NUMBEROF8US_W::new(self)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&mut self) -> DFEINEXTENSION_W<7> {
        DFEINEXTENSION_W::new(self)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&mut self) -> TSWITCHSPACING_W<8> {
        TSWITCHSPACING_W::new(self)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&mut self) -> TSAMPLESPACINGREF_W<12> {
        TSAMPLESPACINGREF_W::new(self)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&mut self) -> SAMPLETYPE_W<15> {
        SAMPLETYPE_W::new(self)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&mut self) -> TSAMPLESPACING_W<16> {
        TSAMPLESPACING_W::new(self)
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&mut self) -> AGCBACKOFFGAIN_W<24> {
        AGCBACKOFFGAIN_W::new(self)
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
