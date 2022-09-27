#[doc = "Register `BAUDRATE` reader"]
pub struct R(crate::R<BAUDRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDRATE` writer"]
pub struct W(crate::W<BAUDRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDRATE_SPEC>;
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
impl From<crate::W<BAUDRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDRATE` reader - UART baudrate."]
pub type BAUDRATE_R = crate::FieldReader<u32, BAUDRATE_A>;
#[doc = "UART baudrate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BAUDRATE_A {
    #[doc = "323584: 1200 baud."]
    BAUD1200 = 323584,
    #[doc = "643072: 2400 baud."]
    BAUD2400 = 643072,
    #[doc = "1290240: 4800 baud."]
    BAUD4800 = 1290240,
    #[doc = "2576384: 9600 baud."]
    BAUD9600 = 2576384,
    #[doc = "3866624: 14400 baud."]
    BAUD14400 = 3866624,
    #[doc = "5152768: 19200 baud."]
    BAUD19200 = 5152768,
    #[doc = "7729152: 28800 baud."]
    BAUD28800 = 7729152,
    #[doc = "8388608: 31250 baud."]
    BAUD31250 = 8388608,
    #[doc = "10309632: 38400 baud."]
    BAUD38400 = 10309632,
    #[doc = "15007744: 56000 baud."]
    BAUD56000 = 15007744,
    #[doc = "15462400: 57600 baud."]
    BAUD57600 = 15462400,
    #[doc = "20615168: 76800 baud."]
    BAUD76800 = 20615168,
    #[doc = "30924800: 115200 baud."]
    BAUD115200 = 30924800,
    #[doc = "61845504: 230400 baud."]
    BAUD230400 = 61845504,
    #[doc = "67108864: 250000 baud."]
    BAUD250000 = 67108864,
    #[doc = "123695104: 460800 baud."]
    BAUD460800 = 123695104,
    #[doc = "247386112: 921600 baud."]
    BAUD921600 = 247386112,
    #[doc = "268435456: 1M baud."]
    BAUD1M = 268435456,
}
impl From<BAUDRATE_A> for u32 {
    #[inline(always)]
    fn from(variant: BAUDRATE_A) -> Self {
        variant as _
    }
}
impl BAUDRATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAUDRATE_A> {
        match self.bits {
            323584 => Some(BAUDRATE_A::BAUD1200),
            643072 => Some(BAUDRATE_A::BAUD2400),
            1290240 => Some(BAUDRATE_A::BAUD4800),
            2576384 => Some(BAUDRATE_A::BAUD9600),
            3866624 => Some(BAUDRATE_A::BAUD14400),
            5152768 => Some(BAUDRATE_A::BAUD19200),
            7729152 => Some(BAUDRATE_A::BAUD28800),
            8388608 => Some(BAUDRATE_A::BAUD31250),
            10309632 => Some(BAUDRATE_A::BAUD38400),
            15007744 => Some(BAUDRATE_A::BAUD56000),
            15462400 => Some(BAUDRATE_A::BAUD57600),
            20615168 => Some(BAUDRATE_A::BAUD76800),
            30924800 => Some(BAUDRATE_A::BAUD115200),
            61845504 => Some(BAUDRATE_A::BAUD230400),
            67108864 => Some(BAUDRATE_A::BAUD250000),
            123695104 => Some(BAUDRATE_A::BAUD460800),
            247386112 => Some(BAUDRATE_A::BAUD921600),
            268435456 => Some(BAUDRATE_A::BAUD1M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BAUD1200`"]
    #[inline(always)]
    pub fn is_baud1200(&self) -> bool {
        *self == BAUDRATE_A::BAUD1200
    }
    #[doc = "Checks if the value of the field is `BAUD2400`"]
    #[inline(always)]
    pub fn is_baud2400(&self) -> bool {
        *self == BAUDRATE_A::BAUD2400
    }
    #[doc = "Checks if the value of the field is `BAUD4800`"]
    #[inline(always)]
    pub fn is_baud4800(&self) -> bool {
        *self == BAUDRATE_A::BAUD4800
    }
    #[doc = "Checks if the value of the field is `BAUD9600`"]
    #[inline(always)]
    pub fn is_baud9600(&self) -> bool {
        *self == BAUDRATE_A::BAUD9600
    }
    #[doc = "Checks if the value of the field is `BAUD14400`"]
    #[inline(always)]
    pub fn is_baud14400(&self) -> bool {
        *self == BAUDRATE_A::BAUD14400
    }
    #[doc = "Checks if the value of the field is `BAUD19200`"]
    #[inline(always)]
    pub fn is_baud19200(&self) -> bool {
        *self == BAUDRATE_A::BAUD19200
    }
    #[doc = "Checks if the value of the field is `BAUD28800`"]
    #[inline(always)]
    pub fn is_baud28800(&self) -> bool {
        *self == BAUDRATE_A::BAUD28800
    }
    #[doc = "Checks if the value of the field is `BAUD31250`"]
    #[inline(always)]
    pub fn is_baud31250(&self) -> bool {
        *self == BAUDRATE_A::BAUD31250
    }
    #[doc = "Checks if the value of the field is `BAUD38400`"]
    #[inline(always)]
    pub fn is_baud38400(&self) -> bool {
        *self == BAUDRATE_A::BAUD38400
    }
    #[doc = "Checks if the value of the field is `BAUD56000`"]
    #[inline(always)]
    pub fn is_baud56000(&self) -> bool {
        *self == BAUDRATE_A::BAUD56000
    }
    #[doc = "Checks if the value of the field is `BAUD57600`"]
    #[inline(always)]
    pub fn is_baud57600(&self) -> bool {
        *self == BAUDRATE_A::BAUD57600
    }
    #[doc = "Checks if the value of the field is `BAUD76800`"]
    #[inline(always)]
    pub fn is_baud76800(&self) -> bool {
        *self == BAUDRATE_A::BAUD76800
    }
    #[doc = "Checks if the value of the field is `BAUD115200`"]
    #[inline(always)]
    pub fn is_baud115200(&self) -> bool {
        *self == BAUDRATE_A::BAUD115200
    }
    #[doc = "Checks if the value of the field is `BAUD230400`"]
    #[inline(always)]
    pub fn is_baud230400(&self) -> bool {
        *self == BAUDRATE_A::BAUD230400
    }
    #[doc = "Checks if the value of the field is `BAUD250000`"]
    #[inline(always)]
    pub fn is_baud250000(&self) -> bool {
        *self == BAUDRATE_A::BAUD250000
    }
    #[doc = "Checks if the value of the field is `BAUD460800`"]
    #[inline(always)]
    pub fn is_baud460800(&self) -> bool {
        *self == BAUDRATE_A::BAUD460800
    }
    #[doc = "Checks if the value of the field is `BAUD921600`"]
    #[inline(always)]
    pub fn is_baud921600(&self) -> bool {
        *self == BAUDRATE_A::BAUD921600
    }
    #[doc = "Checks if the value of the field is `BAUD1M`"]
    #[inline(always)]
    pub fn is_baud1m(&self) -> bool {
        *self == BAUDRATE_A::BAUD1M
    }
}
#[doc = "Field `BAUDRATE` writer - UART baudrate."]
pub type BAUDRATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAUDRATE_SPEC, u32, BAUDRATE_A, 32, O>;
impl<'a, const O: u8> BAUDRATE_W<'a, O> {
    #[doc = "1200 baud."]
    #[inline(always)]
    pub fn baud1200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1200)
    }
    #[doc = "2400 baud."]
    #[inline(always)]
    pub fn baud2400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD2400)
    }
    #[doc = "4800 baud."]
    #[inline(always)]
    pub fn baud4800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD4800)
    }
    #[doc = "9600 baud."]
    #[inline(always)]
    pub fn baud9600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD9600)
    }
    #[doc = "14400 baud."]
    #[inline(always)]
    pub fn baud14400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD14400)
    }
    #[doc = "19200 baud."]
    #[inline(always)]
    pub fn baud19200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD19200)
    }
    #[doc = "28800 baud."]
    #[inline(always)]
    pub fn baud28800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD28800)
    }
    #[doc = "31250 baud."]
    #[inline(always)]
    pub fn baud31250(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD31250)
    }
    #[doc = "38400 baud."]
    #[inline(always)]
    pub fn baud38400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD38400)
    }
    #[doc = "56000 baud."]
    #[inline(always)]
    pub fn baud56000(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD56000)
    }
    #[doc = "57600 baud."]
    #[inline(always)]
    pub fn baud57600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD57600)
    }
    #[doc = "76800 baud."]
    #[inline(always)]
    pub fn baud76800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD76800)
    }
    #[doc = "115200 baud."]
    #[inline(always)]
    pub fn baud115200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD115200)
    }
    #[doc = "230400 baud."]
    #[inline(always)]
    pub fn baud230400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD230400)
    }
    #[doc = "250000 baud."]
    #[inline(always)]
    pub fn baud250000(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD250000)
    }
    #[doc = "460800 baud."]
    #[inline(always)]
    pub fn baud460800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD460800)
    }
    #[doc = "921600 baud."]
    #[inline(always)]
    pub fn baud921600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD921600)
    }
    #[doc = "1M baud."]
    #[inline(always)]
    pub fn baud1m(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1M)
    }
}
impl R {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&self) -> BAUDRATE_R {
        BAUDRATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&mut self) -> BAUDRATE_W<0> {
        BAUDRATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baudrate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudrate](index.html) module"]
pub struct BAUDRATE_SPEC;
impl crate::RegisterSpec for BAUDRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baudrate::R](R) reader structure"]
impl crate::Readable for BAUDRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudrate::W](W) writer structure"]
impl crate::Writable for BAUDRATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUDRATE to value 0"]
impl crate::Resettable for BAUDRATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
