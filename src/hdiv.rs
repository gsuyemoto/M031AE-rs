#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dividend Source Register"]
    pub dividend: crate::Reg<dividend::DIVIDEND_SPEC>,
    #[doc = "0x04 - Divisor Source Resister"]
    pub divisor: crate::Reg<divisor::DIVISOR_SPEC>,
    #[doc = "0x08 - Quotient Result Resister"]
    pub divquo: crate::Reg<divquo::DIVQUO_SPEC>,
    #[doc = "0x0c - Remainder Result Register"]
    pub divrem: crate::Reg<divrem::DIVREM_SPEC>,
    #[doc = "0x10 - Divider Status Register"]
    pub divsts: crate::Reg<divsts::DIVSTS_SPEC>,
}
#[doc = "DIVIDEND register accessor: an alias for `Reg<DIVIDEND_SPEC>`"]
pub type DIVIDEND = crate::Reg<dividend::DIVIDEND_SPEC>;
#[doc = "Dividend Source Register"]
pub mod dividend;
#[doc = "DIVISOR register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "Divisor Source Resister"]
pub mod divisor;
#[doc = "DIVQUO register accessor: an alias for `Reg<DIVQUO_SPEC>`"]
pub type DIVQUO = crate::Reg<divquo::DIVQUO_SPEC>;
#[doc = "Quotient Result Resister"]
pub mod divquo;
#[doc = "DIVREM register accessor: an alias for `Reg<DIVREM_SPEC>`"]
pub type DIVREM = crate::Reg<divrem::DIVREM_SPEC>;
#[doc = "Remainder Result Register"]
pub mod divrem;
#[doc = "DIVSTS register accessor: an alias for `Reg<DIVSTS_SPEC>`"]
pub type DIVSTS = crate::Reg<divsts::DIVSTS_SPEC>;
#[doc = "Divider Status Register"]
pub mod divsts;
