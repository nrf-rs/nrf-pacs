#[doc = "Register `DIRCLR` reader"]
pub type R = crate::R<DirclrSpec>;
#[doc = "Register `DIRCLR` writer"]
pub type W = crate::W<DirclrSpec>;
#[doc = "Set as input pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Set as input pin 0"]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            false => Pin0::Input,
            true => Pin0::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin0::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin0::Output
    }
}
#[doc = "Set as input pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin0WO> for bool {
    #[inline(always)]
    fn from(variant: Pin0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` writer - Set as input pin 0"]
pub type Pin0W<'a, REG> = crate::BitWriter1C<'a, REG, Pin0WO>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0WO::Clear)
    }
}
#[doc = "Set as input pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Set as input pin 1"]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            false => Pin1::Input,
            true => Pin1::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin1::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin1::Output
    }
}
#[doc = "Set as input pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin1WO> for bool {
    #[inline(always)]
    fn from(variant: Pin1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` writer - Set as input pin 1"]
pub type Pin1W<'a, REG> = crate::BitWriter1C<'a, REG, Pin1WO>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1WO::Clear)
    }
}
#[doc = "Set as input pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Set as input pin 2"]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            false => Pin2::Input,
            true => Pin2::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin2::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin2::Output
    }
}
#[doc = "Set as input pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin2WO> for bool {
    #[inline(always)]
    fn from(variant: Pin2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` writer - Set as input pin 2"]
pub type Pin2W<'a, REG> = crate::BitWriter1C<'a, REG, Pin2WO>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2WO::Clear)
    }
}
#[doc = "Set as input pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Set as input pin 3"]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            false => Pin3::Input,
            true => Pin3::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin3::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin3::Output
    }
}
#[doc = "Set as input pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin3WO> for bool {
    #[inline(always)]
    fn from(variant: Pin3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` writer - Set as input pin 3"]
pub type Pin3W<'a, REG> = crate::BitWriter1C<'a, REG, Pin3WO>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3WO::Clear)
    }
}
#[doc = "Set as input pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Set as input pin 4"]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            false => Pin4::Input,
            true => Pin4::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin4::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin4::Output
    }
}
#[doc = "Set as input pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin4WO> for bool {
    #[inline(always)]
    fn from(variant: Pin4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` writer - Set as input pin 4"]
pub type Pin4W<'a, REG> = crate::BitWriter1C<'a, REG, Pin4WO>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4WO::Clear)
    }
}
#[doc = "Set as input pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Set as input pin 5"]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            false => Pin5::Input,
            true => Pin5::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin5::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin5::Output
    }
}
#[doc = "Set as input pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin5WO> for bool {
    #[inline(always)]
    fn from(variant: Pin5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` writer - Set as input pin 5"]
pub type Pin5W<'a, REG> = crate::BitWriter1C<'a, REG, Pin5WO>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5WO::Clear)
    }
}
#[doc = "Set as input pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Set as input pin 6"]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            false => Pin6::Input,
            true => Pin6::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin6::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin6::Output
    }
}
#[doc = "Set as input pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin6WO> for bool {
    #[inline(always)]
    fn from(variant: Pin6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` writer - Set as input pin 6"]
pub type Pin6W<'a, REG> = crate::BitWriter1C<'a, REG, Pin6WO>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6WO::Clear)
    }
}
#[doc = "Set as input pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Set as input pin 7"]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            false => Pin7::Input,
            true => Pin7::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin7::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin7::Output
    }
}
#[doc = "Set as input pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin7WO> for bool {
    #[inline(always)]
    fn from(variant: Pin7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` writer - Set as input pin 7"]
pub type Pin7W<'a, REG> = crate::BitWriter1C<'a, REG, Pin7WO>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7WO::Clear)
    }
}
#[doc = "Set as input pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Set as input pin 8"]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            false => Pin8::Input,
            true => Pin8::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin8::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin8::Output
    }
}
#[doc = "Set as input pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin8WO> for bool {
    #[inline(always)]
    fn from(variant: Pin8WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` writer - Set as input pin 8"]
pub type Pin8W<'a, REG> = crate::BitWriter1C<'a, REG, Pin8WO>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8WO::Clear)
    }
}
#[doc = "Set as input pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Set as input pin 9"]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            false => Pin9::Input,
            true => Pin9::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin9::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin9::Output
    }
}
#[doc = "Set as input pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin9WO> for bool {
    #[inline(always)]
    fn from(variant: Pin9WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` writer - Set as input pin 9"]
pub type Pin9W<'a, REG> = crate::BitWriter1C<'a, REG, Pin9WO>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9WO::Clear)
    }
}
#[doc = "Set as input pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Set as input pin 10"]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            false => Pin10::Input,
            true => Pin10::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin10::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin10::Output
    }
}
#[doc = "Set as input pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin10WO> for bool {
    #[inline(always)]
    fn from(variant: Pin10WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` writer - Set as input pin 10"]
pub type Pin10W<'a, REG> = crate::BitWriter1C<'a, REG, Pin10WO>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10WO::Clear)
    }
}
#[doc = "Set as input pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Set as input pin 11"]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            false => Pin11::Input,
            true => Pin11::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin11::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin11::Output
    }
}
#[doc = "Set as input pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin11WO> for bool {
    #[inline(always)]
    fn from(variant: Pin11WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` writer - Set as input pin 11"]
pub type Pin11W<'a, REG> = crate::BitWriter1C<'a, REG, Pin11WO>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11WO::Clear)
    }
}
#[doc = "Set as input pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Set as input pin 12"]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            false => Pin12::Input,
            true => Pin12::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin12::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin12::Output
    }
}
#[doc = "Set as input pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin12WO> for bool {
    #[inline(always)]
    fn from(variant: Pin12WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` writer - Set as input pin 12"]
pub type Pin12W<'a, REG> = crate::BitWriter1C<'a, REG, Pin12WO>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12WO::Clear)
    }
}
#[doc = "Set as input pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Set as input pin 13"]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            false => Pin13::Input,
            true => Pin13::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin13::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin13::Output
    }
}
#[doc = "Set as input pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin13WO> for bool {
    #[inline(always)]
    fn from(variant: Pin13WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` writer - Set as input pin 13"]
pub type Pin13W<'a, REG> = crate::BitWriter1C<'a, REG, Pin13WO>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13WO::Clear)
    }
}
#[doc = "Set as input pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Set as input pin 14"]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            false => Pin14::Input,
            true => Pin14::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin14::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin14::Output
    }
}
#[doc = "Set as input pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin14WO> for bool {
    #[inline(always)]
    fn from(variant: Pin14WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` writer - Set as input pin 14"]
pub type Pin14W<'a, REG> = crate::BitWriter1C<'a, REG, Pin14WO>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14WO::Clear)
    }
}
#[doc = "Set as input pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Set as input pin 15"]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            false => Pin15::Input,
            true => Pin15::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin15::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin15::Output
    }
}
#[doc = "Set as input pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin15WO> for bool {
    #[inline(always)]
    fn from(variant: Pin15WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` writer - Set as input pin 15"]
pub type Pin15W<'a, REG> = crate::BitWriter1C<'a, REG, Pin15WO>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15WO::Clear)
    }
}
#[doc = "Set as input pin 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin16> for bool {
    #[inline(always)]
    fn from(variant: Pin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` reader - Set as input pin 16"]
pub type Pin16R = crate::BitReader<Pin16>;
impl Pin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin16 {
        match self.bits {
            false => Pin16::Input,
            true => Pin16::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin16::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin16::Output
    }
}
#[doc = "Set as input pin 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin16WO> for bool {
    #[inline(always)]
    fn from(variant: Pin16WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` writer - Set as input pin 16"]
pub type Pin16W<'a, REG> = crate::BitWriter1C<'a, REG, Pin16WO>;
impl<'a, REG> Pin16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16WO::Clear)
    }
}
#[doc = "Set as input pin 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin17> for bool {
    #[inline(always)]
    fn from(variant: Pin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` reader - Set as input pin 17"]
pub type Pin17R = crate::BitReader<Pin17>;
impl Pin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin17 {
        match self.bits {
            false => Pin17::Input,
            true => Pin17::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin17::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin17::Output
    }
}
#[doc = "Set as input pin 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin17WO> for bool {
    #[inline(always)]
    fn from(variant: Pin17WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` writer - Set as input pin 17"]
pub type Pin17W<'a, REG> = crate::BitWriter1C<'a, REG, Pin17WO>;
impl<'a, REG> Pin17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17WO::Clear)
    }
}
#[doc = "Set as input pin 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin18> for bool {
    #[inline(always)]
    fn from(variant: Pin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` reader - Set as input pin 18"]
pub type Pin18R = crate::BitReader<Pin18>;
impl Pin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin18 {
        match self.bits {
            false => Pin18::Input,
            true => Pin18::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin18::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin18::Output
    }
}
#[doc = "Set as input pin 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin18WO> for bool {
    #[inline(always)]
    fn from(variant: Pin18WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` writer - Set as input pin 18"]
pub type Pin18W<'a, REG> = crate::BitWriter1C<'a, REG, Pin18WO>;
impl<'a, REG> Pin18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18WO::Clear)
    }
}
#[doc = "Set as input pin 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin19> for bool {
    #[inline(always)]
    fn from(variant: Pin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` reader - Set as input pin 19"]
pub type Pin19R = crate::BitReader<Pin19>;
impl Pin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin19 {
        match self.bits {
            false => Pin19::Input,
            true => Pin19::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin19::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin19::Output
    }
}
#[doc = "Set as input pin 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin19WO> for bool {
    #[inline(always)]
    fn from(variant: Pin19WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` writer - Set as input pin 19"]
pub type Pin19W<'a, REG> = crate::BitWriter1C<'a, REG, Pin19WO>;
impl<'a, REG> Pin19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19WO::Clear)
    }
}
#[doc = "Set as input pin 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin20> for bool {
    #[inline(always)]
    fn from(variant: Pin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` reader - Set as input pin 20"]
pub type Pin20R = crate::BitReader<Pin20>;
impl Pin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin20 {
        match self.bits {
            false => Pin20::Input,
            true => Pin20::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin20::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin20::Output
    }
}
#[doc = "Set as input pin 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin20WO> for bool {
    #[inline(always)]
    fn from(variant: Pin20WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` writer - Set as input pin 20"]
pub type Pin20W<'a, REG> = crate::BitWriter1C<'a, REG, Pin20WO>;
impl<'a, REG> Pin20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20WO::Clear)
    }
}
#[doc = "Set as input pin 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin21> for bool {
    #[inline(always)]
    fn from(variant: Pin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` reader - Set as input pin 21"]
pub type Pin21R = crate::BitReader<Pin21>;
impl Pin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin21 {
        match self.bits {
            false => Pin21::Input,
            true => Pin21::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin21::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin21::Output
    }
}
#[doc = "Set as input pin 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin21WO> for bool {
    #[inline(always)]
    fn from(variant: Pin21WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` writer - Set as input pin 21"]
pub type Pin21W<'a, REG> = crate::BitWriter1C<'a, REG, Pin21WO>;
impl<'a, REG> Pin21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21WO::Clear)
    }
}
#[doc = "Set as input pin 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin22> for bool {
    #[inline(always)]
    fn from(variant: Pin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` reader - Set as input pin 22"]
pub type Pin22R = crate::BitReader<Pin22>;
impl Pin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin22 {
        match self.bits {
            false => Pin22::Input,
            true => Pin22::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin22::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin22::Output
    }
}
#[doc = "Set as input pin 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin22WO> for bool {
    #[inline(always)]
    fn from(variant: Pin22WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` writer - Set as input pin 22"]
pub type Pin22W<'a, REG> = crate::BitWriter1C<'a, REG, Pin22WO>;
impl<'a, REG> Pin22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22WO::Clear)
    }
}
#[doc = "Set as input pin 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin23> for bool {
    #[inline(always)]
    fn from(variant: Pin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` reader - Set as input pin 23"]
pub type Pin23R = crate::BitReader<Pin23>;
impl Pin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin23 {
        match self.bits {
            false => Pin23::Input,
            true => Pin23::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin23::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin23::Output
    }
}
#[doc = "Set as input pin 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin23WO> for bool {
    #[inline(always)]
    fn from(variant: Pin23WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` writer - Set as input pin 23"]
pub type Pin23W<'a, REG> = crate::BitWriter1C<'a, REG, Pin23WO>;
impl<'a, REG> Pin23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23WO::Clear)
    }
}
#[doc = "Set as input pin 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin24> for bool {
    #[inline(always)]
    fn from(variant: Pin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` reader - Set as input pin 24"]
pub type Pin24R = crate::BitReader<Pin24>;
impl Pin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin24 {
        match self.bits {
            false => Pin24::Input,
            true => Pin24::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin24::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin24::Output
    }
}
#[doc = "Set as input pin 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin24WO> for bool {
    #[inline(always)]
    fn from(variant: Pin24WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` writer - Set as input pin 24"]
pub type Pin24W<'a, REG> = crate::BitWriter1C<'a, REG, Pin24WO>;
impl<'a, REG> Pin24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24WO::Clear)
    }
}
#[doc = "Set as input pin 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin25> for bool {
    #[inline(always)]
    fn from(variant: Pin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` reader - Set as input pin 25"]
pub type Pin25R = crate::BitReader<Pin25>;
impl Pin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin25 {
        match self.bits {
            false => Pin25::Input,
            true => Pin25::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin25::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin25::Output
    }
}
#[doc = "Set as input pin 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin25WO> for bool {
    #[inline(always)]
    fn from(variant: Pin25WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` writer - Set as input pin 25"]
pub type Pin25W<'a, REG> = crate::BitWriter1C<'a, REG, Pin25WO>;
impl<'a, REG> Pin25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25WO::Clear)
    }
}
#[doc = "Set as input pin 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin26> for bool {
    #[inline(always)]
    fn from(variant: Pin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` reader - Set as input pin 26"]
pub type Pin26R = crate::BitReader<Pin26>;
impl Pin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin26 {
        match self.bits {
            false => Pin26::Input,
            true => Pin26::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin26::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin26::Output
    }
}
#[doc = "Set as input pin 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin26WO> for bool {
    #[inline(always)]
    fn from(variant: Pin26WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` writer - Set as input pin 26"]
pub type Pin26W<'a, REG> = crate::BitWriter1C<'a, REG, Pin26WO>;
impl<'a, REG> Pin26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26WO::Clear)
    }
}
#[doc = "Set as input pin 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin27> for bool {
    #[inline(always)]
    fn from(variant: Pin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` reader - Set as input pin 27"]
pub type Pin27R = crate::BitReader<Pin27>;
impl Pin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin27 {
        match self.bits {
            false => Pin27::Input,
            true => Pin27::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin27::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin27::Output
    }
}
#[doc = "Set as input pin 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin27WO> for bool {
    #[inline(always)]
    fn from(variant: Pin27WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` writer - Set as input pin 27"]
pub type Pin27W<'a, REG> = crate::BitWriter1C<'a, REG, Pin27WO>;
impl<'a, REG> Pin27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27WO::Clear)
    }
}
#[doc = "Set as input pin 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin28> for bool {
    #[inline(always)]
    fn from(variant: Pin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` reader - Set as input pin 28"]
pub type Pin28R = crate::BitReader<Pin28>;
impl Pin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin28 {
        match self.bits {
            false => Pin28::Input,
            true => Pin28::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin28::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin28::Output
    }
}
#[doc = "Set as input pin 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin28WO> for bool {
    #[inline(always)]
    fn from(variant: Pin28WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` writer - Set as input pin 28"]
pub type Pin28W<'a, REG> = crate::BitWriter1C<'a, REG, Pin28WO>;
impl<'a, REG> Pin28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28WO::Clear)
    }
}
#[doc = "Set as input pin 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin29> for bool {
    #[inline(always)]
    fn from(variant: Pin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` reader - Set as input pin 29"]
pub type Pin29R = crate::BitReader<Pin29>;
impl Pin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin29 {
        match self.bits {
            false => Pin29::Input,
            true => Pin29::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin29::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin29::Output
    }
}
#[doc = "Set as input pin 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin29WO> for bool {
    #[inline(always)]
    fn from(variant: Pin29WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` writer - Set as input pin 29"]
pub type Pin29W<'a, REG> = crate::BitWriter1C<'a, REG, Pin29WO>;
impl<'a, REG> Pin29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29WO::Clear)
    }
}
#[doc = "Set as input pin 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin30> for bool {
    #[inline(always)]
    fn from(variant: Pin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` reader - Set as input pin 30"]
pub type Pin30R = crate::BitReader<Pin30>;
impl Pin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin30 {
        match self.bits {
            false => Pin30::Input,
            true => Pin30::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin30::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin30::Output
    }
}
#[doc = "Set as input pin 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin30WO> for bool {
    #[inline(always)]
    fn from(variant: Pin30WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` writer - Set as input pin 30"]
pub type Pin30W<'a, REG> = crate::BitWriter1C<'a, REG, Pin30WO>;
impl<'a, REG> Pin30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30WO::Clear)
    }
}
#[doc = "Set as input pin 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31 {
    #[doc = "0: Read: pin set as input"]
    Input = 0,
    #[doc = "1: Read: pin set as output"]
    Output = 1,
}
impl From<Pin31> for bool {
    #[inline(always)]
    fn from(variant: Pin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` reader - Set as input pin 31"]
pub type Pin31R = crate::BitReader<Pin31>;
impl Pin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin31 {
        match self.bits {
            false => Pin31::Input,
            true => Pin31::Output,
        }
    }
    #[doc = "Read: pin set as input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Pin31::Input
    }
    #[doc = "Read: pin set as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Pin31::Output
    }
}
#[doc = "Set as input pin 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31WO {
    #[doc = "1: Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    Clear = 1,
}
impl From<Pin31WO> for bool {
    #[inline(always)]
    fn from(variant: Pin31WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` writer - Set as input pin 31"]
pub type Pin31W<'a, REG> = crate::BitWriter1C<'a, REG, Pin31WO>;
impl<'a, REG> Pin31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: writing a '1' sets pin to input; writing a '0' has no effect"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31WO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Set as input pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set as input pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set as input pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set as input pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set as input pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set as input pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set as input pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set as input pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set as input pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set as input pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set as input pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set as input pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set as input pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set as input pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set as input pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set as input pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set as input pin 16"]
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set as input pin 17"]
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set as input pin 18"]
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set as input pin 19"]
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set as input pin 20"]
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set as input pin 21"]
    #[inline(always)]
    pub fn pin21(&self) -> Pin21R {
        Pin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set as input pin 22"]
    #[inline(always)]
    pub fn pin22(&self) -> Pin22R {
        Pin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set as input pin 23"]
    #[inline(always)]
    pub fn pin23(&self) -> Pin23R {
        Pin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set as input pin 24"]
    #[inline(always)]
    pub fn pin24(&self) -> Pin24R {
        Pin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set as input pin 25"]
    #[inline(always)]
    pub fn pin25(&self) -> Pin25R {
        Pin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set as input pin 26"]
    #[inline(always)]
    pub fn pin26(&self) -> Pin26R {
        Pin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set as input pin 27"]
    #[inline(always)]
    pub fn pin27(&self) -> Pin27R {
        Pin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set as input pin 28"]
    #[inline(always)]
    pub fn pin28(&self) -> Pin28R {
        Pin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set as input pin 29"]
    #[inline(always)]
    pub fn pin29(&self) -> Pin29R {
        Pin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set as input pin 30"]
    #[inline(always)]
    pub fn pin30(&self) -> Pin30R {
        Pin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set as input pin 31"]
    #[inline(always)]
    pub fn pin31(&self) -> Pin31R {
        Pin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set as input pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, DirclrSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set as input pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, DirclrSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set as input pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, DirclrSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set as input pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, DirclrSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set as input pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, DirclrSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set as input pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, DirclrSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set as input pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, DirclrSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set as input pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, DirclrSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set as input pin 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, DirclrSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set as input pin 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, DirclrSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set as input pin 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, DirclrSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set as input pin 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, DirclrSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set as input pin 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, DirclrSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set as input pin 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, DirclrSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set as input pin 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, DirclrSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set as input pin 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, DirclrSpec> {
        Pin15W::new(self, 15)
    }
    #[doc = "Bit 16 - Set as input pin 16"]
    #[inline(always)]
    pub fn pin16(&mut self) -> Pin16W<'_, DirclrSpec> {
        Pin16W::new(self, 16)
    }
    #[doc = "Bit 17 - Set as input pin 17"]
    #[inline(always)]
    pub fn pin17(&mut self) -> Pin17W<'_, DirclrSpec> {
        Pin17W::new(self, 17)
    }
    #[doc = "Bit 18 - Set as input pin 18"]
    #[inline(always)]
    pub fn pin18(&mut self) -> Pin18W<'_, DirclrSpec> {
        Pin18W::new(self, 18)
    }
    #[doc = "Bit 19 - Set as input pin 19"]
    #[inline(always)]
    pub fn pin19(&mut self) -> Pin19W<'_, DirclrSpec> {
        Pin19W::new(self, 19)
    }
    #[doc = "Bit 20 - Set as input pin 20"]
    #[inline(always)]
    pub fn pin20(&mut self) -> Pin20W<'_, DirclrSpec> {
        Pin20W::new(self, 20)
    }
    #[doc = "Bit 21 - Set as input pin 21"]
    #[inline(always)]
    pub fn pin21(&mut self) -> Pin21W<'_, DirclrSpec> {
        Pin21W::new(self, 21)
    }
    #[doc = "Bit 22 - Set as input pin 22"]
    #[inline(always)]
    pub fn pin22(&mut self) -> Pin22W<'_, DirclrSpec> {
        Pin22W::new(self, 22)
    }
    #[doc = "Bit 23 - Set as input pin 23"]
    #[inline(always)]
    pub fn pin23(&mut self) -> Pin23W<'_, DirclrSpec> {
        Pin23W::new(self, 23)
    }
    #[doc = "Bit 24 - Set as input pin 24"]
    #[inline(always)]
    pub fn pin24(&mut self) -> Pin24W<'_, DirclrSpec> {
        Pin24W::new(self, 24)
    }
    #[doc = "Bit 25 - Set as input pin 25"]
    #[inline(always)]
    pub fn pin25(&mut self) -> Pin25W<'_, DirclrSpec> {
        Pin25W::new(self, 25)
    }
    #[doc = "Bit 26 - Set as input pin 26"]
    #[inline(always)]
    pub fn pin26(&mut self) -> Pin26W<'_, DirclrSpec> {
        Pin26W::new(self, 26)
    }
    #[doc = "Bit 27 - Set as input pin 27"]
    #[inline(always)]
    pub fn pin27(&mut self) -> Pin27W<'_, DirclrSpec> {
        Pin27W::new(self, 27)
    }
    #[doc = "Bit 28 - Set as input pin 28"]
    #[inline(always)]
    pub fn pin28(&mut self) -> Pin28W<'_, DirclrSpec> {
        Pin28W::new(self, 28)
    }
    #[doc = "Bit 29 - Set as input pin 29"]
    #[inline(always)]
    pub fn pin29(&mut self) -> Pin29W<'_, DirclrSpec> {
        Pin29W::new(self, 29)
    }
    #[doc = "Bit 30 - Set as input pin 30"]
    #[inline(always)]
    pub fn pin30(&mut self) -> Pin30W<'_, DirclrSpec> {
        Pin30W::new(self, 30)
    }
    #[doc = "Bit 31 - Set as input pin 31"]
    #[inline(always)]
    pub fn pin31(&mut self) -> Pin31W<'_, DirclrSpec> {
        Pin31W::new(self, 31)
    }
}
#[doc = "DIR clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirclrSpec;
impl crate::RegisterSpec for DirclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirclr::R`](R) reader structure"]
impl crate::Readable for DirclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DirclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets DIRCLR to value 0"]
impl crate::Resettable for DirclrSpec {}
