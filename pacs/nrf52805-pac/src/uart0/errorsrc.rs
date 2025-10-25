#[doc = "Register `ERRORSRC` reader"]
pub type R = crate::R<ErrorsrcSpec>;
#[doc = "Register `ERRORSRC` writer"]
pub type W = crate::W<ErrorsrcSpec>;
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overrun {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Overrun> for bool {
    #[inline(always)]
    fn from(variant: Overrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Overrun error"]
pub type OverrunR = crate::BitReader<Overrun>;
impl OverrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overrun {
        match self.bits {
            false => Overrun::NotPresent,
            true => Overrun::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Overrun::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Overrun::Present
    }
}
#[doc = "Field `OVERRUN` writer - Overrun error"]
pub type OverrunW<'a, REG> = crate::BitWriter1C<'a, REG, Overrun>;
impl<'a, REG> OverrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::NotPresent)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::Present)
    }
}
#[doc = "Parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Parity> for bool {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - Parity error"]
pub type ParityR = crate::BitReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parity {
        match self.bits {
            false => Parity::NotPresent,
            true => Parity::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Parity::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Parity::Present
    }
}
#[doc = "Field `PARITY` writer - Parity error"]
pub type ParityW<'a, REG> = crate::BitWriter1C<'a, REG, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::NotPresent)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Present)
    }
}
#[doc = "Framing error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framing {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Framing> for bool {
    #[inline(always)]
    fn from(variant: Framing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMING` reader - Framing error occurred"]
pub type FramingR = crate::BitReader<Framing>;
impl FramingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Framing {
        match self.bits {
            false => Framing::NotPresent,
            true => Framing::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Framing::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Framing::Present
    }
}
#[doc = "Field `FRAMING` writer - Framing error occurred"]
pub type FramingW<'a, REG> = crate::BitWriter1C<'a, REG, Framing>;
impl<'a, REG> FramingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Framing::NotPresent)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Framing::Present)
    }
}
#[doc = "Break condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Break {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Break> for bool {
    #[inline(always)]
    fn from(variant: Break) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREAK` reader - Break condition"]
pub type BreakR = crate::BitReader<Break>;
impl BreakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Break {
        match self.bits {
            false => Break::NotPresent,
            true => Break::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Break::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Break::Present
    }
}
#[doc = "Field `BREAK` writer - Break condition"]
pub type BreakW<'a, REG> = crate::BitWriter1C<'a, REG, Break>;
impl<'a, REG> BreakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Break::NotPresent)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Break::Present)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    pub fn framing(&self) -> FramingR {
        FramingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    pub fn break_(&self) -> BreakR {
        BreakR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, ErrorsrcSpec> {
        OverrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, ErrorsrcSpec> {
        ParityW::new(self, 1)
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    pub fn framing(&mut self) -> FramingW<'_, ErrorsrcSpec> {
        FramingW::new(self, 2)
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    pub fn break_(&mut self) -> BreakW<'_, ErrorsrcSpec> {
        BreakW::new(self, 3)
    }
}
#[doc = "Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorsrcSpec;
impl crate::RegisterSpec for ErrorsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorsrc::R`](R) reader structure"]
impl crate::Readable for ErrorsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorsrc::W`](W) writer structure"]
impl crate::Writable for ErrorsrcSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ErrorsrcSpec {}
