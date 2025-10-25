#[doc = "Register `CHACHA_ENDIANNESS` reader"]
pub type R = crate::R<ChachaEndiannessSpec>;
#[doc = "Register `CHACHA_ENDIANNESS` writer"]
pub type W = crate::W<ChachaEndiannessSpec>;
#[doc = "Change the word order of the input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaDinWordOrder {
    #[doc = "0: Use default word order for 128-bits input, where words are ordered as follows: w0, w1, w2, w3."]
    Default = 0,
    #[doc = "1: Reverses the word order for 128-bits input, where words are re-ordered as follows: w3, w2, w1, w0."]
    Reverse = 1,
}
impl From<ChachaDinWordOrder> for bool {
    #[inline(always)]
    fn from(variant: ChachaDinWordOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_DIN_WORD_ORDER` reader - Change the word order of the input data."]
pub type ChachaDinWordOrderR = crate::BitReader<ChachaDinWordOrder>;
impl ChachaDinWordOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaDinWordOrder {
        match self.bits {
            false => ChachaDinWordOrder::Default,
            true => ChachaDinWordOrder::Reverse,
        }
    }
    #[doc = "Use default word order for 128-bits input, where words are ordered as follows: w0, w1, w2, w3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == ChachaDinWordOrder::Default
    }
    #[doc = "Reverses the word order for 128-bits input, where words are re-ordered as follows: w3, w2, w1, w0."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == ChachaDinWordOrder::Reverse
    }
}
#[doc = "Field `CHACHA_DIN_WORD_ORDER` writer - Change the word order of the input data."]
pub type ChachaDinWordOrderW<'a, REG> = crate::BitWriter<'a, REG, ChachaDinWordOrder>;
impl<'a, REG> ChachaDinWordOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use default word order for 128-bits input, where words are ordered as follows: w0, w1, w2, w3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDinWordOrder::Default)
    }
    #[doc = "Reverses the word order for 128-bits input, where words are re-ordered as follows: w3, w2, w1, w0."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDinWordOrder::Reverse)
    }
}
#[doc = "Change the byte order of the input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaDinByteOrder {
    #[doc = "0: Use default byte order within each input word, where bytes are ordered as follows: B0, B1, B2, B3."]
    Default = 0,
    #[doc = "1: Reverse the byte order within each input word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    Reverse = 1,
}
impl From<ChachaDinByteOrder> for bool {
    #[inline(always)]
    fn from(variant: ChachaDinByteOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_DIN_BYTE_ORDER` reader - Change the byte order of the input data."]
pub type ChachaDinByteOrderR = crate::BitReader<ChachaDinByteOrder>;
impl ChachaDinByteOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaDinByteOrder {
        match self.bits {
            false => ChachaDinByteOrder::Default,
            true => ChachaDinByteOrder::Reverse,
        }
    }
    #[doc = "Use default byte order within each input word, where bytes are ordered as follows: B0, B1, B2, B3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == ChachaDinByteOrder::Default
    }
    #[doc = "Reverse the byte order within each input word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == ChachaDinByteOrder::Reverse
    }
}
#[doc = "Field `CHACHA_DIN_BYTE_ORDER` writer - Change the byte order of the input data."]
pub type ChachaDinByteOrderW<'a, REG> = crate::BitWriter<'a, REG, ChachaDinByteOrder>;
impl<'a, REG> ChachaDinByteOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use default byte order within each input word, where bytes are ordered as follows: B0, B1, B2, B3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDinByteOrder::Default)
    }
    #[doc = "Reverse the byte order within each input word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDinByteOrder::Reverse)
    }
}
#[doc = "Change the quarter of a matrix order in the engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaCoreMatrixLbeOrder {
    #[doc = "0: Use default quarter of matrix order, where quarters are ordered as follows: q0, q1, q2, q3. Each quarter represents a 128-bits section of the matrix."]
    Default = 0,
    #[doc = "1: Reverse the order of matrix quarters, where quarters are re-ordered as follows: q3, q2, q1, q0. Each quarter represents a 128-bits section of the matrix."]
    Reverse = 1,
}
impl From<ChachaCoreMatrixLbeOrder> for bool {
    #[inline(always)]
    fn from(variant: ChachaCoreMatrixLbeOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_CORE_MATRIX_LBE_ORDER` reader - Change the quarter of a matrix order in the engine."]
pub type ChachaCoreMatrixLbeOrderR = crate::BitReader<ChachaCoreMatrixLbeOrder>;
impl ChachaCoreMatrixLbeOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaCoreMatrixLbeOrder {
        match self.bits {
            false => ChachaCoreMatrixLbeOrder::Default,
            true => ChachaCoreMatrixLbeOrder::Reverse,
        }
    }
    #[doc = "Use default quarter of matrix order, where quarters are ordered as follows: q0, q1, q2, q3. Each quarter represents a 128-bits section of the matrix."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == ChachaCoreMatrixLbeOrder::Default
    }
    #[doc = "Reverse the order of matrix quarters, where quarters are re-ordered as follows: q3, q2, q1, q0. Each quarter represents a 128-bits section of the matrix."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == ChachaCoreMatrixLbeOrder::Reverse
    }
}
#[doc = "Field `CHACHA_CORE_MATRIX_LBE_ORDER` writer - Change the quarter of a matrix order in the engine."]
pub type ChachaCoreMatrixLbeOrderW<'a, REG> = crate::BitWriter<'a, REG, ChachaCoreMatrixLbeOrder>;
impl<'a, REG> ChachaCoreMatrixLbeOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use default quarter of matrix order, where quarters are ordered as follows: q0, q1, q2, q3. Each quarter represents a 128-bits section of the matrix."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaCoreMatrixLbeOrder::Default)
    }
    #[doc = "Reverse the order of matrix quarters, where quarters are re-ordered as follows: q3, q2, q1, q0. Each quarter represents a 128-bits section of the matrix."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaCoreMatrixLbeOrder::Reverse)
    }
}
#[doc = "Change the word order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaDoutWordOrder {
    #[doc = "0: Uses default word order for 128-bits output, where words are ordered as follows: w0, w1, w2, w3."]
    Default = 0,
    #[doc = "1: Reverse the word order for 128-bits output, where words are re-ordered as follows: w3, w2, w1, w0."]
    Reverse = 1,
}
impl From<ChachaDoutWordOrder> for bool {
    #[inline(always)]
    fn from(variant: ChachaDoutWordOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_DOUT_WORD_ORDER` reader - Change the word order of the output data."]
pub type ChachaDoutWordOrderR = crate::BitReader<ChachaDoutWordOrder>;
impl ChachaDoutWordOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaDoutWordOrder {
        match self.bits {
            false => ChachaDoutWordOrder::Default,
            true => ChachaDoutWordOrder::Reverse,
        }
    }
    #[doc = "Uses default word order for 128-bits output, where words are ordered as follows: w0, w1, w2, w3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == ChachaDoutWordOrder::Default
    }
    #[doc = "Reverse the word order for 128-bits output, where words are re-ordered as follows: w3, w2, w1, w0."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == ChachaDoutWordOrder::Reverse
    }
}
#[doc = "Field `CHACHA_DOUT_WORD_ORDER` writer - Change the word order of the output data."]
pub type ChachaDoutWordOrderW<'a, REG> = crate::BitWriter<'a, REG, ChachaDoutWordOrder>;
impl<'a, REG> ChachaDoutWordOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Uses default word order for 128-bits output, where words are ordered as follows: w0, w1, w2, w3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDoutWordOrder::Default)
    }
    #[doc = "Reverse the word order for 128-bits output, where words are re-ordered as follows: w3, w2, w1, w0."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDoutWordOrder::Reverse)
    }
}
#[doc = "Change the byte order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaDoutByteOrder {
    #[doc = "0: Use default byte order within each output word, where bytes are ordered as follows: B0, B1, B2, B3."]
    Default = 0,
    #[doc = "1: Reverse the byte order within each output word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    Reverse = 1,
}
impl From<ChachaDoutByteOrder> for bool {
    #[inline(always)]
    fn from(variant: ChachaDoutByteOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_DOUT_BYTE_ORDER` reader - Change the byte order of the output data."]
pub type ChachaDoutByteOrderR = crate::BitReader<ChachaDoutByteOrder>;
impl ChachaDoutByteOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaDoutByteOrder {
        match self.bits {
            false => ChachaDoutByteOrder::Default,
            true => ChachaDoutByteOrder::Reverse,
        }
    }
    #[doc = "Use default byte order within each output word, where bytes are ordered as follows: B0, B1, B2, B3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == ChachaDoutByteOrder::Default
    }
    #[doc = "Reverse the byte order within each output word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == ChachaDoutByteOrder::Reverse
    }
}
#[doc = "Field `CHACHA_DOUT_BYTE_ORDER` writer - Change the byte order of the output data."]
pub type ChachaDoutByteOrderW<'a, REG> = crate::BitWriter<'a, REG, ChachaDoutByteOrder>;
impl<'a, REG> ChachaDoutByteOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use default byte order within each output word, where bytes are ordered as follows: B0, B1, B2, B3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDoutByteOrder::Default)
    }
    #[doc = "Reverse the byte order within each output word, where bytes are re-ordered as follows: B3, B2, B1, B0."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaDoutByteOrder::Reverse)
    }
}
impl R {
    #[doc = "Bit 0 - Change the word order of the input data."]
    #[inline(always)]
    pub fn chacha_din_word_order(&self) -> ChachaDinWordOrderR {
        ChachaDinWordOrderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Change the byte order of the input data."]
    #[inline(always)]
    pub fn chacha_din_byte_order(&self) -> ChachaDinByteOrderR {
        ChachaDinByteOrderR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change the quarter of a matrix order in the engine."]
    #[inline(always)]
    pub fn chacha_core_matrix_lbe_order(&self) -> ChachaCoreMatrixLbeOrderR {
        ChachaCoreMatrixLbeOrderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Change the word order of the output data."]
    #[inline(always)]
    pub fn chacha_dout_word_order(&self) -> ChachaDoutWordOrderR {
        ChachaDoutWordOrderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Change the byte order of the output data."]
    #[inline(always)]
    pub fn chacha_dout_byte_order(&self) -> ChachaDoutByteOrderR {
        ChachaDoutByteOrderR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Change the word order of the input data."]
    #[inline(always)]
    pub fn chacha_din_word_order(&mut self) -> ChachaDinWordOrderW<'_, ChachaEndiannessSpec> {
        ChachaDinWordOrderW::new(self, 0)
    }
    #[doc = "Bit 1 - Change the byte order of the input data."]
    #[inline(always)]
    pub fn chacha_din_byte_order(&mut self) -> ChachaDinByteOrderW<'_, ChachaEndiannessSpec> {
        ChachaDinByteOrderW::new(self, 1)
    }
    #[doc = "Bit 2 - Change the quarter of a matrix order in the engine."]
    #[inline(always)]
    pub fn chacha_core_matrix_lbe_order(
        &mut self,
    ) -> ChachaCoreMatrixLbeOrderW<'_, ChachaEndiannessSpec> {
        ChachaCoreMatrixLbeOrderW::new(self, 2)
    }
    #[doc = "Bit 3 - Change the word order of the output data."]
    #[inline(always)]
    pub fn chacha_dout_word_order(&mut self) -> ChachaDoutWordOrderW<'_, ChachaEndiannessSpec> {
        ChachaDoutWordOrderW::new(self, 3)
    }
    #[doc = "Bit 4 - Change the byte order of the output data."]
    #[inline(always)]
    pub fn chacha_dout_byte_order(&mut self) -> ChachaDoutByteOrderW<'_, ChachaEndiannessSpec> {
        ChachaDoutByteOrderW::new(self, 4)
    }
}
#[doc = "CHACHA engine data order configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_endianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_endianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaEndiannessSpec;
impl crate::RegisterSpec for ChachaEndiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_endianness::R`](R) reader structure"]
impl crate::Readable for ChachaEndiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`chacha_endianness::W`](W) writer structure"]
impl crate::Writable for ChachaEndiannessSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHACHA_ENDIANNESS to value 0"]
impl crate::Resettable for ChachaEndiannessSpec {}
