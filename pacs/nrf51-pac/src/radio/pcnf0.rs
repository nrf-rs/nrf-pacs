#[doc = "Register `PCNF0` reader"]
pub struct R(crate::R<PCNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF0` writer"]
pub struct W(crate::W<PCNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF0_SPEC>;
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
impl From<crate::W<PCNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFLEN` reader - Length of length field in number of bits. Decision point: START task."]
pub type LFLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFLEN` writer - Length of length field in number of bits. Decision point: START task."]
pub type LFLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF0_SPEC, u8, u8, 4, O>;
#[doc = "Field `S0LEN` reader - Length of S0 field in number of bytes. Decision point: START task."]
pub type S0LEN_R = crate::BitReader<bool>;
#[doc = "Field `S0LEN` writer - Length of S0 field in number of bytes. Decision point: START task."]
pub type S0LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF0_SPEC, bool, O>;
#[doc = "Field `S1LEN` reader - Length of S1 field in number of bits. Decision point: START task."]
pub type S1LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S1LEN` writer - Length of S1 field in number of bits. Decision point: START task."]
pub type S1LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LFLEN_W<0> {
        LFLEN_W::new(self)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0LEN_W<8> {
        S0LEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1LEN_W<16> {
        S1LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf0](index.html) module"]
pub struct PCNF0_SPEC;
impl crate::RegisterSpec for PCNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf0::R](R) reader structure"]
impl crate::Readable for PCNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf0::W](W) writer structure"]
impl crate::Writable for PCNF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for PCNF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
