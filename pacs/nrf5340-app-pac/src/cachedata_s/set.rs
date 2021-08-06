#[doc = r"Register block"]
#[repr(C)]
pub struct WAY {
    #[doc = "0x00 - Description cluster: Cache data bits \\[31:0\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data0: crate::Reg<self::way::data0::DATA0_SPEC>,
    #[doc = "0x04 - Description cluster: Cache data bits \\[63:32\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data1: crate::Reg<self::way::data1::DATA1_SPEC>,
    #[doc = "0x08 - Description cluster: Cache data bits \\[95:64\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data2: crate::Reg<self::way::data2::DATA2_SPEC>,
    #[doc = "0x0c - Description cluster: Cache data bits \\[127:96\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data3: crate::Reg<self::way::data3::DATA3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod way;
