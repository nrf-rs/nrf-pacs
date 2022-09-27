#[doc = "Register `CTIINTACK` writer"]
pub struct W(crate::W<CTIINTACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIINTACK_SPEC>;
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
impl From<crate::W<CTIINTACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIINTACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGREQ_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<DEBUGREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: DEBUGREQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` writer - Processor debug request"]
pub type DEBUGREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, DEBUGREQ_AW, O>;
impl<'a, const O: u8> DEBUGREQ_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(DEBUGREQ_AW::ACKNOWLEDGE)
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURESTART_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<CPURESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: CPURESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` writer - Processor Restart"]
pub type CPURESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTIINTACK_SPEC, CPURESTART_AW, O>;
impl<'a, const O: u8> CPURESTART_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(CPURESTART_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED0_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` writer - N/A"]
pub type UNUSED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED0_AW, O>;
impl<'a, const O: u8> UNUSED0_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED0_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED1_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED1_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` writer - N/A"]
pub type UNUSED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED1_AW, O>;
impl<'a, const O: u8> UNUSED1_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED1_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED2_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED2_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED2` writer - N/A"]
pub type UNUSED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED2_AW, O>;
impl<'a, const O: u8> UNUSED2_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED2_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED3_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED3_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED3` writer - N/A"]
pub type UNUSED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED3_AW, O>;
impl<'a, const O: u8> UNUSED3_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED3_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED4_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED4_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED4` writer - N/A"]
pub type UNUSED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED4_AW, O>;
impl<'a, const O: u8> UNUSED4_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED4_AW::ACKNOWLEDGE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED5_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED5_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED5` writer - N/A"]
pub type UNUSED5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTIINTACK_SPEC, UNUSED5_AW, O>;
impl<'a, const O: u8> UNUSED5_W<'a, O> {
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED5_AW::ACKNOWLEDGE)
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&mut self) -> DEBUGREQ_W<0> {
        DEBUGREQ_W::new(self)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&mut self) -> CPURESTART_W<1> {
        CPURESTART_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&mut self) -> UNUSED0_W<2> {
        UNUSED0_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&mut self) -> UNUSED1_W<3> {
        UNUSED1_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused2(&mut self) -> UNUSED2_W<4> {
        UNUSED2_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused3(&mut self) -> UNUSED3_W<5> {
        UNUSED3_W::new(self)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused4(&mut self) -> UNUSED4_W<6> {
        UNUSED4_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused5(&mut self) -> UNUSED5_W<7> {
        UNUSED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Interrupt Acknowledge register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiintack](index.html) module"]
pub struct CTIINTACK_SPEC;
impl crate::RegisterSpec for CTIINTACK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctiintack::W](W) writer structure"]
impl crate::Writable for CTIINTACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIINTACK to value 0"]
impl crate::Resettable for CTIINTACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
