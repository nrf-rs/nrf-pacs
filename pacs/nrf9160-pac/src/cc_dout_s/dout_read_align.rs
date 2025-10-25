#[doc = "Register `DOUT_READ_ALIGN` writer"]
pub type W = crate::W<DoutReadAlignSpec>;
#[doc = "Next CPU read from the DOUT_BUFFER is the last word, and the remaining read aligned content can be flushed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Last {
    #[doc = "1: Flush the remaining read aligned content."]
    Flush = 1,
}
impl From<Last> for bool {
    #[inline(always)]
    fn from(variant: Last) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST` writer - Next CPU read from the DOUT_BUFFER is the last word, and the remaining read aligned content can be flushed."]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG, Last>;
impl<'a, REG> LastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush the remaining read aligned content."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Last::Flush)
    }
}
impl W {
    #[doc = "Bit 0 - Next CPU read from the DOUT_BUFFER is the last word, and the remaining read aligned content can be flushed."]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<'_, DoutReadAlignSpec> {
        LastW::new(self, 0)
    }
}
#[doc = "Indication that the next CPU read from the DOUT_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_read_align::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutReadAlignSpec;
impl crate::RegisterSpec for DoutReadAlignSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout_read_align::W`](W) writer structure"]
impl crate::Writable for DoutReadAlignSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOUT_READ_ALIGN to value 0"]
impl crate::Resettable for DoutReadAlignSpec {}
