#[doc = "Register `MCKFREQ` reader"]
pub struct R(crate::R<MCKFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCKFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCKFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCKFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCKFREQ` writer"]
pub struct W(crate::W<MCKFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCKFREQ_SPEC>;
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
impl From<crate::W<MCKFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCKFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero.\n\nValue on reset: 536870912"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum MCKFREQ_A {
    #[doc = "2147483648: 32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV2 = 2147483648,
    #[doc = "1342177280: 32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV3 = 1342177280,
    #[doc = "1073741824: 32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV4 = 1073741824,
    #[doc = "805306368: 32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV5 = 805306368,
    #[doc = "671088640: 32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV6 = 671088640,
    #[doc = "536870912: 32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV8 = 536870912,
    #[doc = "402653184: 32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV10 = 402653184,
    #[doc = "369098752: 32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV11 = 369098752,
    #[doc = "285212672: 32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV15 = 285212672,
    #[doc = "268435456: 32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV16 = 268435456,
    #[doc = "201326592: 32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV21 = 201326592,
    #[doc = "184549376: 32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV23 = 184549376,
    #[doc = "142606336: 32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV30 = 142606336,
    #[doc = "138412032: 32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV31 = 138412032,
    #[doc = "134217728: 32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV32 = 134217728,
    #[doc = "100663296: 32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV42 = 100663296,
    #[doc = "68157440: 32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV63 = 68157440,
    #[doc = "34340864: 32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
    _32MDIV125 = 34340864,
}
impl From<MCKFREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: MCKFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCKFREQ` reader - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
pub struct MCKFREQ_R(crate::FieldReader<u32, MCKFREQ_A>);
impl MCKFREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        MCKFREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCKFREQ_A> {
        match self.bits {
            2147483648 => Some(MCKFREQ_A::_32MDIV2),
            1342177280 => Some(MCKFREQ_A::_32MDIV3),
            1073741824 => Some(MCKFREQ_A::_32MDIV4),
            805306368 => Some(MCKFREQ_A::_32MDIV5),
            671088640 => Some(MCKFREQ_A::_32MDIV6),
            536870912 => Some(MCKFREQ_A::_32MDIV8),
            402653184 => Some(MCKFREQ_A::_32MDIV10),
            369098752 => Some(MCKFREQ_A::_32MDIV11),
            285212672 => Some(MCKFREQ_A::_32MDIV15),
            268435456 => Some(MCKFREQ_A::_32MDIV16),
            201326592 => Some(MCKFREQ_A::_32MDIV21),
            184549376 => Some(MCKFREQ_A::_32MDIV23),
            142606336 => Some(MCKFREQ_A::_32MDIV30),
            138412032 => Some(MCKFREQ_A::_32MDIV31),
            134217728 => Some(MCKFREQ_A::_32MDIV32),
            100663296 => Some(MCKFREQ_A::_32MDIV42),
            68157440 => Some(MCKFREQ_A::_32MDIV63),
            34340864 => Some(MCKFREQ_A::_32MDIV125),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_32MDIV2`"]
    #[inline(always)]
    pub fn is_32mdiv2(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV2
    }
    #[doc = "Checks if the value of the field is `_32MDIV3`"]
    #[inline(always)]
    pub fn is_32mdiv3(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV3
    }
    #[doc = "Checks if the value of the field is `_32MDIV4`"]
    #[inline(always)]
    pub fn is_32mdiv4(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV4
    }
    #[doc = "Checks if the value of the field is `_32MDIV5`"]
    #[inline(always)]
    pub fn is_32mdiv5(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV5
    }
    #[doc = "Checks if the value of the field is `_32MDIV6`"]
    #[inline(always)]
    pub fn is_32mdiv6(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV6
    }
    #[doc = "Checks if the value of the field is `_32MDIV8`"]
    #[inline(always)]
    pub fn is_32mdiv8(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV8
    }
    #[doc = "Checks if the value of the field is `_32MDIV10`"]
    #[inline(always)]
    pub fn is_32mdiv10(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV10
    }
    #[doc = "Checks if the value of the field is `_32MDIV11`"]
    #[inline(always)]
    pub fn is_32mdiv11(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV11
    }
    #[doc = "Checks if the value of the field is `_32MDIV15`"]
    #[inline(always)]
    pub fn is_32mdiv15(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV15
    }
    #[doc = "Checks if the value of the field is `_32MDIV16`"]
    #[inline(always)]
    pub fn is_32mdiv16(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV16
    }
    #[doc = "Checks if the value of the field is `_32MDIV21`"]
    #[inline(always)]
    pub fn is_32mdiv21(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV21
    }
    #[doc = "Checks if the value of the field is `_32MDIV23`"]
    #[inline(always)]
    pub fn is_32mdiv23(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV23
    }
    #[doc = "Checks if the value of the field is `_32MDIV30`"]
    #[inline(always)]
    pub fn is_32mdiv30(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV30
    }
    #[doc = "Checks if the value of the field is `_32MDIV31`"]
    #[inline(always)]
    pub fn is_32mdiv31(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV31
    }
    #[doc = "Checks if the value of the field is `_32MDIV32`"]
    #[inline(always)]
    pub fn is_32mdiv32(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV32
    }
    #[doc = "Checks if the value of the field is `_32MDIV42`"]
    #[inline(always)]
    pub fn is_32mdiv42(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV42
    }
    #[doc = "Checks if the value of the field is `_32MDIV63`"]
    #[inline(always)]
    pub fn is_32mdiv63(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV63
    }
    #[doc = "Checks if the value of the field is `_32MDIV125`"]
    #[inline(always)]
    pub fn is_32mdiv125(&self) -> bool {
        **self == MCKFREQ_A::_32MDIV125
    }
}
impl core::ops::Deref for MCKFREQ_R {
    type Target = crate::FieldReader<u32, MCKFREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKFREQ` writer - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
pub struct MCKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKFREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv2(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV2)
    }
    #[doc = "32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv3(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV3)
    }
    #[doc = "32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv4(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV4)
    }
    #[doc = "32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv5(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV5)
    }
    #[doc = "32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv6(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV6)
    }
    #[doc = "32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv8(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV8)
    }
    #[doc = "32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv10(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV10)
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv11(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV11)
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv15(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV15)
    }
    #[doc = "32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv16(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV16)
    }
    #[doc = "32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv21(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV21)
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv23(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV23)
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv30(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV30)
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv31(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV31)
    }
    #[doc = "32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv32(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV32)
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv42(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV42)
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv63(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV63)
    }
    #[doc = "32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv125(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV125)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn mckfreq(&self) -> MCKFREQ_R {
        MCKFREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn mckfreq(&mut self) -> MCKFREQ_W {
        MCKFREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S clock generator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mckfreq](index.html) module"]
pub struct MCKFREQ_SPEC;
impl crate::RegisterSpec for MCKFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mckfreq::R](R) reader structure"]
impl crate::Readable for MCKFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mckfreq::W](W) writer structure"]
impl crate::Writable for MCKFREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCKFREQ to value 0x2000_0000"]
impl crate::Resettable for MCKFREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
