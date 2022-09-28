#[doc = "Register `DCXCNT` reader"]
pub struct R(crate::R<DCXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCXCNT` writer"]
pub struct W(crate::W<DCXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCXCNT_SPEC>;
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
impl From<crate::W<DCXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCXCNT` reader - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
pub type DCXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCXCNT` writer - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
pub type DCXCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCXCNT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&self) -> DCXCNT_R {
        DCXCNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&mut self) -> DCXCNT_W<0> {
        DCXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCX configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcxcnt](index.html) module"]
pub struct DCXCNT_SPEC;
impl crate::RegisterSpec for DCXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcxcnt::R](R) reader structure"]
impl crate::Readable for DCXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcxcnt::W](W) writer structure"]
impl crate::Writable for DCXCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCXCNT to value 0"]
impl crate::Resettable for DCXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
