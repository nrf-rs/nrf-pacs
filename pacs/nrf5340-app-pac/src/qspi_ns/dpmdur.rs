#[doc = "Register `DPMDUR` reader"]
pub struct R(crate::R<DPMDUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPMDUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPMDUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPMDUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPMDUR` writer"]
pub struct W(crate::W<DPMDUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPMDUR_SPEC>;
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
impl From<crate::W<DPMDUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPMDUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENTER` reader - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
pub type ENTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENTER` writer - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
pub type ENTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPMDUR_SPEC, u16, u16, 16, O>;
#[doc = "Field `EXIT` reader - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
pub type EXIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXIT` writer - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
pub type EXIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPMDUR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&self) -> ENTER_R {
        ENTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&self) -> EXIT_R {
        EXIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&mut self) -> ENTER_W<0> {
        ENTER_W::new(self)
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&mut self) -> EXIT_W<16> {
        EXIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpmdur](index.html) module"]
pub struct DPMDUR_SPEC;
impl crate::RegisterSpec for DPMDUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpmdur::R](R) reader structure"]
impl crate::Readable for DPMDUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpmdur::W](W) writer structure"]
impl crate::Writable for DPMDUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPMDUR to value 0xffff_ffff"]
impl crate::Resettable for DPMDUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
