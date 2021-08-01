#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI Control Register"]
    pub ui2c_ctl: crate::Reg<ui2c_ctl::UI2C_CTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - USCI Baud Rate Generator Register"]
    pub ui2c_brgen: crate::Reg<ui2c_brgen::UI2C_BRGEN_SPEC>,
    _reserved2: [u8; 0x20],
    #[doc = "0x2c - USCI Line Control Register"]
    pub ui2c_linectl: crate::Reg<ui2c_linectl::UI2C_LINECTL_SPEC>,
    #[doc = "0x30 - USCI Transmit Data Register"]
    pub ui2c_txdat: crate::Reg<ui2c_txdat::UI2C_TXDAT_SPEC>,
    #[doc = "0x34 - USCI Receive Data Register"]
    pub ui2c_rxdat: crate::Reg<ui2c_rxdat::UI2C_RXDAT_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x44 - USCI Device Address Register 0"]
    pub ui2c_devaddr0: crate::Reg<ui2c_devaddr0::UI2C_DEVADDR0_SPEC>,
    #[doc = "0x48 - USCI Device Address Register 1"]
    pub ui2c_devaddr1: crate::Reg<ui2c_devaddr1::UI2C_DEVADDR1_SPEC>,
    #[doc = "0x4c - USCI Device Address Mask Register 0"]
    pub ui2c_addrmsk0: crate::Reg<ui2c_addrmsk0::UI2C_ADDRMSK0_SPEC>,
    #[doc = "0x50 - USCI Device Address Mask Register 1"]
    pub ui2c_addrmsk1: crate::Reg<ui2c_addrmsk1::UI2C_ADDRMSK1_SPEC>,
    #[doc = "0x54 - USCI Wake-up Control Register"]
    pub ui2c_wkctl: crate::Reg<ui2c_wkctl::UI2C_WKCTL_SPEC>,
    #[doc = "0x58 - USCI Wake-up Status Register"]
    pub ui2c_wksts: crate::Reg<ui2c_wksts::UI2C_WKSTS_SPEC>,
    #[doc = "0x5c - USCI Protocol Control Register"]
    pub ui2c_protctl: crate::Reg<ui2c_protctl::UI2C_PROTCTL_SPEC>,
    #[doc = "0x60 - USCI Protocol Interrupt Enable Register"]
    pub ui2c_protien: crate::Reg<ui2c_protien::UI2C_PROTIEN_SPEC>,
    #[doc = "0x64 - USCI Protocol Status Register"]
    pub ui2c_protsts: crate::Reg<ui2c_protsts::UI2C_PROTSTS_SPEC>,
    _reserved14: [u8; 0x20],
    #[doc = "0x88 - I2C Slave Match Address Register"]
    pub ui2c_admat: crate::Reg<ui2c_admat::UI2C_ADMAT_SPEC>,
    #[doc = "0x8c - I2C Timing Configure Control Register"]
    pub ui2c_tmctl: crate::Reg<ui2c_tmctl::UI2C_TMCTL_SPEC>,
}
#[doc = "UI2C_CTL register accessor: an alias for `Reg<UI2C_CTL_SPEC>`"]
pub type UI2C_CTL = crate::Reg<ui2c_ctl::UI2C_CTL_SPEC>;
#[doc = "USCI Control Register"]
pub mod ui2c_ctl;
#[doc = "UI2C_BRGEN register accessor: an alias for `Reg<UI2C_BRGEN_SPEC>`"]
pub type UI2C_BRGEN = crate::Reg<ui2c_brgen::UI2C_BRGEN_SPEC>;
#[doc = "USCI Baud Rate Generator Register"]
pub mod ui2c_brgen;
#[doc = "UI2C_LINECTL register accessor: an alias for `Reg<UI2C_LINECTL_SPEC>`"]
pub type UI2C_LINECTL = crate::Reg<ui2c_linectl::UI2C_LINECTL_SPEC>;
#[doc = "USCI Line Control Register"]
pub mod ui2c_linectl;
#[doc = "UI2C_TXDAT register accessor: an alias for `Reg<UI2C_TXDAT_SPEC>`"]
pub type UI2C_TXDAT = crate::Reg<ui2c_txdat::UI2C_TXDAT_SPEC>;
#[doc = "USCI Transmit Data Register"]
pub mod ui2c_txdat;
#[doc = "UI2C_RXDAT register accessor: an alias for `Reg<UI2C_RXDAT_SPEC>`"]
pub type UI2C_RXDAT = crate::Reg<ui2c_rxdat::UI2C_RXDAT_SPEC>;
#[doc = "USCI Receive Data Register"]
pub mod ui2c_rxdat;
#[doc = "UI2C_DEVADDR0 register accessor: an alias for `Reg<UI2C_DEVADDR0_SPEC>`"]
pub type UI2C_DEVADDR0 = crate::Reg<ui2c_devaddr0::UI2C_DEVADDR0_SPEC>;
#[doc = "USCI Device Address Register 0"]
pub mod ui2c_devaddr0;
#[doc = "UI2C_DEVADDR1 register accessor: an alias for `Reg<UI2C_DEVADDR1_SPEC>`"]
pub type UI2C_DEVADDR1 = crate::Reg<ui2c_devaddr1::UI2C_DEVADDR1_SPEC>;
#[doc = "USCI Device Address Register 1"]
pub mod ui2c_devaddr1;
#[doc = "UI2C_ADDRMSK0 register accessor: an alias for `Reg<UI2C_ADDRMSK0_SPEC>`"]
pub type UI2C_ADDRMSK0 = crate::Reg<ui2c_addrmsk0::UI2C_ADDRMSK0_SPEC>;
#[doc = "USCI Device Address Mask Register 0"]
pub mod ui2c_addrmsk0;
#[doc = "UI2C_ADDRMSK1 register accessor: an alias for `Reg<UI2C_ADDRMSK1_SPEC>`"]
pub type UI2C_ADDRMSK1 = crate::Reg<ui2c_addrmsk1::UI2C_ADDRMSK1_SPEC>;
#[doc = "USCI Device Address Mask Register 1"]
pub mod ui2c_addrmsk1;
#[doc = "UI2C_WKCTL register accessor: an alias for `Reg<UI2C_WKCTL_SPEC>`"]
pub type UI2C_WKCTL = crate::Reg<ui2c_wkctl::UI2C_WKCTL_SPEC>;
#[doc = "USCI Wake-up Control Register"]
pub mod ui2c_wkctl;
#[doc = "UI2C_WKSTS register accessor: an alias for `Reg<UI2C_WKSTS_SPEC>`"]
pub type UI2C_WKSTS = crate::Reg<ui2c_wksts::UI2C_WKSTS_SPEC>;
#[doc = "USCI Wake-up Status Register"]
pub mod ui2c_wksts;
#[doc = "UI2C_PROTCTL register accessor: an alias for `Reg<UI2C_PROTCTL_SPEC>`"]
pub type UI2C_PROTCTL = crate::Reg<ui2c_protctl::UI2C_PROTCTL_SPEC>;
#[doc = "USCI Protocol Control Register"]
pub mod ui2c_protctl;
#[doc = "UI2C_PROTIEN register accessor: an alias for `Reg<UI2C_PROTIEN_SPEC>`"]
pub type UI2C_PROTIEN = crate::Reg<ui2c_protien::UI2C_PROTIEN_SPEC>;
#[doc = "USCI Protocol Interrupt Enable Register"]
pub mod ui2c_protien;
#[doc = "UI2C_PROTSTS register accessor: an alias for `Reg<UI2C_PROTSTS_SPEC>`"]
pub type UI2C_PROTSTS = crate::Reg<ui2c_protsts::UI2C_PROTSTS_SPEC>;
#[doc = "USCI Protocol Status Register"]
pub mod ui2c_protsts;
#[doc = "UI2C_ADMAT register accessor: an alias for `Reg<UI2C_ADMAT_SPEC>`"]
pub type UI2C_ADMAT = crate::Reg<ui2c_admat::UI2C_ADMAT_SPEC>;
#[doc = "I2C Slave Match Address Register"]
pub mod ui2c_admat;
#[doc = "UI2C_TMCTL register accessor: an alias for `Reg<UI2C_TMCTL_SPEC>`"]
pub type UI2C_TMCTL = crate::Reg<ui2c_tmctl::UI2C_TMCTL_SPEC>;
#[doc = "I2C Timing Configure Control Register"]
pub mod ui2c_tmctl;
