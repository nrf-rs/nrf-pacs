#[doc = "Register `RAMONB` reader"]
pub struct R(crate::R<RAMONB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMONB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMONB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMONB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMONB` writer"]
pub struct W(crate::W<RAMONB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMONB_SPEC>;
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
impl From<crate::W<RAMONB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMONB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Keep RAM block 2 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM2_A {
    #[doc = "0: Off"]
    RAM2OFF = 0,
    #[doc = "1: On"]
    RAM2ON = 1,
}
impl From<ONRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM2` reader - Keep RAM block 2 on or off in system ON Mode"]
pub struct ONRAM2_R(crate::FieldReader<bool, ONRAM2_A>);
impl ONRAM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONRAM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM2_A {
        match self.bits {
            false => ONRAM2_A::RAM2OFF,
            true => ONRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        **self == ONRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        **self == ONRAM2_A::RAM2ON
    }
}
impl core::ops::Deref for ONRAM2_R {
    type Target = crate::FieldReader<bool, ONRAM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONRAM2` writer - Keep RAM block 2 on or off in system ON Mode"]
pub struct ONRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2ON)
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
#[doc = "Keep RAM block 3 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM3_A {
    #[doc = "0: Off"]
    RAM3OFF = 0,
    #[doc = "1: On"]
    RAM3ON = 1,
}
impl From<ONRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM3` reader - Keep RAM block 3 on or off in system ON Mode"]
pub struct ONRAM3_R(crate::FieldReader<bool, ONRAM3_A>);
impl ONRAM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONRAM3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM3_A {
        match self.bits {
            false => ONRAM3_A::RAM3OFF,
            true => ONRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        **self == ONRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        **self == ONRAM3_A::RAM3ON
    }
}
impl core::ops::Deref for ONRAM3_R {
    type Target = crate::FieldReader<bool, ONRAM3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONRAM3` writer - Keep RAM block 3 on or off in system ON Mode"]
pub struct ONRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3ON)
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
#[doc = "Keep retention on RAM block 2 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM2_A {
    #[doc = "0: Off"]
    RAM2OFF = 0,
    #[doc = "1: On"]
    RAM2ON = 1,
}
impl From<OFFRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM2` reader - Keep retention on RAM block 2 when RAM block is switched off"]
pub struct OFFRAM2_R(crate::FieldReader<bool, OFFRAM2_A>);
impl OFFRAM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFRAM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM2_A {
        match self.bits {
            false => OFFRAM2_A::RAM2OFF,
            true => OFFRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        **self == OFFRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        **self == OFFRAM2_A::RAM2ON
    }
}
impl core::ops::Deref for OFFRAM2_R {
    type Target = crate::FieldReader<bool, OFFRAM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFRAM2` writer - Keep retention on RAM block 2 when RAM block is switched off"]
pub struct OFFRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Keep retention on RAM block 3 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM3_A {
    #[doc = "0: Off"]
    RAM3OFF = 0,
    #[doc = "1: On"]
    RAM3ON = 1,
}
impl From<OFFRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM3` reader - Keep retention on RAM block 3 when RAM block is switched off"]
pub struct OFFRAM3_R(crate::FieldReader<bool, OFFRAM3_A>);
impl OFFRAM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFRAM3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM3_A {
        match self.bits {
            false => OFFRAM3_A::RAM3OFF,
            true => OFFRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        **self == OFFRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        **self == OFFRAM3_A::RAM3ON
    }
}
impl core::ops::Deref for OFFRAM3_R {
    type Target = crate::FieldReader<bool, OFFRAM3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFRAM3` writer - Keep retention on RAM block 3 when RAM block is switched off"]
pub struct OFFRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM block 2 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram2(&self) -> ONRAM2_R {
        ONRAM2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Keep RAM block 3 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram3(&self) -> ONRAM3_R {
        ONRAM3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 2 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram2(&self) -> OFFRAM2_R {
        OFFRAM2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 3 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram3(&self) -> OFFRAM3_R {
        OFFRAM3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM block 2 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram2(&mut self) -> ONRAM2_W {
        ONRAM2_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM block 3 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram3(&mut self) -> ONRAM3_W {
        ONRAM3_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM block 2 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram2(&mut self) -> OFFRAM2_W {
        OFFRAM2_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM block 3 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram3(&mut self) -> OFFRAM3_W {
        OFFRAM3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deprecated register - RAM on/off register (this register is retained)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramonb](index.html) module"]
pub struct RAMONB_SPEC;
impl crate::RegisterSpec for RAMONB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramonb::R](R) reader structure"]
impl crate::Readable for RAMONB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramonb::W](W) writer structure"]
impl crate::Writable for RAMONB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMONB to value 0x03"]
impl crate::Resettable for RAMONB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
