#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Part Device Identification Number Register"]
    pub sys_pdid: crate::Reg<sys_pdid::SYS_PDID_SPEC>,
    #[doc = "0x04 - System Reset Status Register"]
    pub sys_rststs: crate::Reg<sys_rststs::SYS_RSTSTS_SPEC>,
    #[doc = "0x08 - Peripheral Reset Control Register 0"]
    pub sys_iprst0: crate::Reg<sys_iprst0::SYS_IPRST0_SPEC>,
    #[doc = "0x0c - Peripheral Reset Control Register 1"]
    pub sys_iprst1: crate::Reg<sys_iprst1::SYS_IPRST1_SPEC>,
    #[doc = "0x10 - Peripheral Reset Control Register 2"]
    pub sys_iprst2: crate::Reg<sys_iprst2::SYS_IPRST2_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Brown-out Detector Control Register"]
    pub sys_bodctl: crate::Reg<sys_bodctl::SYS_BODCTL_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x24 - Power-On-reset Controller Register"]
    pub sys_porctl: crate::Reg<sys_porctl::SYS_PORCTL_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - GPIOA Low Byte Multiple Function Control Register"]
    pub sys_gpa_mfpl: crate::Reg<sys_gpa_mfpl::SYS_GPA_MFPL_SPEC>,
    #[doc = "0x34 - GPIOA High Byte Multiple Function Control Register"]
    pub sys_gpa_mfph: crate::Reg<sys_gpa_mfph::SYS_GPA_MFPH_SPEC>,
    #[doc = "0x38 - GPIOB Low Byte Multiple Function Control Register"]
    pub sys_gpb_mfpl: crate::Reg<sys_gpb_mfpl::SYS_GPB_MFPL_SPEC>,
    #[doc = "0x3c - GPIOB High Byte Multiple Function Control Register"]
    pub sys_gpb_mfph: crate::Reg<sys_gpb_mfph::SYS_GPB_MFPH_SPEC>,
    #[doc = "0x40 - GPIOC Low Byte Multiple Function Control Register"]
    pub sys_gpc_mfpl: crate::Reg<sys_gpc_mfpl::SYS_GPC_MFPL_SPEC>,
    #[doc = "0x44 - GPIOC High Byte Multiple Function Control Register"]
    pub sys_gpc_mfph: crate::Reg<sys_gpc_mfph::SYS_GPC_MFPH_SPEC>,
    #[doc = "0x48 - GPIOD Low Byte Multiple Function Control Register"]
    pub sys_gpd_mfpl: crate::Reg<sys_gpd_mfpl::SYS_GPD_MFPL_SPEC>,
    #[doc = "0x4c - GPIOD High Byte Multiple Function Control Register"]
    pub sys_gpd_mfph: crate::Reg<sys_gpd_mfph::SYS_GPD_MFPH_SPEC>,
    #[doc = "0x50 - GPIOE Low Byte Multiple Function Control Register"]
    pub sys_gpe_mfpl: crate::Reg<sys_gpe_mfpl::SYS_GPE_MFPL_SPEC>,
    #[doc = "0x54 - GPIOE High Byte Multiple Function Control Register"]
    pub sys_gpe_mfph: crate::Reg<sys_gpe_mfph::SYS_GPE_MFPH_SPEC>,
    #[doc = "0x58 - GPIOF Low Byte Multiple Function Control Register"]
    pub sys_gpf_mfpl: crate::Reg<sys_gpf_mfpl::SYS_GPF_MFPL_SPEC>,
    #[doc = "0x5c - GPIOF High Byte Multiple Function Control Register"]
    pub sys_gpf_mfph: crate::Reg<sys_gpf_mfph::SYS_GPF_MFPH_SPEC>,
    #[doc = "0x60 - GPIOG Low Byte Multiple Function Control Register"]
    pub sys_gpg_mfpl: crate::Reg<sys_gpg_mfpl::SYS_GPG_MFPL_SPEC>,
    #[doc = "0x64 - GPIOG High Byte Multiple Function Control Register"]
    pub sys_gpg_mfph: crate::Reg<sys_gpg_mfph::SYS_GPG_MFPH_SPEC>,
    #[doc = "0x68 - GPIOH Low Byte Multiple Function Control Register"]
    pub sys_gph_mfpl: crate::Reg<sys_gph_mfpl::SYS_GPH_MFPL_SPEC>,
    #[doc = "0x6c - GPIOH High Byte Multiple Function Control Register"]
    pub sys_gph_mfph: crate::Reg<sys_gph_mfph::SYS_GPH_MFPH_SPEC>,
    _reserved23: [u8; 0x50],
    #[doc = "0xc0 - Modulation Control Register"]
    pub sys_modctl: crate::Reg<sys_modctl::SYS_MODCTL_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0xd0 - System SRAM BIST Test Control Register"]
    pub sys_sram_bistctl: crate::Reg<sys_sram_bistctl::SYS_SRAM_BISTCTL_SPEC>,
    #[doc = "0xd4 - System SRAM BIST Test Status Register"]
    pub sys_sram_biststs: crate::Reg<sys_sram_biststs::SYS_SRAM_BISTSTS_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xdc - System SRAM Interrupt Enable Control Register"]
    pub sys_sram_intctl: crate::Reg<sys_sram_intctl::SYS_SRAM_INTCTL_SPEC>,
    #[doc = "0xe0 - System SRAM Parity Error Status Register"]
    pub sys_sram_status: crate::Reg<sys_sram_status::SYS_SRAM_STATUS_SPEC>,
    #[doc = "0xe4 - System SRAM Parity Check Error Address Register"]
    pub sys_sram_erraddr: crate::Reg<sys_sram_erraddr::SYS_SRAM_ERRADDR_SPEC>,
    _reserved29: [u8; 0x08],
    #[doc = "0xf0 - HIRC Trim Control Register"]
    pub sys_hirctrimctl: crate::Reg<sys_hirctrimctl::SYS_HIRCTRIMCTL_SPEC>,
    #[doc = "0xf4 - HIRC Trim Interrupt Enable Register"]
    pub sys_hirctrimien: crate::Reg<sys_hirctrimien::SYS_HIRCTRIMIEN_SPEC>,
    #[doc = "0xf8 - HIRC Trim Interrupt Status Register"]
    pub sys_hirctrimsts: crate::Reg<sys_hirctrimsts::SYS_HIRCTRIMSTS_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x100 - Register Lock Control Register"]
    pub sys_reglctl: crate::Reg<sys_reglctl::SYS_REGLCTL_SPEC>,
    _reserved33: [u8; 0xe8],
    #[doc = "0x1ec - Analog POR Disable Control Register"]
    pub sys_pordisan: crate::Reg<sys_pordisan::SYS_PORDISAN_SPEC>,
}
#[doc = "SYS_PDID register accessor: an alias for `Reg<SYS_PDID_SPEC>`"]
pub type SYS_PDID = crate::Reg<sys_pdid::SYS_PDID_SPEC>;
#[doc = "Part Device Identification Number Register"]
pub mod sys_pdid;
#[doc = "SYS_RSTSTS register accessor: an alias for `Reg<SYS_RSTSTS_SPEC>`"]
pub type SYS_RSTSTS = crate::Reg<sys_rststs::SYS_RSTSTS_SPEC>;
#[doc = "System Reset Status Register"]
pub mod sys_rststs;
#[doc = "SYS_IPRST0 register accessor: an alias for `Reg<SYS_IPRST0_SPEC>`"]
pub type SYS_IPRST0 = crate::Reg<sys_iprst0::SYS_IPRST0_SPEC>;
#[doc = "Peripheral Reset Control Register 0"]
pub mod sys_iprst0;
#[doc = "SYS_IPRST1 register accessor: an alias for `Reg<SYS_IPRST1_SPEC>`"]
pub type SYS_IPRST1 = crate::Reg<sys_iprst1::SYS_IPRST1_SPEC>;
#[doc = "Peripheral Reset Control Register 1"]
pub mod sys_iprst1;
#[doc = "SYS_IPRST2 register accessor: an alias for `Reg<SYS_IPRST2_SPEC>`"]
pub type SYS_IPRST2 = crate::Reg<sys_iprst2::SYS_IPRST2_SPEC>;
#[doc = "Peripheral Reset Control Register 2"]
pub mod sys_iprst2;
#[doc = "SYS_BODCTL register accessor: an alias for `Reg<SYS_BODCTL_SPEC>`"]
pub type SYS_BODCTL = crate::Reg<sys_bodctl::SYS_BODCTL_SPEC>;
#[doc = "Brown-out Detector Control Register"]
pub mod sys_bodctl;
#[doc = "SYS_PORCTL register accessor: an alias for `Reg<SYS_PORCTL_SPEC>`"]
pub type SYS_PORCTL = crate::Reg<sys_porctl::SYS_PORCTL_SPEC>;
#[doc = "Power-On-reset Controller Register"]
pub mod sys_porctl;
#[doc = "SYS_GPA_MFPL register accessor: an alias for `Reg<SYS_GPA_MFPL_SPEC>`"]
pub type SYS_GPA_MFPL = crate::Reg<sys_gpa_mfpl::SYS_GPA_MFPL_SPEC>;
#[doc = "GPIOA Low Byte Multiple Function Control Register"]
pub mod sys_gpa_mfpl;
#[doc = "SYS_GPA_MFPH register accessor: an alias for `Reg<SYS_GPA_MFPH_SPEC>`"]
pub type SYS_GPA_MFPH = crate::Reg<sys_gpa_mfph::SYS_GPA_MFPH_SPEC>;
#[doc = "GPIOA High Byte Multiple Function Control Register"]
pub mod sys_gpa_mfph;
#[doc = "SYS_GPB_MFPL register accessor: an alias for `Reg<SYS_GPB_MFPL_SPEC>`"]
pub type SYS_GPB_MFPL = crate::Reg<sys_gpb_mfpl::SYS_GPB_MFPL_SPEC>;
#[doc = "GPIOB Low Byte Multiple Function Control Register"]
pub mod sys_gpb_mfpl;
#[doc = "SYS_GPB_MFPH register accessor: an alias for `Reg<SYS_GPB_MFPH_SPEC>`"]
pub type SYS_GPB_MFPH = crate::Reg<sys_gpb_mfph::SYS_GPB_MFPH_SPEC>;
#[doc = "GPIOB High Byte Multiple Function Control Register"]
pub mod sys_gpb_mfph;
#[doc = "SYS_GPC_MFPL register accessor: an alias for `Reg<SYS_GPC_MFPL_SPEC>`"]
pub type SYS_GPC_MFPL = crate::Reg<sys_gpc_mfpl::SYS_GPC_MFPL_SPEC>;
#[doc = "GPIOC Low Byte Multiple Function Control Register"]
pub mod sys_gpc_mfpl;
#[doc = "SYS_GPC_MFPH register accessor: an alias for `Reg<SYS_GPC_MFPH_SPEC>`"]
pub type SYS_GPC_MFPH = crate::Reg<sys_gpc_mfph::SYS_GPC_MFPH_SPEC>;
#[doc = "GPIOC High Byte Multiple Function Control Register"]
pub mod sys_gpc_mfph;
#[doc = "SYS_GPD_MFPL register accessor: an alias for `Reg<SYS_GPD_MFPL_SPEC>`"]
pub type SYS_GPD_MFPL = crate::Reg<sys_gpd_mfpl::SYS_GPD_MFPL_SPEC>;
#[doc = "GPIOD Low Byte Multiple Function Control Register"]
pub mod sys_gpd_mfpl;
#[doc = "SYS_GPD_MFPH register accessor: an alias for `Reg<SYS_GPD_MFPH_SPEC>`"]
pub type SYS_GPD_MFPH = crate::Reg<sys_gpd_mfph::SYS_GPD_MFPH_SPEC>;
#[doc = "GPIOD High Byte Multiple Function Control Register"]
pub mod sys_gpd_mfph;
#[doc = "SYS_GPE_MFPL register accessor: an alias for `Reg<SYS_GPE_MFPL_SPEC>`"]
pub type SYS_GPE_MFPL = crate::Reg<sys_gpe_mfpl::SYS_GPE_MFPL_SPEC>;
#[doc = "GPIOE Low Byte Multiple Function Control Register"]
pub mod sys_gpe_mfpl;
#[doc = "SYS_GPE_MFPH register accessor: an alias for `Reg<SYS_GPE_MFPH_SPEC>`"]
pub type SYS_GPE_MFPH = crate::Reg<sys_gpe_mfph::SYS_GPE_MFPH_SPEC>;
#[doc = "GPIOE High Byte Multiple Function Control Register"]
pub mod sys_gpe_mfph;
#[doc = "SYS_GPF_MFPL register accessor: an alias for `Reg<SYS_GPF_MFPL_SPEC>`"]
pub type SYS_GPF_MFPL = crate::Reg<sys_gpf_mfpl::SYS_GPF_MFPL_SPEC>;
#[doc = "GPIOF Low Byte Multiple Function Control Register"]
pub mod sys_gpf_mfpl;
#[doc = "SYS_GPF_MFPH register accessor: an alias for `Reg<SYS_GPF_MFPH_SPEC>`"]
pub type SYS_GPF_MFPH = crate::Reg<sys_gpf_mfph::SYS_GPF_MFPH_SPEC>;
#[doc = "GPIOF High Byte Multiple Function Control Register"]
pub mod sys_gpf_mfph;
#[doc = "SYS_GPG_MFPL register accessor: an alias for `Reg<SYS_GPG_MFPL_SPEC>`"]
pub type SYS_GPG_MFPL = crate::Reg<sys_gpg_mfpl::SYS_GPG_MFPL_SPEC>;
#[doc = "GPIOG Low Byte Multiple Function Control Register"]
pub mod sys_gpg_mfpl;
#[doc = "SYS_GPG_MFPH register accessor: an alias for `Reg<SYS_GPG_MFPH_SPEC>`"]
pub type SYS_GPG_MFPH = crate::Reg<sys_gpg_mfph::SYS_GPG_MFPH_SPEC>;
#[doc = "GPIOG High Byte Multiple Function Control Register"]
pub mod sys_gpg_mfph;
#[doc = "SYS_GPH_MFPL register accessor: an alias for `Reg<SYS_GPH_MFPL_SPEC>`"]
pub type SYS_GPH_MFPL = crate::Reg<sys_gph_mfpl::SYS_GPH_MFPL_SPEC>;
#[doc = "GPIOH Low Byte Multiple Function Control Register"]
pub mod sys_gph_mfpl;
#[doc = "SYS_GPH_MFPH register accessor: an alias for `Reg<SYS_GPH_MFPH_SPEC>`"]
pub type SYS_GPH_MFPH = crate::Reg<sys_gph_mfph::SYS_GPH_MFPH_SPEC>;
#[doc = "GPIOH High Byte Multiple Function Control Register"]
pub mod sys_gph_mfph;
#[doc = "SYS_MODCTL register accessor: an alias for `Reg<SYS_MODCTL_SPEC>`"]
pub type SYS_MODCTL = crate::Reg<sys_modctl::SYS_MODCTL_SPEC>;
#[doc = "Modulation Control Register"]
pub mod sys_modctl;
#[doc = "SYS_SRAM_BISTCTL register accessor: an alias for `Reg<SYS_SRAM_BISTCTL_SPEC>`"]
pub type SYS_SRAM_BISTCTL = crate::Reg<sys_sram_bistctl::SYS_SRAM_BISTCTL_SPEC>;
#[doc = "System SRAM BIST Test Control Register"]
pub mod sys_sram_bistctl;
#[doc = "SYS_SRAM_BISTSTS register accessor: an alias for `Reg<SYS_SRAM_BISTSTS_SPEC>`"]
pub type SYS_SRAM_BISTSTS = crate::Reg<sys_sram_biststs::SYS_SRAM_BISTSTS_SPEC>;
#[doc = "System SRAM BIST Test Status Register"]
pub mod sys_sram_biststs;
#[doc = "SYS_SRAM_INTCTL register accessor: an alias for `Reg<SYS_SRAM_INTCTL_SPEC>`"]
pub type SYS_SRAM_INTCTL = crate::Reg<sys_sram_intctl::SYS_SRAM_INTCTL_SPEC>;
#[doc = "System SRAM Interrupt Enable Control Register"]
pub mod sys_sram_intctl;
#[doc = "SYS_SRAM_STATUS register accessor: an alias for `Reg<SYS_SRAM_STATUS_SPEC>`"]
pub type SYS_SRAM_STATUS = crate::Reg<sys_sram_status::SYS_SRAM_STATUS_SPEC>;
#[doc = "System SRAM Parity Error Status Register"]
pub mod sys_sram_status;
#[doc = "SYS_SRAM_ERRADDR register accessor: an alias for `Reg<SYS_SRAM_ERRADDR_SPEC>`"]
pub type SYS_SRAM_ERRADDR = crate::Reg<sys_sram_erraddr::SYS_SRAM_ERRADDR_SPEC>;
#[doc = "System SRAM Parity Check Error Address Register"]
pub mod sys_sram_erraddr;
#[doc = "SYS_HIRCTRIMCTL register accessor: an alias for `Reg<SYS_HIRCTRIMCTL_SPEC>`"]
pub type SYS_HIRCTRIMCTL = crate::Reg<sys_hirctrimctl::SYS_HIRCTRIMCTL_SPEC>;
#[doc = "HIRC Trim Control Register"]
pub mod sys_hirctrimctl;
#[doc = "SYS_HIRCTRIMIEN register accessor: an alias for `Reg<SYS_HIRCTRIMIEN_SPEC>`"]
pub type SYS_HIRCTRIMIEN = crate::Reg<sys_hirctrimien::SYS_HIRCTRIMIEN_SPEC>;
#[doc = "HIRC Trim Interrupt Enable Register"]
pub mod sys_hirctrimien;
#[doc = "SYS_HIRCTRIMSTS register accessor: an alias for `Reg<SYS_HIRCTRIMSTS_SPEC>`"]
pub type SYS_HIRCTRIMSTS = crate::Reg<sys_hirctrimsts::SYS_HIRCTRIMSTS_SPEC>;
#[doc = "HIRC Trim Interrupt Status Register"]
pub mod sys_hirctrimsts;
#[doc = "SYS_REGLCTL register accessor: an alias for `Reg<SYS_REGLCTL_SPEC>`"]
pub type SYS_REGLCTL = crate::Reg<sys_reglctl::SYS_REGLCTL_SPEC>;
#[doc = "Register Lock Control Register"]
pub mod sys_reglctl;
#[doc = "SYS_PORDISAN register accessor: an alias for `Reg<SYS_PORDISAN_SPEC>`"]
pub type SYS_PORDISAN = crate::Reg<sys_pordisan::SYS_PORDISAN_SPEC>;
#[doc = "Analog POR Disable Control Register"]
pub mod sys_pordisan;
