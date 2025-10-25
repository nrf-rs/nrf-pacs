#[doc = "Register `ADDRCONF` reader"]
pub type R = crate::R<AddrconfSpec>;
#[doc = "Register `ADDRCONF` writer"]
pub type W = crate::W<AddrconfSpec>;
#[doc = "Field `OPCODE` reader - Opcode that enters the 32-bit addressing mode."]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode that enters the 32-bit addressing mode."]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE0` reader - Byte 0 following opcode."]
pub type Byte0R = crate::FieldReader;
#[doc = "Field `BYTE0` writer - Byte 0 following opcode."]
pub type Byte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Byte 1 following byte 0."]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Byte 1 following byte 0."]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Extended addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Do not send any instruction."]
    NoInstr = 0,
    #[doc = "1: Send opcode."]
    Opcode = 1,
    #[doc = "2: Send opcode, byte0."]
    OpByte0 = 2,
    #[doc = "3: Send opcode, byte0, byte1."]
    All = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Extended addressing mode."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::NoInstr,
            1 => Mode::Opcode,
            2 => Mode::OpByte0,
            3 => Mode::All,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not send any instruction."]
    #[inline(always)]
    pub fn is_no_instr(&self) -> bool {
        *self == Mode::NoInstr
    }
    #[doc = "Send opcode."]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == Mode::Opcode
    }
    #[doc = "Send opcode, byte0."]
    #[inline(always)]
    pub fn is_op_byte0(&self) -> bool {
        *self == Mode::OpByte0
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Mode::All
    }
}
#[doc = "Field `MODE` writer - Extended addressing mode."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not send any instruction."]
    #[inline(always)]
    pub fn no_instr(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::NoInstr)
    }
    #[doc = "Send opcode."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Opcode)
    }
    #[doc = "Send opcode, byte0."]
    #[inline(always)]
    pub fn op_byte0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::OpByte0)
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::All)
    }
}
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
impl R {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        Byte0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WipwaitR {
        WipwaitR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<'_, AddrconfSpec> {
        OpcodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&mut self) -> Byte0W<'_, AddrconfSpec> {
        Byte0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<'_, AddrconfSpec> {
        Byte1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, AddrconfSpec> {
        ModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WipwaitW<'_, AddrconfSpec> {
        WipwaitW::new(self, 26)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WrenW<'_, AddrconfSpec> {
        WrenW::new(self, 27)
    }
}
#[doc = "Extended address configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`addrconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrconfSpec;
impl crate::RegisterSpec for AddrconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrconf::R`](R) reader structure"]
impl crate::Readable for AddrconfSpec {}
#[doc = "`write(|w| ..)` method takes [`addrconf::W`](W) writer structure"]
impl crate::Writable for AddrconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRCONF to value 0xb7"]
impl crate::Resettable for AddrconfSpec {
    const RESET_VALUE: u32 = 0xb7;
}
