#[doc = "Register `CTEINLINECONF` reader"]
pub struct R(crate::R<CTEINLINECONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTEINLINECONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTEINLINECONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTEINLINECONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTEINLINECONF` writer"]
pub struct W(crate::W<CTEINLINECONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTEINLINECONF_SPEC>;
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
impl From<crate::W<CTEINLINECONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTEINLINECONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEINLINECTRLEN` reader - Enable parsing of CTEInfo from received packet in BLE modes"]
pub type CTEINLINECTRLEN_R = crate::BitReader<CTEINLINECTRLEN_A>;
#[doc = "Enable parsing of CTEInfo from received packet in BLE modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINLINECTRLEN_A {
    #[doc = "1: Parsing of CTEInfo is enabled"]
    ENABLED = 1,
    #[doc = "0: Parsing of CTEInfo is disabled"]
    DISABLED = 0,
}
impl From<CTEINLINECTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTEINLINECTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEINLINECTRLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEINLINECTRLEN_A {
        match self.bits {
            true => CTEINLINECTRLEN_A::ENABLED,
            false => CTEINLINECTRLEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTEINLINECTRLEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTEINLINECTRLEN_A::DISABLED
    }
}
#[doc = "Field `CTEINLINECTRLEN` writer - Enable parsing of CTEInfo from received packet in BLE modes"]
pub type CTEINLINECTRLEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTEINLINECONF_SPEC, CTEINLINECTRLEN_A, O>;
impl<'a, const O: u8> CTEINLINECTRLEN_W<'a, O> {
    #[doc = "Parsing of CTEInfo is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTEINLINECTRLEN_A::ENABLED)
    }
    #[doc = "Parsing of CTEInfo is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTEINLINECTRLEN_A::DISABLED)
    }
}
#[doc = "Field `CTEINFOINS1` reader - CTEInfo is S1 byte or not"]
pub type CTEINFOINS1_R = crate::BitReader<CTEINFOINS1_A>;
#[doc = "CTEInfo is S1 byte or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINFOINS1_A {
    #[doc = "1: CTEInfo is in S1 byte (data PDU)"]
    IN_S1 = 1,
    #[doc = "0: CTEInfo is NOT in S1 byte (advertising PDU)"]
    NOT_IN_S1 = 0,
}
impl From<CTEINFOINS1_A> for bool {
    #[inline(always)]
    fn from(variant: CTEINFOINS1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEINFOINS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEINFOINS1_A {
        match self.bits {
            true => CTEINFOINS1_A::IN_S1,
            false => CTEINFOINS1_A::NOT_IN_S1,
        }
    }
    #[doc = "Checks if the value of the field is `IN_S1`"]
    #[inline(always)]
    pub fn is_in_s1(&self) -> bool {
        *self == CTEINFOINS1_A::IN_S1
    }
    #[doc = "Checks if the value of the field is `NOT_IN_S1`"]
    #[inline(always)]
    pub fn is_not_in_s1(&self) -> bool {
        *self == CTEINFOINS1_A::NOT_IN_S1
    }
}
#[doc = "Field `CTEINFOINS1` writer - CTEInfo is S1 byte or not"]
pub type CTEINFOINS1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTEINLINECONF_SPEC, CTEINFOINS1_A, O>;
impl<'a, const O: u8> CTEINFOINS1_W<'a, O> {
    #[doc = "CTEInfo is in S1 byte (data PDU)"]
    #[inline(always)]
    pub fn in_s1(self) -> &'a mut W {
        self.variant(CTEINFOINS1_A::IN_S1)
    }
    #[doc = "CTEInfo is NOT in S1 byte (advertising PDU)"]
    #[inline(always)]
    pub fn not_in_s1(self) -> &'a mut W {
        self.variant(CTEINFOINS1_A::NOT_IN_S1)
    }
}
#[doc = "Field `CTEERRORHANDLING` reader - Sampling/switching if CRC is not OK"]
pub type CTEERRORHANDLING_R = crate::BitReader<CTEERRORHANDLING_A>;
#[doc = "Sampling/switching if CRC is not OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEERRORHANDLING_A {
    #[doc = "1: Sampling and antenna switching also when CRC is not OK"]
    YES = 1,
    #[doc = "0: No sampling and antenna switching when CRC is not OK"]
    NO = 0,
}
impl From<CTEERRORHANDLING_A> for bool {
    #[inline(always)]
    fn from(variant: CTEERRORHANDLING_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEERRORHANDLING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEERRORHANDLING_A {
        match self.bits {
            true => CTEERRORHANDLING_A::YES,
            false => CTEERRORHANDLING_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CTEERRORHANDLING_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CTEERRORHANDLING_A::NO
    }
}
#[doc = "Field `CTEERRORHANDLING` writer - Sampling/switching if CRC is not OK"]
pub type CTEERRORHANDLING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTEINLINECONF_SPEC, CTEERRORHANDLING_A, O>;
impl<'a, const O: u8> CTEERRORHANDLING_W<'a, O> {
    #[doc = "Sampling and antenna switching also when CRC is not OK"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CTEERRORHANDLING_A::YES)
    }
    #[doc = "No sampling and antenna switching when CRC is not OK"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CTEERRORHANDLING_A::NO)
    }
}
#[doc = "Field `CTETIMEVALIDRANGE` reader - Max range of CTETime"]
pub type CTETIMEVALIDRANGE_R = crate::FieldReader<u8, CTETIMEVALIDRANGE_A>;
#[doc = "Max range of CTETime\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTETIMEVALIDRANGE_A {
    #[doc = "0: 20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    _20 = 0,
    #[doc = "1: 31 in 8 us unit"]
    _31 = 1,
    #[doc = "2: 63 in 8 us unit"]
    _63 = 2,
}
impl From<CTETIMEVALIDRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTETIMEVALIDRANGE_A) -> Self {
        variant as _
    }
}
impl CTETIMEVALIDRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTETIMEVALIDRANGE_A> {
        match self.bits {
            0 => Some(CTETIMEVALIDRANGE_A::_20),
            1 => Some(CTETIMEVALIDRANGE_A::_31),
            2 => Some(CTETIMEVALIDRANGE_A::_63),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_20
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_31
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_63
    }
}
#[doc = "Field `CTETIMEVALIDRANGE` writer - Max range of CTETime"]
pub type CTETIMEVALIDRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTEINLINECONF_SPEC, u8, CTETIMEVALIDRANGE_A, 2, O>;
impl<'a, const O: u8> CTETIMEVALIDRANGE_W<'a, O> {
    #[doc = "20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_20)
    }
    #[doc = "31 in 8 us unit"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_31)
    }
    #[doc = "63 in 8 us unit"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_63)
    }
}
#[doc = "Field `CTEINLINERXMODE1US` reader - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type CTEINLINERXMODE1US_R = crate::FieldReader<u8, CTEINLINERXMODE1US_A>;
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTEINLINERXMODE1US_A {
    #[doc = "1: 4 us"]
    _4US = 1,
    #[doc = "2: 2 us"]
    _2US = 2,
    #[doc = "3: 1 us"]
    _1US = 3,
    #[doc = "4: 0.5 us"]
    _500NS = 4,
    #[doc = "5: 0.25 us"]
    _250NS = 5,
    #[doc = "6: 0.125 us"]
    _125NS = 6,
}
impl From<CTEINLINERXMODE1US_A> for u8 {
    #[inline(always)]
    fn from(variant: CTEINLINERXMODE1US_A) -> Self {
        variant as _
    }
}
impl CTEINLINERXMODE1US_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEINLINERXMODE1US_A> {
        match self.bits {
            1 => Some(CTEINLINERXMODE1US_A::_4US),
            2 => Some(CTEINLINERXMODE1US_A::_2US),
            3 => Some(CTEINLINERXMODE1US_A::_1US),
            4 => Some(CTEINLINERXMODE1US_A::_500NS),
            5 => Some(CTEINLINERXMODE1US_A::_250NS),
            6 => Some(CTEINLINERXMODE1US_A::_125NS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_125NS
    }
}
#[doc = "Field `CTEINLINERXMODE1US` writer - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type CTEINLINERXMODE1US_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTEINLINECONF_SPEC, u8, CTEINLINERXMODE1US_A, 3, O>;
impl<'a, const O: u8> CTEINLINERXMODE1US_W<'a, O> {
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_4US)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_2US)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_1US)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_500NS)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_250NS)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_125NS)
    }
}
#[doc = "Field `CTEINLINERXMODE2US` reader - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type CTEINLINERXMODE2US_R = crate::FieldReader<u8, CTEINLINERXMODE2US_A>;
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTEINLINERXMODE2US_A {
    #[doc = "1: 4 us"]
    _4US = 1,
    #[doc = "2: 2 us"]
    _2US = 2,
    #[doc = "3: 1 us"]
    _1US = 3,
    #[doc = "4: 0.5 us"]
    _500NS = 4,
    #[doc = "5: 0.25 us"]
    _250NS = 5,
    #[doc = "6: 0.125 us"]
    _125NS = 6,
}
impl From<CTEINLINERXMODE2US_A> for u8 {
    #[inline(always)]
    fn from(variant: CTEINLINERXMODE2US_A) -> Self {
        variant as _
    }
}
impl CTEINLINERXMODE2US_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEINLINERXMODE2US_A> {
        match self.bits {
            1 => Some(CTEINLINERXMODE2US_A::_4US),
            2 => Some(CTEINLINERXMODE2US_A::_2US),
            3 => Some(CTEINLINERXMODE2US_A::_1US),
            4 => Some(CTEINLINERXMODE2US_A::_500NS),
            5 => Some(CTEINLINERXMODE2US_A::_250NS),
            6 => Some(CTEINLINERXMODE2US_A::_125NS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_125NS
    }
}
#[doc = "Field `CTEINLINERXMODE2US` writer - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type CTEINLINERXMODE2US_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTEINLINECONF_SPEC, u8, CTEINLINERXMODE2US_A, 3, O>;
impl<'a, const O: u8> CTEINLINERXMODE2US_W<'a, O> {
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_4US)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_2US)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_1US)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_500NS)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_250NS)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_125NS)
    }
}
#[doc = "Field `S0CONF` reader - S0 bit pattern to match"]
pub type S0CONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S0CONF` writer - S0 bit pattern to match"]
pub type S0CONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTEINLINECONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `S0MASK` reader - S0 bit mask to set which bit to match"]
pub type S0MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S0MASK` writer - S0 bit mask to set which bit to match"]
pub type S0MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTEINLINECONF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&self) -> CTEINLINECTRLEN_R {
        CTEINLINECTRLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&self) -> CTEINFOINS1_R {
        CTEINFOINS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&self) -> CTEERRORHANDLING_R {
        CTEERRORHANDLING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&self) -> CTETIMEVALIDRANGE_R {
        CTETIMEVALIDRANGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&self) -> CTEINLINERXMODE1US_R {
        CTEINLINERXMODE1US_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&self) -> CTEINLINERXMODE2US_R {
        CTEINLINERXMODE2US_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&self) -> S0CONF_R {
        S0CONF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&self) -> S0MASK_R {
        S0MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&mut self) -> CTEINLINECTRLEN_W<0> {
        CTEINLINECTRLEN_W::new(self)
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&mut self) -> CTEINFOINS1_W<3> {
        CTEINFOINS1_W::new(self)
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&mut self) -> CTEERRORHANDLING_W<4> {
        CTEERRORHANDLING_W::new(self)
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&mut self) -> CTETIMEVALIDRANGE_W<6> {
        CTETIMEVALIDRANGE_W::new(self)
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&mut self) -> CTEINLINERXMODE1US_W<10> {
        CTEINLINERXMODE1US_W::new(self)
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&mut self) -> CTEINLINERXMODE2US_W<13> {
        CTEINLINERXMODE2US_W::new(self)
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&mut self) -> S0CONF_W<16> {
        S0CONF_W::new(self)
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&mut self) -> S0MASK_W<24> {
        S0MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for CTE inline mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cteinlineconf](index.html) module"]
pub struct CTEINLINECONF_SPEC;
impl crate::RegisterSpec for CTEINLINECONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cteinlineconf::R](R) reader structure"]
impl crate::Readable for CTEINLINECONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cteinlineconf::W](W) writer structure"]
impl crate::Writable for CTEINLINECONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTEINLINECONF to value 0x2800"]
impl crate::Resettable for CTEINLINECONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2800
    }
}
