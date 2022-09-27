#[doc = "Register `IFCONFIG1` reader"]
pub struct R(crate::R<IFCONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFCONFIG1` writer"]
pub struct W(crate::W<IFCONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCONFIG1_SPEC>;
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
impl From<crate::W<IFCONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDELAY` reader - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
pub type SCKDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCKDELAY` writer - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
pub type SCKDELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFCONFIG1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DPMEN` reader - Enter/exit deep power-down mode (DPM) for external flash memory."]
pub type DPMEN_R = crate::BitReader<DPMEN_A>;
#[doc = "Enter/exit deep power-down mode (DPM) for external flash memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMEN_A {
    #[doc = "0: Exit DPM."]
    EXIT = 0,
    #[doc = "1: Enter DPM."]
    ENTER = 1,
}
impl From<DPMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DPMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPMEN_A {
        match self.bits {
            false => DPMEN_A::EXIT,
            true => DPMEN_A::ENTER,
        }
    }
    #[doc = "Checks if the value of the field is `EXIT`"]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == DPMEN_A::EXIT
    }
    #[doc = "Checks if the value of the field is `ENTER`"]
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == DPMEN_A::ENTER
    }
}
#[doc = "Field `DPMEN` writer - Enter/exit deep power-down mode (DPM) for external flash memory."]
pub type DPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCONFIG1_SPEC, DPMEN_A, O>;
impl<'a, const O: u8> DPMEN_W<'a, O> {
    #[doc = "Exit DPM."]
    #[inline(always)]
    pub fn exit(self) -> &'a mut W {
        self.variant(DPMEN_A::EXIT)
    }
    #[doc = "Enter DPM."]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(DPMEN_A::ENTER)
    }
}
#[doc = "Field `SPIMODE` reader - Select SPI mode."]
pub type SPIMODE_R = crate::BitReader<SPIMODE_A>;
#[doc = "Select SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMODE_A {
    #[doc = "0: Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    MODE0 = 0,
    #[doc = "1: Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    MODE3 = 1,
}
impl From<SPIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMODE_A {
        match self.bits {
            false => SPIMODE_A::MODE0,
            true => SPIMODE_A::MODE3,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == SPIMODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == SPIMODE_A::MODE3
    }
}
#[doc = "Field `SPIMODE` writer - Select SPI mode."]
pub type SPIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCONFIG1_SPEC, SPIMODE_A, O>;
impl<'a, const O: u8> SPIMODE_W<'a, O> {
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(SPIMODE_A::MODE0)
    }
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(SPIMODE_A::MODE3)
    }
}
#[doc = "Field `SCKFREQ` reader - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
pub type SCKFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCKFREQ` writer - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
pub type SCKFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFCONFIG1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&self) -> SCKDELAY_R {
        SCKDELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&self) -> DPMEN_R {
        DPMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub fn sckfreq(&self) -> SCKFREQ_R {
        SCKFREQ_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&mut self) -> SCKDELAY_W<0> {
        SCKDELAY_W::new(self)
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&mut self) -> DPMEN_W<24> {
        DPMEN_W::new(self)
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&mut self) -> SPIMODE_W<25> {
        SPIMODE_W::new(self)
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub fn sckfreq(&mut self) -> SCKFREQ_W<28> {
        SCKFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifconfig1](index.html) module"]
pub struct IFCONFIG1_SPEC;
impl crate::RegisterSpec for IFCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifconfig1::R](R) reader structure"]
impl crate::Readable for IFCONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifconfig1::W](W) writer structure"]
impl crate::Writable for IFCONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCONFIG1 to value 0x0004_0480"]
impl crate::Resettable for IFCONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0480
    }
}
