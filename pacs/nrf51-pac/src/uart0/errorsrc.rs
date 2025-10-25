#[doc = "Register `ERRORSRC` reader"]
pub type R = crate::R<ErrorsrcSpec>;
#[doc = "Register `ERRORSRC` writer"]
pub type W = crate::W<ErrorsrcSpec>;
#[doc = "A start bit is received while the previous data still lies in RXD. (Data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overrun {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Overrun> for bool {
    #[inline(always)]
    fn from(variant: Overrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - A start bit is received while the previous data still lies in RXD. (Data loss)."]
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
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Overrun::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Overrun::Present
    }
}
#[doc = "A start bit is received while the previous data still lies in RXD. (Data loss).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverrunWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<OverrunWO> for bool {
    #[inline(always)]
    fn from(variant: OverrunWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` writer - A start bit is received while the previous data still lies in RXD. (Data loss)."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG, OverrunWO>;
impl<'a, REG> OverrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverrunWO::Clear)
    }
}
#[doc = "A character with bad parity is received. Only checked if HW parity control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Parity> for bool {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - A character with bad parity is received. Only checked if HW parity control is enabled."]
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
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Parity::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Parity::Present
    }
}
#[doc = "A character with bad parity is received. Only checked if HW parity control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParityWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<ParityWO> for bool {
    #[inline(always)]
    fn from(variant: ParityWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` writer - A character with bad parity is received. Only checked if HW parity control is enabled."]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG, ParityWO>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ParityWO::Clear)
    }
}
#[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framing {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Framing> for bool {
    #[inline(always)]
    fn from(variant: Framing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMING` reader - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
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
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Framing::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Framing::Present
    }
}
#[doc = "A valid stop bit is not detected on the serial data input after all bits in a character have been received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FramingWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<FramingWO> for bool {
    #[inline(always)]
    fn from(variant: FramingWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMING` writer - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
pub type FramingW<'a, REG> = crate::BitWriter<'a, REG, FramingWO>;
impl<'a, REG> FramingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FramingWO::Clear)
    }
}
#[doc = "The serial data input is '0' for longer than the length of a data frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Break {
    #[doc = "0: Error not present."]
    NotPresent = 0,
    #[doc = "1: Error present."]
    Present = 1,
}
impl From<Break> for bool {
    #[inline(always)]
    fn from(variant: Break) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREAK` reader - The serial data input is '0' for longer than the length of a data frame."]
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
    #[doc = "Error not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Break::NotPresent
    }
    #[doc = "Error present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Break::Present
    }
}
#[doc = "The serial data input is '0' for longer than the length of a data frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakWO {
    #[doc = "1: Clear error on write."]
    Clear = 1,
}
impl From<BreakWO> for bool {
    #[inline(always)]
    fn from(variant: BreakWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREAK` writer - The serial data input is '0' for longer than the length of a data frame."]
pub type BreakW<'a, REG> = crate::BitWriter<'a, REG, BreakWO>;
impl<'a, REG> BreakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BreakWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline(always)]
    pub fn framing(&self) -> FramingR {
        FramingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The serial data input is '0' for longer than the length of a data frame."]
    #[inline(always)]
    pub fn break_(&self) -> BreakR {
        BreakR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, ErrorsrcSpec> {
        OverrunW::new(self, 0)
    }
    #[doc = "Bit 1 - A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, ErrorsrcSpec> {
        ParityW::new(self, 1)
    }
    #[doc = "Bit 2 - A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline(always)]
    pub fn framing(&mut self) -> FramingW<'_, ErrorsrcSpec> {
        FramingW::new(self, 2)
    }
    #[doc = "Bit 3 - The serial data input is '0' for longer than the length of a data frame."]
    #[inline(always)]
    pub fn break_(&mut self) -> BreakW<'_, ErrorsrcSpec> {
        BreakW::new(self, 3)
    }
}
#[doc = "Error source. Write error field to 1 to clear error.\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorsrcSpec;
impl crate::RegisterSpec for ErrorsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorsrc::R`](R) reader structure"]
impl crate::Readable for ErrorsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorsrc::W`](W) writer structure"]
impl crate::Writable for ErrorsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ErrorsrcSpec {}
