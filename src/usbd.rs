#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Interrupt Enable Register"]
    pub usbd_inten: crate::Reg<usbd_inten::USBD_INTEN_SPEC>,
    #[doc = "0x04 - USB Device Interrupt Event Status Register"]
    pub usbd_intsts: crate::Reg<usbd_intsts::USBD_INTSTS_SPEC>,
    #[doc = "0x08 - USB Device Function Address Register"]
    pub usbd_faddr: crate::Reg<usbd_faddr::USBD_FADDR_SPEC>,
    #[doc = "0x0c - USB Device Endpoint Status Register"]
    pub usbd_epsts: crate::Reg<usbd_epsts::USBD_EPSTS_SPEC>,
    #[doc = "0x10 - USB Device Bus Status and Attribution Register"]
    pub usbd_attr: crate::Reg<usbd_attr::USBD_ATTR_SPEC>,
    #[doc = "0x14 - USB Device VBUS Detection Register"]
    pub usbd_vbusdet: crate::Reg<usbd_vbusdet::USBD_VBUSDET_SPEC>,
    #[doc = "0x18 - SETUP Token Buffer Segmentation Register"]
    pub usbd_stbufseg: crate::Reg<usbd_stbufseg::USBD_STBUFSEG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - USB Device Endpoint Status Register 0"]
    pub usbd_epsts0: crate::Reg<usbd_epsts0::USBD_EPSTS0_SPEC>,
    _reserved8: [u8; 0x64],
    #[doc = "0x88 - USB LPM Attribution Register"]
    pub usbd_lpmattr: crate::Reg<usbd_lpmattr::USBD_LPMATTR_SPEC>,
    #[doc = "0x8c - USB Frame Number Register"]
    pub usbd_fn: crate::Reg<usbd_fn::USBD_FN_SPEC>,
    #[doc = "0x90 - USB Device Drive SE0 Control Register"]
    pub usbd_se0: crate::Reg<usbd_se0::USBD_SE0_SPEC>,
    _reserved11: [u8; 0x046c],
    #[doc = "0x500 - Endpoint 0 Buffer Segmentation Register"]
    pub usbd_bufseg0: crate::Reg<usbd_bufseg0::USBD_BUFSEG0_SPEC>,
    #[doc = "0x504 - Endpoint 0 Maximal Payload Register"]
    pub usbd_mxpld0: crate::Reg<usbd_mxpld0::USBD_MXPLD0_SPEC>,
    #[doc = "0x508 - Endpoint 0 Configuration Register"]
    pub usbd_cfg0: crate::Reg<usbd_cfg0::USBD_CFG0_SPEC>,
    #[doc = "0x50c - Endpoint 0 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp0: crate::Reg<usbd_cfgp0::USBD_CFGP0_SPEC>,
    #[doc = "0x510 - Endpoint 1 Buffer Segmentation Register"]
    pub usbd_bufseg1: crate::Reg<usbd_bufseg1::USBD_BUFSEG1_SPEC>,
    #[doc = "0x514 - Endpoint 1 Maximal Payload Register"]
    pub usbd_mxpld1: crate::Reg<usbd_mxpld1::USBD_MXPLD1_SPEC>,
    #[doc = "0x518 - Endpoint 1 Configuration Register"]
    pub usbd_cfg1: crate::Reg<usbd_cfg1::USBD_CFG1_SPEC>,
    #[doc = "0x51c - Endpoint 1 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp1: crate::Reg<usbd_cfgp1::USBD_CFGP1_SPEC>,
    #[doc = "0x520 - Endpoint 2 Buffer Segmentation Register"]
    pub usbd_bufseg2: crate::Reg<usbd_bufseg2::USBD_BUFSEG2_SPEC>,
    #[doc = "0x524 - Endpoint 2 Maximal Payload Register"]
    pub usbd_mxpld2: crate::Reg<usbd_mxpld2::USBD_MXPLD2_SPEC>,
    #[doc = "0x528 - Endpoint 2 Configuration Register"]
    pub usbd_cfg2: crate::Reg<usbd_cfg2::USBD_CFG2_SPEC>,
    #[doc = "0x52c - Endpoint 2 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp2: crate::Reg<usbd_cfgp2::USBD_CFGP2_SPEC>,
    #[doc = "0x530 - Endpoint 3 Buffer Segmentation Register"]
    pub usbd_bufseg3: crate::Reg<usbd_bufseg3::USBD_BUFSEG3_SPEC>,
    #[doc = "0x534 - Endpoint 3 Maximal Payload Register"]
    pub usbd_mxpld3: crate::Reg<usbd_mxpld3::USBD_MXPLD3_SPEC>,
    #[doc = "0x538 - Endpoint 3 Configuration Register"]
    pub usbd_cfg3: crate::Reg<usbd_cfg3::USBD_CFG3_SPEC>,
    #[doc = "0x53c - Endpoint 3 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp3: crate::Reg<usbd_cfgp3::USBD_CFGP3_SPEC>,
    #[doc = "0x540 - Endpoint 4 Buffer Segmentation Register"]
    pub usbd_bufseg4: crate::Reg<usbd_bufseg4::USBD_BUFSEG4_SPEC>,
    #[doc = "0x544 - Endpoint 4 Maximal Payload Register"]
    pub usbd_mxpld4: crate::Reg<usbd_mxpld4::USBD_MXPLD4_SPEC>,
    #[doc = "0x548 - Endpoint 4 Configuration Register"]
    pub usbd_cfg4: crate::Reg<usbd_cfg4::USBD_CFG4_SPEC>,
    #[doc = "0x54c - Endpoint 4 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp4: crate::Reg<usbd_cfgp4::USBD_CFGP4_SPEC>,
    #[doc = "0x550 - Endpoint 5 Buffer Segmentation Register"]
    pub usbd_bufseg5: crate::Reg<usbd_bufseg5::USBD_BUFSEG5_SPEC>,
    #[doc = "0x554 - Endpoint 5 Maximal Payload Register"]
    pub usbd_mxpld5: crate::Reg<usbd_mxpld5::USBD_MXPLD5_SPEC>,
    #[doc = "0x558 - Endpoint 5 Configuration Register"]
    pub usbd_cfg5: crate::Reg<usbd_cfg5::USBD_CFG5_SPEC>,
    #[doc = "0x55c - Endpoint 5 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp5: crate::Reg<usbd_cfgp5::USBD_CFGP5_SPEC>,
    #[doc = "0x560 - Endpoint 6 Buffer Segmentation Register"]
    pub usbd_bufseg6: crate::Reg<usbd_bufseg6::USBD_BUFSEG6_SPEC>,
    #[doc = "0x564 - Endpoint 6 Maximal Payload Register"]
    pub usbd_mxpld6: crate::Reg<usbd_mxpld6::USBD_MXPLD6_SPEC>,
    #[doc = "0x568 - Endpoint 6 Configuration Register"]
    pub usbd_cfg6: crate::Reg<usbd_cfg6::USBD_CFG6_SPEC>,
    #[doc = "0x56c - Endpoint 6 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp6: crate::Reg<usbd_cfgp6::USBD_CFGP6_SPEC>,
    #[doc = "0x570 - Endpoint 7 Buffer Segmentation Register"]
    pub usbd_bufseg7: crate::Reg<usbd_bufseg7::USBD_BUFSEG7_SPEC>,
    #[doc = "0x574 - Endpoint 7 Maximal Payload Register"]
    pub usbd_mxpld7: crate::Reg<usbd_mxpld7::USBD_MXPLD7_SPEC>,
    #[doc = "0x578 - Endpoint 7 Configuration Register"]
    pub usbd_cfg7: crate::Reg<usbd_cfg7::USBD_CFG7_SPEC>,
    #[doc = "0x57c - Endpoint 7 Set Stall and Clear In/Out Ready Control Register"]
    pub usbd_cfgp7: crate::Reg<usbd_cfgp7::USBD_CFGP7_SPEC>,
}
#[doc = "USBD_INTEN register accessor: an alias for `Reg<USBD_INTEN_SPEC>`"]
pub type USBD_INTEN = crate::Reg<usbd_inten::USBD_INTEN_SPEC>;
#[doc = "USB Device Interrupt Enable Register"]
pub mod usbd_inten;
#[doc = "USBD_INTSTS register accessor: an alias for `Reg<USBD_INTSTS_SPEC>`"]
pub type USBD_INTSTS = crate::Reg<usbd_intsts::USBD_INTSTS_SPEC>;
#[doc = "USB Device Interrupt Event Status Register"]
pub mod usbd_intsts;
#[doc = "USBD_FADDR register accessor: an alias for `Reg<USBD_FADDR_SPEC>`"]
pub type USBD_FADDR = crate::Reg<usbd_faddr::USBD_FADDR_SPEC>;
#[doc = "USB Device Function Address Register"]
pub mod usbd_faddr;
#[doc = "USBD_EPSTS register accessor: an alias for `Reg<USBD_EPSTS_SPEC>`"]
pub type USBD_EPSTS = crate::Reg<usbd_epsts::USBD_EPSTS_SPEC>;
#[doc = "USB Device Endpoint Status Register"]
pub mod usbd_epsts;
#[doc = "USBD_ATTR register accessor: an alias for `Reg<USBD_ATTR_SPEC>`"]
pub type USBD_ATTR = crate::Reg<usbd_attr::USBD_ATTR_SPEC>;
#[doc = "USB Device Bus Status and Attribution Register"]
pub mod usbd_attr;
#[doc = "USBD_VBUSDET register accessor: an alias for `Reg<USBD_VBUSDET_SPEC>`"]
pub type USBD_VBUSDET = crate::Reg<usbd_vbusdet::USBD_VBUSDET_SPEC>;
#[doc = "USB Device VBUS Detection Register"]
pub mod usbd_vbusdet;
#[doc = "USBD_STBUFSEG register accessor: an alias for `Reg<USBD_STBUFSEG_SPEC>`"]
pub type USBD_STBUFSEG = crate::Reg<usbd_stbufseg::USBD_STBUFSEG_SPEC>;
#[doc = "SETUP Token Buffer Segmentation Register"]
pub mod usbd_stbufseg;
#[doc = "USBD_EPSTS0 register accessor: an alias for `Reg<USBD_EPSTS0_SPEC>`"]
pub type USBD_EPSTS0 = crate::Reg<usbd_epsts0::USBD_EPSTS0_SPEC>;
#[doc = "USB Device Endpoint Status Register 0"]
pub mod usbd_epsts0;
#[doc = "USBD_LPMATTR register accessor: an alias for `Reg<USBD_LPMATTR_SPEC>`"]
pub type USBD_LPMATTR = crate::Reg<usbd_lpmattr::USBD_LPMATTR_SPEC>;
#[doc = "USB LPM Attribution Register"]
pub mod usbd_lpmattr;
#[doc = "USBD_FN register accessor: an alias for `Reg<USBD_FN_SPEC>`"]
pub type USBD_FN = crate::Reg<usbd_fn::USBD_FN_SPEC>;
#[doc = "USB Frame Number Register"]
pub mod usbd_fn;
#[doc = "USBD_SE0 register accessor: an alias for `Reg<USBD_SE0_SPEC>`"]
pub type USBD_SE0 = crate::Reg<usbd_se0::USBD_SE0_SPEC>;
#[doc = "USB Device Drive SE0 Control Register"]
pub mod usbd_se0;
#[doc = "USBD_BUFSEG0 register accessor: an alias for `Reg<USBD_BUFSEG0_SPEC>`"]
pub type USBD_BUFSEG0 = crate::Reg<usbd_bufseg0::USBD_BUFSEG0_SPEC>;
#[doc = "Endpoint 0 Buffer Segmentation Register"]
pub mod usbd_bufseg0;
#[doc = "USBD_MXPLD0 register accessor: an alias for `Reg<USBD_MXPLD0_SPEC>`"]
pub type USBD_MXPLD0 = crate::Reg<usbd_mxpld0::USBD_MXPLD0_SPEC>;
#[doc = "Endpoint 0 Maximal Payload Register"]
pub mod usbd_mxpld0;
#[doc = "USBD_CFG0 register accessor: an alias for `Reg<USBD_CFG0_SPEC>`"]
pub type USBD_CFG0 = crate::Reg<usbd_cfg0::USBD_CFG0_SPEC>;
#[doc = "Endpoint 0 Configuration Register"]
pub mod usbd_cfg0;
#[doc = "USBD_CFGP0 register accessor: an alias for `Reg<USBD_CFGP0_SPEC>`"]
pub type USBD_CFGP0 = crate::Reg<usbd_cfgp0::USBD_CFGP0_SPEC>;
#[doc = "Endpoint 0 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp0;
#[doc = "USBD_BUFSEG1 register accessor: an alias for `Reg<USBD_BUFSEG1_SPEC>`"]
pub type USBD_BUFSEG1 = crate::Reg<usbd_bufseg1::USBD_BUFSEG1_SPEC>;
#[doc = "Endpoint 1 Buffer Segmentation Register"]
pub mod usbd_bufseg1;
#[doc = "USBD_MXPLD1 register accessor: an alias for `Reg<USBD_MXPLD1_SPEC>`"]
pub type USBD_MXPLD1 = crate::Reg<usbd_mxpld1::USBD_MXPLD1_SPEC>;
#[doc = "Endpoint 1 Maximal Payload Register"]
pub mod usbd_mxpld1;
#[doc = "USBD_CFG1 register accessor: an alias for `Reg<USBD_CFG1_SPEC>`"]
pub type USBD_CFG1 = crate::Reg<usbd_cfg1::USBD_CFG1_SPEC>;
#[doc = "Endpoint 1 Configuration Register"]
pub mod usbd_cfg1;
#[doc = "USBD_CFGP1 register accessor: an alias for `Reg<USBD_CFGP1_SPEC>`"]
pub type USBD_CFGP1 = crate::Reg<usbd_cfgp1::USBD_CFGP1_SPEC>;
#[doc = "Endpoint 1 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp1;
#[doc = "USBD_BUFSEG2 register accessor: an alias for `Reg<USBD_BUFSEG2_SPEC>`"]
pub type USBD_BUFSEG2 = crate::Reg<usbd_bufseg2::USBD_BUFSEG2_SPEC>;
#[doc = "Endpoint 2 Buffer Segmentation Register"]
pub mod usbd_bufseg2;
#[doc = "USBD_MXPLD2 register accessor: an alias for `Reg<USBD_MXPLD2_SPEC>`"]
pub type USBD_MXPLD2 = crate::Reg<usbd_mxpld2::USBD_MXPLD2_SPEC>;
#[doc = "Endpoint 2 Maximal Payload Register"]
pub mod usbd_mxpld2;
#[doc = "USBD_CFG2 register accessor: an alias for `Reg<USBD_CFG2_SPEC>`"]
pub type USBD_CFG2 = crate::Reg<usbd_cfg2::USBD_CFG2_SPEC>;
#[doc = "Endpoint 2 Configuration Register"]
pub mod usbd_cfg2;
#[doc = "USBD_CFGP2 register accessor: an alias for `Reg<USBD_CFGP2_SPEC>`"]
pub type USBD_CFGP2 = crate::Reg<usbd_cfgp2::USBD_CFGP2_SPEC>;
#[doc = "Endpoint 2 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp2;
#[doc = "USBD_BUFSEG3 register accessor: an alias for `Reg<USBD_BUFSEG3_SPEC>`"]
pub type USBD_BUFSEG3 = crate::Reg<usbd_bufseg3::USBD_BUFSEG3_SPEC>;
#[doc = "Endpoint 3 Buffer Segmentation Register"]
pub mod usbd_bufseg3;
#[doc = "USBD_MXPLD3 register accessor: an alias for `Reg<USBD_MXPLD3_SPEC>`"]
pub type USBD_MXPLD3 = crate::Reg<usbd_mxpld3::USBD_MXPLD3_SPEC>;
#[doc = "Endpoint 3 Maximal Payload Register"]
pub mod usbd_mxpld3;
#[doc = "USBD_CFG3 register accessor: an alias for `Reg<USBD_CFG3_SPEC>`"]
pub type USBD_CFG3 = crate::Reg<usbd_cfg3::USBD_CFG3_SPEC>;
#[doc = "Endpoint 3 Configuration Register"]
pub mod usbd_cfg3;
#[doc = "USBD_CFGP3 register accessor: an alias for `Reg<USBD_CFGP3_SPEC>`"]
pub type USBD_CFGP3 = crate::Reg<usbd_cfgp3::USBD_CFGP3_SPEC>;
#[doc = "Endpoint 3 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp3;
#[doc = "USBD_BUFSEG4 register accessor: an alias for `Reg<USBD_BUFSEG4_SPEC>`"]
pub type USBD_BUFSEG4 = crate::Reg<usbd_bufseg4::USBD_BUFSEG4_SPEC>;
#[doc = "Endpoint 4 Buffer Segmentation Register"]
pub mod usbd_bufseg4;
#[doc = "USBD_MXPLD4 register accessor: an alias for `Reg<USBD_MXPLD4_SPEC>`"]
pub type USBD_MXPLD4 = crate::Reg<usbd_mxpld4::USBD_MXPLD4_SPEC>;
#[doc = "Endpoint 4 Maximal Payload Register"]
pub mod usbd_mxpld4;
#[doc = "USBD_CFG4 register accessor: an alias for `Reg<USBD_CFG4_SPEC>`"]
pub type USBD_CFG4 = crate::Reg<usbd_cfg4::USBD_CFG4_SPEC>;
#[doc = "Endpoint 4 Configuration Register"]
pub mod usbd_cfg4;
#[doc = "USBD_CFGP4 register accessor: an alias for `Reg<USBD_CFGP4_SPEC>`"]
pub type USBD_CFGP4 = crate::Reg<usbd_cfgp4::USBD_CFGP4_SPEC>;
#[doc = "Endpoint 4 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp4;
#[doc = "USBD_BUFSEG5 register accessor: an alias for `Reg<USBD_BUFSEG5_SPEC>`"]
pub type USBD_BUFSEG5 = crate::Reg<usbd_bufseg5::USBD_BUFSEG5_SPEC>;
#[doc = "Endpoint 5 Buffer Segmentation Register"]
pub mod usbd_bufseg5;
#[doc = "USBD_MXPLD5 register accessor: an alias for `Reg<USBD_MXPLD5_SPEC>`"]
pub type USBD_MXPLD5 = crate::Reg<usbd_mxpld5::USBD_MXPLD5_SPEC>;
#[doc = "Endpoint 5 Maximal Payload Register"]
pub mod usbd_mxpld5;
#[doc = "USBD_CFG5 register accessor: an alias for `Reg<USBD_CFG5_SPEC>`"]
pub type USBD_CFG5 = crate::Reg<usbd_cfg5::USBD_CFG5_SPEC>;
#[doc = "Endpoint 5 Configuration Register"]
pub mod usbd_cfg5;
#[doc = "USBD_CFGP5 register accessor: an alias for `Reg<USBD_CFGP5_SPEC>`"]
pub type USBD_CFGP5 = crate::Reg<usbd_cfgp5::USBD_CFGP5_SPEC>;
#[doc = "Endpoint 5 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp5;
#[doc = "USBD_BUFSEG6 register accessor: an alias for `Reg<USBD_BUFSEG6_SPEC>`"]
pub type USBD_BUFSEG6 = crate::Reg<usbd_bufseg6::USBD_BUFSEG6_SPEC>;
#[doc = "Endpoint 6 Buffer Segmentation Register"]
pub mod usbd_bufseg6;
#[doc = "USBD_MXPLD6 register accessor: an alias for `Reg<USBD_MXPLD6_SPEC>`"]
pub type USBD_MXPLD6 = crate::Reg<usbd_mxpld6::USBD_MXPLD6_SPEC>;
#[doc = "Endpoint 6 Maximal Payload Register"]
pub mod usbd_mxpld6;
#[doc = "USBD_CFG6 register accessor: an alias for `Reg<USBD_CFG6_SPEC>`"]
pub type USBD_CFG6 = crate::Reg<usbd_cfg6::USBD_CFG6_SPEC>;
#[doc = "Endpoint 6 Configuration Register"]
pub mod usbd_cfg6;
#[doc = "USBD_CFGP6 register accessor: an alias for `Reg<USBD_CFGP6_SPEC>`"]
pub type USBD_CFGP6 = crate::Reg<usbd_cfgp6::USBD_CFGP6_SPEC>;
#[doc = "Endpoint 6 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp6;
#[doc = "USBD_BUFSEG7 register accessor: an alias for `Reg<USBD_BUFSEG7_SPEC>`"]
pub type USBD_BUFSEG7 = crate::Reg<usbd_bufseg7::USBD_BUFSEG7_SPEC>;
#[doc = "Endpoint 7 Buffer Segmentation Register"]
pub mod usbd_bufseg7;
#[doc = "USBD_MXPLD7 register accessor: an alias for `Reg<USBD_MXPLD7_SPEC>`"]
pub type USBD_MXPLD7 = crate::Reg<usbd_mxpld7::USBD_MXPLD7_SPEC>;
#[doc = "Endpoint 7 Maximal Payload Register"]
pub mod usbd_mxpld7;
#[doc = "USBD_CFG7 register accessor: an alias for `Reg<USBD_CFG7_SPEC>`"]
pub type USBD_CFG7 = crate::Reg<usbd_cfg7::USBD_CFG7_SPEC>;
#[doc = "Endpoint 7 Configuration Register"]
pub mod usbd_cfg7;
#[doc = "USBD_CFGP7 register accessor: an alias for `Reg<USBD_CFGP7_SPEC>`"]
pub type USBD_CFGP7 = crate::Reg<usbd_cfgp7::USBD_CFGP7_SPEC>;
#[doc = "Endpoint 7 Set Stall and Clear In/Out Ready Control Register"]
pub mod usbd_cfgp7;
