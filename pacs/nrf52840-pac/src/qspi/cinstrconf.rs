#[doc = "Register `CINSTRCONF` reader"]
pub type R = crate::R<CinstrconfSpec>;
#[doc = "Register `CINSTRCONF` writer"]
pub type W = crate::W<CinstrconfSpec>;
#[doc = "Field `OPCODE` reader - Opcode of Custom instruction."]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode of Custom instruction."]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Length of custom instruction in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Length {
    #[doc = "1: Send opcode only."]
    _1b = 1,
    #[doc = "2: Send opcode, CINSTRDAT0.BYTE0."]
    _2b = 2,
    #[doc = "3: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    _3b = 3,
    #[doc = "4: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    _4b = 4,
    #[doc = "5: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    _5b = 5,
    #[doc = "6: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    _6b = 6,
    #[doc = "7: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    _7b = 7,
    #[doc = "8: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    _8b = 8,
    #[doc = "9: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    _9b = 9,
}
impl From<Length> for u8 {
    #[inline(always)]
    fn from(variant: Length) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Length {
    type Ux = u8;
}
impl crate::IsEnum for Length {}
#[doc = "Field `LENGTH` reader - Length of custom instruction in number of bytes."]
pub type LengthR = crate::FieldReader<Length>;
impl LengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Length> {
        match self.bits {
            1 => Some(Length::_1b),
            2 => Some(Length::_2b),
            3 => Some(Length::_3b),
            4 => Some(Length::_4b),
            5 => Some(Length::_5b),
            6 => Some(Length::_6b),
            7 => Some(Length::_7b),
            8 => Some(Length::_8b),
            9 => Some(Length::_9b),
            _ => None,
        }
    }
    #[doc = "Send opcode only."]
    #[inline(always)]
    pub fn is_1b(&self) -> bool {
        *self == Length::_1b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    #[inline(always)]
    pub fn is_2b(&self) -> bool {
        *self == Length::_2b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    #[inline(always)]
    pub fn is_3b(&self) -> bool {
        *self == Length::_3b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == Length::_4b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    #[inline(always)]
    pub fn is_5b(&self) -> bool {
        *self == Length::_5b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    #[inline(always)]
    pub fn is_6b(&self) -> bool {
        *self == Length::_6b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    #[inline(always)]
    pub fn is_7b(&self) -> bool {
        *self == Length::_7b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Length::_8b
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    #[inline(always)]
    pub fn is_9b(&self) -> bool {
        *self == Length::_9b
    }
}
#[doc = "Field `LENGTH` writer - Length of custom instruction in number of bytes."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 4, Length>;
impl<'a, REG> LengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Send opcode only."]
    #[inline(always)]
    pub fn _1b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_1b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    #[inline(always)]
    pub fn _2b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_2b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    #[inline(always)]
    pub fn _3b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_3b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_4b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_5b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    #[inline(always)]
    pub fn _6b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_6b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    #[inline(always)]
    pub fn _7b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_7b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_8b)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    #[inline(always)]
    pub fn _9b(self) -> &'a mut crate::W<REG> {
        self.variant(Length::_9b)
    }
}
#[doc = "Field `LIO2` reader - Level of the IO2 pin (if connected) during transmission of custom instruction."]
pub type Lio2R = crate::BitReader;
#[doc = "Field `LIO2` writer - Level of the IO2 pin (if connected) during transmission of custom instruction."]
pub type Lio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIO3` reader - Level of the IO3 pin (if connected) during transmission of custom instruction."]
pub type Lio3R = crate::BitReader;
#[doc = "Field `LIO3` writer - Level of the IO3 pin (if connected) during transmission of custom instruction."]
pub type Lio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Wait for write complete before sending command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wipwait {
    #[doc = "0: No wait."]
    Disable = 0,
    #[doc = "1: Wait."]
    Enable = 1,
}
impl From<Wipwait> for bool {
    #[inline(always)]
    fn from(variant: Wipwait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIPWAIT` reader - Wait for write complete before sending command."]
pub type WipwaitR = crate::BitReader<Wipwait>;
impl WipwaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wipwait {
        match self.bits {
            false => Wipwait::Disable,
            true => Wipwait::Enable,
        }
    }
    #[doc = "No wait."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wipwait::Disable
    }
    #[doc = "Wait."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wipwait::Enable
    }
}
#[doc = "Field `WIPWAIT` writer - Wait for write complete before sending command."]
pub type WipwaitW<'a, REG> = crate::BitWriter<'a, REG, Wipwait>;
impl<'a, REG> WipwaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wait."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wipwait::Disable)
    }
    #[doc = "Wait."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wipwait::Enable)
    }
}
#[doc = "Send WREN (write enable opcode 0x06) before instruction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wren {
    #[doc = "0: Do not send WREN."]
    Disable = 0,
    #[doc = "1: Send WREN."]
    Enable = 1,
}
impl From<Wren> for bool {
    #[inline(always)]
    fn from(variant: Wren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WREN` reader - Send WREN (write enable opcode 0x06) before instruction."]
pub type WrenR = crate::BitReader<Wren>;
impl WrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wren {
        match self.bits {
            false => Wren::Disable,
            true => Wren::Enable,
        }
    }
    #[doc = "Do not send WREN."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wren::Disable
    }
    #[doc = "Send WREN."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wren::Enable
    }
}
#[doc = "Field `WREN` writer - Send WREN (write enable opcode 0x06) before instruction."]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG, Wren>;
impl<'a, REG> WrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not send WREN."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wren::Disable)
    }
    #[doc = "Send WREN."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wren::Enable)
    }
}
#[doc = "Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfen {
    #[doc = "0: Long frame mode disabled"]
    Disable = 0,
    #[doc = "1: Long frame mode enabled"]
    Enable = 1,
}
impl From<Lfen> for bool {
    #[inline(always)]
    fn from(variant: Lfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFEN` reader - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
pub type LfenR = crate::BitReader<Lfen>;
impl LfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfen {
        match self.bits {
            false => Lfen::Disable,
            true => Lfen::Enable,
        }
    }
    #[doc = "Long frame mode disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lfen::Disable
    }
    #[doc = "Long frame mode enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lfen::Enable
    }
}
#[doc = "Field `LFEN` writer - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
pub type LfenW<'a, REG> = crate::BitWriter<'a, REG, Lfen>;
impl<'a, REG> LfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Long frame mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfen::Disable)
    }
    #[doc = "Long frame mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lfen::Enable)
    }
}
#[doc = "Stop (finalize) long frame transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfstop {
    #[doc = "1: Stop"]
    Stop = 1,
}
impl From<Lfstop> for bool {
    #[inline(always)]
    fn from(variant: Lfstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSTOP` reader - Stop (finalize) long frame transaction"]
pub type LfstopR = crate::BitReader<Lfstop>;
impl LfstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfstop> {
        match self.bits {
            true => Some(Lfstop::Stop),
            _ => None,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Lfstop::Stop
    }
}
#[doc = "Field `LFSTOP` writer - Stop (finalize) long frame transaction"]
pub type LfstopW<'a, REG> = crate::BitWriter<'a, REG, Lfstop>;
impl<'a, REG> LfstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Lfstop::Stop)
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio2(&self) -> Lio2R {
        Lio2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio3(&self) -> Lio3R {
        Lio3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WipwaitR {
        WipwaitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn lfen(&self) -> LfenR {
        LfenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn lfstop(&self) -> LfstopR {
        LfstopR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<'_, CinstrconfSpec> {
        OpcodeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn length(&mut self) -> LengthW<'_, CinstrconfSpec> {
        LengthW::new(self, 8)
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio2(&mut self) -> Lio2W<'_, CinstrconfSpec> {
        Lio2W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio3(&mut self) -> Lio3W<'_, CinstrconfSpec> {
        Lio3W::new(self, 13)
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WipwaitW<'_, CinstrconfSpec> {
        WipwaitW::new(self, 14)
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WrenW<'_, CinstrconfSpec> {
        WrenW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn lfen(&mut self) -> LfenW<'_, CinstrconfSpec> {
        LfenW::new(self, 16)
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn lfstop(&mut self) -> LfstopW<'_, CinstrconfSpec> {
        LfstopW::new(self, 17)
    }
}
#[doc = "Custom instruction configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CinstrconfSpec;
impl crate::RegisterSpec for CinstrconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cinstrconf::R`](R) reader structure"]
impl crate::Readable for CinstrconfSpec {}
#[doc = "`write(|w| ..)` method takes [`cinstrconf::W`](W) writer structure"]
impl crate::Writable for CinstrconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CINSTRCONF to value 0x2000"]
impl crate::Resettable for CinstrconfSpec {
    const RESET_VALUE: u32 = 0x2000;
}
