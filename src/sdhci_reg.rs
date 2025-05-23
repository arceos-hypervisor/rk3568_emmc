/// 寄存器 外设封装
pub struct Reg {
    base_addr: u64, // 寄存器基地址
}

impl Reg {
    /// 创建实例（无 unsafe 标记，因地址由调用方保证）
    pub const fn new(base_addr: u64) -> Self {
        Self { base_addr }
    }

    /// 安全写入 MMIO 寄存器
    fn write_reg(&self, addr: u64, value: u32) {
        unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
    }

    /// 安全读取 MMIO 寄存器
    fn read_reg(&self, addr: u64) -> u32 {
        unsafe { core::ptr::read_volatile(addr as *const u32) }
    }

    /// 安全写入 MMIO 寄存器
    fn write_reg16(&self, addr: u64, value: u16) {
        unsafe { core::ptr::write_volatile(addr as *mut u16, value) }
    }

    /// 安全读取 MMIO 寄存器
    fn read_reg16(&self, addr: u64) -> u16 {
        unsafe { core::ptr::read_volatile(addr as *const u16) }
    }

    /// 安全写入 MMIO 寄存器
    fn write_reg8(&self, addr: u64, value: u8) {
        unsafe { core::ptr::write_volatile(addr as *mut u8, value) }
    }

    /// 安全读取 MMIO 寄存器
    fn read_reg8(&self, addr: u64) -> u8 {
        unsafe { core::ptr::read_volatile(addr as *const u8) }
    }
}

/// This module contains the offset position of the `EMMC_ARGUMENT` register and the definitions of its individual bits.
/// The `EMMC_ARGUMENT` register is a 32-bit read-write register that contains the command argument.
pub mod emmc_argument_bits {
    /// the offset of the `EMMC_ADMA_ID` register from the base address of the SDHCI controller.
    pub const EMMC_ARGUMENT_OFFSET: u64 = 0x08;
    /// Command Argument
    pub const EMMC_ARGUMENT_POS: u32 = 0;
    pub const EMMC_ARGUMENT_MASK: u32 = 0xffffffff << EMMC_ARGUMENT_POS;
    pub const EMMC_ARGUMENT: u32 = EMMC_ARGUMENT_MASK;
}

/// This module implements read and write operations for the `EMMC_ARGUMENT` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_argument_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_ARGUMENT` register.
    ///
    /// These bits specify the SD/eMMC command argument that is 
    /// specified in bits 39-8 of the Command format.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_argument(&self) -> u32 {
        let addr = self.base_addr + emmc_argument_bits::EMMC_ARGUMENT_OFFSET;
        self.read_reg(addr)
    }

    /// Set the entire value of the `EMMC_ARGUMENT` register.
    ///
    /// These bits specify the SD/eMMC command argument that is 
    /// specified in bits 39-8 of the Command format.
    /// 
    /// # Arguments
    /// 
    /// - `arg` - The value to be written to the register.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_argument(&self, arg: u32) {
        let addr = self.base_addr + emmc_argument_bits::EMMC_ARGUMENT_OFFSET;
        self.write_reg(addr, arg);
    }

}

/// This module contains the offset position of the `EMMC_CMD` register and the definitions of its individual bits.
/// The `EMMC_CMD` register is a 32-bit read-write register that contains the command.
pub mod emmc_cmd_bits {
    /// the offset of the `EMMC_CMD` register from the base address of the SDHCI controller.
    pub const EMMC_CMD_OFFSET: u64 = 0x0e;
    /// Response Type Select
    pub const EMMC_RESP_TYPE_POS: u16 = 0;
    pub const EMMC_RESP_TYPE_MASK: u16 = 0x03 << EMMC_RESP_TYPE_POS;
    pub const EMMC_RESP_TYPE: u16 = EMMC_RESP_TYPE_MASK;
    pub const EMMC_RESP_TYPE_NONE: u16 = 0x00 << EMMC_RESP_TYPE_POS;
    pub const EMMC_RESP_TYPE_LEN_136: u16 = 0x01 << EMMC_RESP_TYPE_POS;
    pub const EMMC_RESP_TYPE_LEN_48: u16 = 0x02 << EMMC_RESP_TYPE_POS;
    pub const EMMC_RESP_TYPE_LEN_48_CHECK: u16 = 0x03 << EMMC_RESP_TYPE_POS;
    /// Sub Command Flag
    pub const EMMC_SUB_CMD_POS: u16 = 2;
    pub const EMMC_SUB_CMD_MASK: u16 = 0x01 << EMMC_SUB_CMD_POS;
    pub const EMMC_SUB_CMD: u16 = EMMC_SUB_CMD_MASK;
    /// Command CRC Check Enable
    pub const EMMC_CMD_CRC_CHK_POS: u16 = 3;
    pub const EMMC_CMD_CRC_CHK_MASK: u16 = 0x01 << EMMC_CMD_CRC_CHK_POS;
    pub const EMMC_CMD_CRC_CHK: u16 = EMMC_CMD_CRC_CHK_MASK;
    /// Command Index Check Enable
    pub const EMMC_CMD_IDX_CHK_POS: u16 = 4;
    pub const EMMC_CMD_IDX_CHK_MASK: u16 = 0x01 << EMMC_CMD_IDX_CHK_POS;
    pub const EMMC_CMD_IDX_CHK: u16 = EMMC_CMD_IDX_CHK_MASK;
    /// Data Present Select
    pub const EMMC_DATA_PRESENT_POS: u16 = 5;
    pub const EMMC_DATA_PRESENT_MASK: u16 = 0x01 << EMMC_DATA_PRESENT_POS;
    pub const EMMC_DATA_PRESENT: u16 = EMMC_DATA_PRESENT_MASK;
    /// Command Type
    pub const EMMC_CMD_TYPE_POS: u16 = 6;
    pub const EMMC_CMD_TYPE_MASK: u16 = 0x03 << EMMC_CMD_TYPE_POS;
    pub const EMMC_CMD_TYPE: u16 = EMMC_CMD_TYPE_MASK;
    pub const EMMC_CMD_TYPE_NORMAL: u16 = 0x00 << EMMC_CMD_TYPE_POS;
    pub const EMMC_CMD_TYPE_SUSPEND: u16 = 0x01 << EMMC_CMD_TYPE_POS;
    pub const EMMC_CMD_TYPE_RESUME: u16 = 0x02 << EMMC_CMD_TYPE_POS;
    pub const EMMC_CMD_TYPE_ABORT: u16 = 0x03 << EMMC_CMD_TYPE_POS;
    /// Command Index
    pub const EMMC_CMD_INDEX_POS: u16 = 8;
    pub const EMMC_CMD_INDEX_MASK: u16 = 0x3f << EMMC_CMD_INDEX_POS;
    pub const EMMC_CMD_INDEX: u16 = EMMC_CMD_INDEX_MASK;
}

/// This module implements read and write operations for the `EMMC_CMD` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_cmd_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_CMD` register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_cmd(&self) -> u16 {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        self.read_reg16(addr)
    }

    /// Set the entire value of the `EMMC_CMD` register.
    ///
    /// # Arguments
    /// 
    /// - `cmd` - The value to be written to the register. It is a combination of individual bits defined in `emmc_cmd_bits`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_cmd(&self, cmd: u16) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        self.write_reg16(addr, cmd);
    }

    /// Set the response type.
    ///
    /// # Arguments
    /// 
    /// - `resp_type` - The value to be written to the register. It is one of the following value defined in `emmc_cmd_bits`.
    ///     - EMMC_RESP_TYPE_NONE
    ///     - EMMC_RESP_TYPE_LEN_136
    ///     - EMMC_RESP_TYPE_LEN_48
    ///     - EMMC_RESP_TYPE_LEN_48_CHECK
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_resp_type(&self, resp_type: u16) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, (value & emmc_cmd_bits::EMMC_RESP_TYPE_MASK) | resp_type);
    }

    /// Enable the sub command. By default, the main command is used.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_sub_cmd(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_cmd_bits::EMMC_SUB_CMD);
    }

    /// Disable the sub command. The main command is used.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_sub_cmd(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_cmd_bits::EMMC_SUB_CMD);
    }

    /// Enable the command crc check.
    ///
    /// This bit enables the Host Controller to check the CRC field in the response. 
    /// If an error is detected, it is reported as a Command CRC error. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// 1. CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. 
    /// 2. For the tuning command, this bit must always be set to 1 to enable the CRC check.
    pub fn emmc_enable_cmd_crc_check(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_cmd_bits::EMMC_CMD_CRC_CHK);
    }

    /// Disable the command crc check.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_cmd_crc_check(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_cmd_bits::EMMC_CMD_CRC_CHK);
    }

    /// Enable the command index check.
    ///
    /// This bit enables the Host Controller to check the index field in the 
    /// response to verify if it has the same value as the command index. 
    /// If the value is not the same, it is reported as a Command Index error. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// 1. Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. 
    /// 2. For the tuning command, this bit must always be set to enable the index check.
    pub fn emmc_enable_cmd_idx_check(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_cmd_bits::EMMC_CMD_IDX_CHK);
    }

    /// Disable the command index check.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_cmd_idx_check(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_cmd_bits::EMMC_CMD_IDX_CHK);
    }

    /// Enable data present.
    ///
    /// This bit is set to 1 to indicate that data is present and that the 
    /// data is transferred using the DAT line. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// This bit is set to 0 in the following instances: 
    /// 1. Command using the CMD line 
    /// 2. Command with no data transfer but using busy signal on the `DAT[0]` line 
    /// 3. Resume Command
    pub fn emmc_enable_data_present(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_cmd_bits::EMMC_DATA_PRESENT);
    }

    /// Disable data present.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_data_present(&self) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_cmd_bits::EMMC_DATA_PRESENT);
    }

    /// Set the command type
    ///
    /// # Arguments
    /// 
    /// - `cmd_type` - The value to be written to the register. It is one of the following values defined in `emmc_cmd_bits`.
    ///     - `EMMC_CMD_TYPE_NORMAL`
    ///     - `EMMC_CMD_TYPE_SUSPEND`
    ///     - `EMMC_CMD_TYPE_RESUME`
    ///     - `EMMC_CMD_TYPE_ABORT`
    /// 
    ///  While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to EMMC_CMD_TYPE_ABORT.
    ///
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_cmd_type(&self, cmd_type: u16) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, (value & emmc_cmd_bits::EMMC_RESP_TYPE_MASK) | cmd_type);
    }

    /// Set the command index
    ///
    /// # Arguments
    /// 
    /// - `cmd_idx` - The value to be written to the register. These bits are set to the command number that is specified in 
    /// bits 45-40 of the Command Format.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_cmd_index(&self, cmd_idx: u16) {
        let addr = self.base_addr + emmc_cmd_bits::EMMC_CMD_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, (value & emmc_cmd_bits::EMMC_CMD_INDEX_MASK) | cmd_idx);
    }
}

pub mod emmc_resp01_bits {
    pub const EMMC_RESP01_OFFSET: u64 = 0x10;

    pub const EMMC_RESP01_POS: u32 = 0;
    pub const EMMC_RESP01_MASK: u32 = 0x0ffffffff << EMMC_RESP01_POS;
    pub const EMMC_RESP01: u32 = EMMC_RESP01_MASK;
}

impl Reg {
    pub fn emmc_get_resp01(&self) -> u32 {
        let addr = self.base_addr + emmc_resp01_bits::EMMC_RESP01_OFFSET;
        self.read_reg(addr)
    }
}

pub mod emmc_resp23_bits {
    pub const EMMC_RESP23_OFFSET: u64 = 0x14;

    pub const EMMC_RESP23_POS: u32 = 0;
    pub const EMMC_RESP23_MASK: u32 = 0x0ffffffff << EMMC_RESP23_POS;
    pub const EMMC_RESP23: u32 = EMMC_RESP23_MASK;
}

impl Reg {
    pub fn emmc_get_resp23(&self) -> u32 {
        let addr = self.base_addr + emmc_resp23_bits::EMMC_RESP23_OFFSET;
        self.read_reg(addr)
    }
}

pub mod emmc_resp45_bits {
    pub const EMMC_RESP45_OFFSET: u64 = 0x14;

    pub const EMMC_RESP45_POS: u32 = 0;
    pub const EMMC_RESP45_MASK: u32 = 0x0ffffffff << EMMC_RESP45_POS;
    pub const EMMC_RESP45: u32 = EMMC_RESP45_MASK;
}

impl Reg {
    pub fn emmc_get_resp45(&self) -> u32 {
        let addr = self.base_addr + emmc_resp45_bits::EMMC_RESP45_OFFSET;
        self.read_reg(addr)
    }
}

pub mod emmc_resp67_bits {
    pub const EMMC_RESP67_OFFSET: u64 = 0x14;

    pub const EMMC_RESP67_POS: u32 = 0;
    pub const EMMC_RESP67_MASK: u32 = 0x0ffffffff << EMMC_RESP67_POS;
    pub const EMMC_RESP67: u32 = EMMC_RESP67_MASK;
}

impl Reg {
    pub fn emmc_get_resp67(&self) -> u32 {
        let addr = self.base_addr + emmc_resp67_bits::EMMC_RESP67_OFFSET;
        self.read_reg(addr)
    }
}

pub mod emmc_pstate_bits {
    pub const EMMC_PSTATE_OFFSET: u64 = 0x24;

    pub const EMMC_CMD_INHIBIT_POS: u32 = 0;
    pub const EMMC_CMD_INHIBIT_MASK: u32 = 0x01 << EMMC_CMD_INHIBIT_POS;
    pub const EMMC_CMD_INHIBIT: u32 = EMMC_CMD_INHIBIT_MASK;
    pub const EMMC_CMD_INHIBIT_DATA_POS: u32 = 1;
    pub const EMMC_CMD_INHIBIT_DATA_MASK: u32 = 0x01 << EMMC_CMD_INHIBIT_DATA_POS;
    pub const EMMC_CMD_INHIBIT_DATA: u32 = EMMC_CMD_INHIBIT_DATA_MASK;
    pub const EMMC_DATA_LINE_ACTIVE_POS: u32 = 2;
    pub const EMMC_DATA_LINE_ACTIVE_MASK: u32 = 0x01 << EMMC_DATA_LINE_ACTIVE_POS;
    pub const EMMC_DATA_LINE_ACTIVE: u32 = EMMC_DATA_LINE_ACTIVE_MASK;
    pub const EMMC_DATA_LINE7_4_LEVEL_POS: u32 = 4;
    pub const EMMC_DATA_LINE7_4_LEVEL_MASK: u32 = 0x0f << EMMC_DATA_LINE7_4_LEVEL_POS;
    pub const EMMC_DATA_LINE7_4_LEVEL: u32 = EMMC_DATA_LINE7_4_LEVEL_MASK;

    pub const EMMC_CARD_INSERTED_POS: u32 = 16;
    pub const EMMC_CARD_INSERTED_MASK: u32 = 0x01 << EMMC_CARD_INSERTED_POS;
    pub const EMMC_CARD_INSERTED: u32 = EMMC_CARD_INSERTED_MASK;
    pub const EMMC_CARD_STABLE_POS: u32 = 17;
    pub const EMMC_CARD_STABLE_MASK: u32 = 0x01 << EMMC_CARD_STABLE_POS;
    pub const EMMC_CARD_STABLE: u32 = EMMC_CARD_STABLE_MASK;

    pub const EMMC_DATA_LINE3_0_LEVEL_POS: u32 = 20;
    pub const EMMC_DATA_LINE3_0_LEVEL_MASK: u32 = 0x0f << EMMC_DATA_LINE3_0_LEVEL_POS;
    pub const EMMC_DATA_LINE3_0_LEVEL: u32 = EMMC_DATA_LINE3_0_LEVEL_MASK;
}

impl Reg {
    pub fn emmc_cmd_is_ready(&self) -> bool {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        self.read_reg(addr) & emmc_pstate_bits::EMMC_CMD_INHIBIT == 0
    }

    pub fn emmc_cmd_data_is_ready(&self) -> bool {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        self.read_reg(addr) & emmc_pstate_bits::EMMC_CMD_INHIBIT_DATA == 0
    }

    pub fn emmc_data_line_is_active(&self) -> bool {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        self.read_reg(addr) & emmc_pstate_bits::EMMC_DATA_LINE_ACTIVE == emmc_pstate_bits::EMMC_DATA_LINE_ACTIVE
    }

    pub fn emmc_get_data_line_level(&self) -> u8 {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        let value = self.read_reg(addr);
        ((value & emmc_pstate_bits::EMMC_DATA_LINE7_4_LEVEL) | ((value & emmc_pstate_bits::EMMC_DATA_LINE3_0_LEVEL) >> emmc_pstate_bits::EMMC_DATA_LINE3_0_LEVEL_POS)) as u8
    }

    pub fn emmc_card_is_inserted(&self) -> bool {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        self.read_reg(addr) & emmc_pstate_bits::EMMC_CARD_INSERTED == emmc_pstate_bits::EMMC_CARD_INSERTED
    }

    pub fn emmc_card_is_stable(&self) -> bool {
        let addr = self.base_addr + emmc_pstate_bits::EMMC_PSTATE_OFFSET;
        self.read_reg(addr) & emmc_pstate_bits::EMMC_CARD_STABLE == emmc_pstate_bits::EMMC_CARD_STABLE
    }
}

pub mod emmc_host_ctrl1_bits {
    pub const EMMC_HOST_CTRL1_OFFSET: u64 = 0x28;

    pub const EMMC_DAT_XFER_WIDTH_POS: u8 = 1;
    pub const EMMC_DAT_XFER_WIDTH_MASK: u8 = 0x01 << EMMC_DAT_XFER_WIDTH_POS;
    pub const EMMC_DAT_XFER_WIDTH: u8 = EMMC_DAT_XFER_WIDTH_MASK;
    pub const EMMC_HIGH_SPEED_EN_POS: u8 = 2;
    pub const EMMC_HIGH_SPEED_EN_MASK: u8 = 0x01 << EMMC_HIGH_SPEED_EN_POS;
    pub const EMMC_HIGH_SPEED_EN: u8 = EMMC_HIGH_SPEED_EN_MASK;
    pub const EMMC_DMA_SEL_POS: u8 = 3;
    pub const EMMC_DMA_SEL_MASK: u8 = 0x03 << EMMC_DMA_SEL_POS;
    pub const EMMC_DMA_SEL: u8 = EMMC_DMA_SEL_MASK;
    pub const EMMC_DMA_SEL_SDMA: u8 = 0 << EMMC_DMA_SEL_POS;
    pub const EMMC_DMA_SEL_ADMA2: u8 = 2 << EMMC_DMA_SEL_POS;
    pub const EMMC_DMA_SEL_ADMA2_3: u8 = 3 << EMMC_DMA_SEL_POS;
    pub const EMMC_EXT_DAT_XFER_POS: u8 = 5;
    pub const EMMC_EXT_DAT_XFER_MASK: u8 = 0x01 << EMMC_EXT_DAT_XFER_POS;
    pub const EMMC_EXT_DAT_XFER: u8 = EMMC_EXT_DAT_XFER_MASK;
    // pub const EMMC_CARD_DETECT_TEST_LVL_POS: u8 = 6;
    // pub const EMMC_CARD_DETECT_TEST_LVL_MASK: u8 = 0x01 << EMMC_CARD_DETECT_TEST_LVL_POS;
    // pub const EMMC_CARD_DETECT_TEST_LVL: u8 = EMMC_CARD_DETECT_TEST_LVL_MASK;
    // pub const EMMC_CARD_DETECT_SIG_SEL_POS: u8 = 7;
    // pub const EMMC_CARD_DETECT_SIG_SEL_MASK: u8 = 0x01 << EMMC_CARD_DETECT_SIG_SEL_POS;
    // pub const EMMC_CARD_DETECT_SIG_SEL: u8 = EMMC_CARD_DETECT_SIG_SEL_MASK;
}

impl Reg {
    pub fn emmc_enable_data_xfer_width_1bit(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_host_ctrl1_bits::EMMC_DAT_XFER_WIDTH);
    }

    pub fn emmc_enable_data_xfer_width_4bit(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_host_ctrl1_bits::EMMC_DAT_XFER_WIDTH);
    }

    pub fn emmc_enable_high_speed(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_host_ctrl1_bits::EMMC_HIGH_SPEED_EN);
    }

    pub fn emmc_disable_high_speed(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_host_ctrl1_bits::EMMC_HIGH_SPEED_EN);
    }

    pub fn emmc_select_dma(&self, dma_sel: u8) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, (value & !emmc_host_ctrl1_bits::EMMC_DMA_SEL_MASK) | dma_sel);
    }

    pub fn emmc_enable_ext_data_xfre(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_host_ctrl1_bits::EMMC_EXT_DAT_XFER);
    }

    pub fn emmc_disable_ext_data_xfre(&self) {
        let addr = self.base_addr + emmc_host_ctrl1_bits::EMMC_HOST_CTRL1_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_host_ctrl1_bits::EMMC_EXT_DAT_XFER);
    }

}

/// This module contains the offset position of the `EMMC_PWR_CTRL` register and the definitions of its individual bits.
/// The `EMMC_PWR_CTRL` register is a 8-bit read-write register that contains the power related settings.
pub mod emmc_pwr_ctrl_bits {
    /// the offset of the `EMMC_PWR_CTRL` register from the base address of the SDHCI controller.
    pub const EMMC_PWR_CTRL_OFFSET: u64 = 0x29;
    /// Power on/off
    pub const EMMC_PWR_ON_POS: u8 = 0;
    pub const EMMC_PWR_ON_MASK: u8 = 0x01 << EMMC_PWR_ON_POS;
    pub const EMMC_PWR_ON: u8 = EMMC_PWR_ON_MASK;
}

/// This module implements read and write operations for the `EMMC_PWR_CTRL` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_pwr_ctrl_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_PWR_CTRL` register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00
    pub fn emmc_get_pwr_ctrl(&self) -> u8 {
        let addr = self.base_addr + emmc_pwr_ctrl_bits::EMMC_PWR_CTRL_OFFSET;
        self.read_reg8(addr)
    }

    /// Set the entire value of the `EMMC_PWR_CTRL` register.
    ///
    /// # Arguments
    /// 
    /// - `pwr_ctrl` - The value to be written to the register.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_pwr_ctrl(&self, pwr_ctrl: u8) {
        let addr = self.base_addr + emmc_pwr_ctrl_bits::EMMC_PWR_CTRL_OFFSET;
        self.write_reg8(addr, pwr_ctrl);
    }

    /// Power on
    ///
    /// If this bit is cleared, the Host Controller stops the SD Clock by 
    /// clearing the SD_CLK_IN bit in the CLK_CTRL_R register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_pwr_on(&self) {
        let addr = self.base_addr + emmc_pwr_ctrl_bits::EMMC_PWR_CTRL_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_pwr_ctrl_bits::EMMC_PWR_ON);
    }

    /// Power off
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_pwr_off(&self) {
        let addr = self.base_addr + emmc_pwr_ctrl_bits::EMMC_PWR_CTRL_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_pwr_ctrl_bits::EMMC_PWR_ON);
    }
}

/// This module contains the offset position of the `EMMC_CLK_CTRL` register and the definitions of its individual bits.
/// The `EMMC_CLK_CTRL` register is a 16-bit read-write register that contains the clock related settings.
pub mod emmc_clk_ctrl_bits {
    pub const EMMC_CLK_CTRL_OFFSET: u64 = 0x2c;

    pub const EMMC_INTERNAL_CLK_EN_POS: u16 = 0;
    pub const EMMC_INTERNAL_CLK_EN_MASK: u16 = 0x01 << EMMC_INTERNAL_CLK_EN_POS;
    pub const EMMC_INTERNAL_CLK_EN: u16 = EMMC_INTERNAL_CLK_EN_MASK;
    pub const EMMC_INTERNAL_CLK_STABLE_POS: u16 = 1;
    pub const EMMC_INTERNAL_CLK_STABLE_MASK: u16 = 0x01 << EMMC_INTERNAL_CLK_STABLE_POS;
    pub const EMMC_INTERNAL_CLK_STABLE: u16 = EMMC_INTERNAL_CLK_STABLE_MASK;
    pub const EMMC_SD_CLK_EN_POS: u16 = 2;
    pub const EMMC_SD_CLK_EN_MASK: u16 = 0x01 << EMMC_SD_CLK_EN_POS;
    pub const EMMC_SD_CLK_EN: u16 = EMMC_SD_CLK_EN_MASK;
    pub const EMMC_CLK_GEN_TYPE_POS: u16 = 5;
    pub const EMMC_CLK_GEN_TYPE_MASK: u16 = 0x01 << EMMC_CLK_GEN_TYPE_POS;
    pub const EMMC_CLK_GEN_TYPE: u16 = EMMC_CLK_GEN_TYPE_MASK;
    pub const EMMC_CLK_GEN_TYPE_PROG: u16 = 0x01 << EMMC_CLK_GEN_TYPE_POS;
    pub const EMMC_CLK_GEN_TYPE_DIV: u16 = 0x00;
    pub const EMMC_UPPER_FREQ_POS: u16 = 6;
    pub const EMMC_UPPER_FREQ_MASK: u16 = 0x03 << EMMC_UPPER_FREQ_POS;
    pub const EMMC_UPPER_FREQ: u16 = EMMC_UPPER_FREQ_MASK;
    pub const EMMC_FREQ_POS: u16 = 8;
    pub const EMMC_FREQ_MASK: u16 = 0xff << EMMC_FREQ_POS;
    pub const EMMC_FREQ: u16 = EMMC_FREQ_MASK;
}

/// This module implements read and write operations for the `EMMC_CLK_CTRL` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_clk_ctrl_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_CLK_CTRL` register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x0000
    pub fn emmc_get_clk_ctrl(&self) -> u16 {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        self.read_reg16(addr)
    }

    /// Set the entire value of the `EMMC_CLK_CTRL` register.
    ///
    /// # Arguments
    /// 
    /// - `pwr_ctrl` - The value to be written to the register. It is a combination of individual bits defined in `emmc_clk_ctrl_bits`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_clk_ctrl(&self, clk_ctrl: u16) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        self.write_reg16(addr, clk_ctrl);
    }

    /// Enable the internal clock
    ///
    /// The Host Controller must stop its internal clock to enter a very 
    /// low power state. However, registers can still be read and written 
    /// to. The value is reflected on the intclk_en signal. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// If this bit is not used to control the internal clock (base clock and 
    /// master clock), it is recommended to set this bit to 1 .
    pub fn emmc_enable_internal_clk(&self) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_clk_ctrl_bits::EMMC_INTERNAL_CLK_EN);
    }

    /// Disable the internal clock
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_internal_clk(&self) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_clk_ctrl_bits::EMMC_INTERNAL_CLK_EN);
    }

    /// Check the internal clock is stable or not
    ///
    /// This bit reflects the synchronized value of the 
    /// intclk_stable signal after the Internal Clock Enable bit is set to 1 
    /// and also reflects the synchronized value of the card_clk_stable 
    /// signal after the PLL Enable bit is set to 1.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the internal clock is stable
    /// - false if the internal clock is not stable
    pub fn emmc_internal_clk_is_stable(&self) -> bool {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        (self.read_reg16(addr) & emmc_clk_ctrl_bits::EMMC_INTERNAL_CLK_STABLE) == emmc_clk_ctrl_bits::EMMC_INTERNAL_CLK_STABLE
    }

    /// Enable the SD/eMMC clock
    ///
    /// This bit stops the SDCLK or RCLK when set to 0. The 
    /// SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_sd_clk(&self) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value | emmc_clk_ctrl_bits::EMMC_SD_CLK_EN);
    }

    /// Disable the SD/eMMC clock
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_sd_clk(&self) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !emmc_clk_ctrl_bits::EMMC_SD_CLK_EN);
    }

    /// Set the clock generator
    ///
    /// # Arguments
    /// 
    /// - `clk_gen_type` - The clock generator type. It can be either `EMMC_CLK_GEN_TYPE_PROG` or `EMMC_CLK_GEN_TYPE_DIV`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_clk_gen_type(&self, clk_gen_type: u16) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, (value & !emmc_clk_ctrl_bits::EMMC_CLK_GEN_TYPE_MASK) | clk_gen_type);
    }

    /// Set the frequency
    ///
    /// # Arguments
    /// 
    /// - `freq` - the frequency to be set. It is a 9-bit value.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_freq(&self, freq: u16) {
        let addr = self.base_addr + emmc_clk_ctrl_bits::EMMC_CLK_CTRL_OFFSET;
        let value = self.read_reg16(addr);
        self.write_reg16(addr, value & !(emmc_clk_ctrl_bits::EMMC_FREQ_MASK | emmc_clk_ctrl_bits::EMMC_UPPER_FREQ_MASK) 
                                | ((freq & 0xff) << emmc_clk_ctrl_bits::EMMC_FREQ_POS) 
                                | (((freq & 0x300) >> emmc_clk_ctrl_bits::EMMC_FREQ_POS) << emmc_clk_ctrl_bits::EMMC_UPPER_FREQ_POS));
    }
}

/// This module contains the offset position of the `EMMC_SW_RST` register and the definitions of its individual bits.
/// The `EMMC_SW_RST` register is a 8-bit read-write register that contains the reset related settings.
pub mod emmc_sw_rst_bits {
    pub const EMMC_SW_RST_OFFSET: u64 = 0x2F;

    pub const EMMC_SW_RST_ALL_POS: u8 = 0;
    pub const EMMC_SW_RST_ALL_MASK: u8 = 0x01 << EMMC_SW_RST_ALL_POS;
    pub const EMMC_SW_RST_ALL: u8 = EMMC_SW_RST_ALL_MASK;
    pub const EMMC_SW_RST_CMD_POS: u8 = 1;
    pub const EMMC_SW_RST_CMD_MASK: u8 = 0x01 << EMMC_SW_RST_CMD_POS;
    pub const EMMC_SW_RST_CMD: u8 = EMMC_SW_RST_CMD_MASK;
    pub const EMMC_SW_RST_DATA_POS: u8 = 2;
    pub const EMMC_SW_RST_DATA_MASK: u8 = 0x01 << EMMC_SW_RST_DATA_POS;
    pub const EMMC_SW_RST_DATA: u8 = EMMC_SW_RST_DATA_MASK;
}

/// This module implements read and write operations for the `EMMC_SW_RST` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_sw_rst_bits` module.
impl Reg {
    /// Do software reset for all
    ///
    /// This reset affects the entire Host Controller except for the card 
    /// detection circuit. During its initialization, the Host Driver sets this 
    /// bit to 1 to reset the Host Controller. 
    /// 
    /// All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver 
    /// must issue reset command and reinitialize the card.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_reset_all(&self) {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_sw_rst_bits::EMMC_SW_RST_ALL);
    }

    /// Check the software reset is finished or not
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the software reset is finished
    /// - false if the software reset is not finished
    pub fn emmc_reset_all_is_finished(&self) -> bool {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        self.read_reg8(addr) & emmc_sw_rst_bits::EMMC_SW_RST_ALL == 0
    }

    /// Do software reset for command line
    ///
    /// This bit resets only a part of the command circuit to be able to 
    /// issue a command. This reset is effective only for a command 
    /// issuing circuit (including response error statuses related to 
    /// Command Inhibit (CMD) control) and does not affect the data 
    /// transfer circuit. Host Controller can continue data transfer even 
    /// after this reset is executed while handling subcommand-response errors.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// The following registers and bits are cleared by this bit: 
    /// 1. Present State register - Command Inhibit (CMD) bit 
    /// 2. Normal Interrupt Status register - Command Complete bit 
    /// 3. Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit 
    pub fn emmc_reset_cmd(&self) {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_sw_rst_bits::EMMC_SW_RST_CMD);
    }

    /// Check the command line reset is finished or not
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the command line reset is finished
    /// - false if the command line reset is not finished
    pub fn emmc_reset_cmd_is_finished(&self) -> bool {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        self.read_reg8(addr) & emmc_sw_rst_bits::EMMC_SW_RST_CMD == 0
    }

    /// Do software reset for data line
    ///
    /// This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. 
    ///
    /// The following registers and bits are cleared by this bit:
    /// 1. Buffer Data Port register: 
    ///     - Buffer is cleared and initialized
    /// 2. Present state register
    ///     - Buffer Read Enable 
    ///     - Buffer Write Enable 
    ///     - Read Transfer Active 
    ///     - Write Transfer Active 
    ///     - DAT Line Active 
    ///     - Command Inhibit (DAT)
    /// 3. Block Gap Control register: 
    ///     - Continue Request 
    ///     - Stop At Block Gap Request
    /// 4. Normal Interrupt status register: 
    ///     - Buffer Read Ready 
    ///     - Buffer Write Ready 
    ///     - DMA Interrupt 
    ///     - Block Gap Event 
    ///     - Transfer Complete
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    /// 
    /// # Note
    /// 
    /// The following registers and bits are cleared by this bit: 
    /// 1. Present State register - Command Inhibit (CMD) bit 
    /// 2. Normal Interrupt Status register - Command Complete bit 
    /// 3. Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit 
    pub fn emmc_reset_data(&self) {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_sw_rst_bits::EMMC_SW_RST_DATA);
    }

    /// Check the data line reset is finished or not
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the data line reset is finished
    /// - false if the data line reset is not finished
    pub fn emmc_reset_data_is_finished(&self) -> bool {
        let addr = self.base_addr + emmc_sw_rst_bits::EMMC_SW_RST_OFFSET;
        self.read_reg8(addr) & emmc_sw_rst_bits::EMMC_SW_RST_DATA == 0
    }
}

/// This module contains the offset position of the `EMMC_NORMAL_INT_STAT` register and the definitions of its individual bits.
/// The `EMMC_NORMAL_INT_STAT` register is a 16-bit read-write register that contains some interrupt status.
pub mod emmc_normal_int_stat_bits {
    pub const EMMC_NORMAL_INT_STAT_OFFSET: u64 = 0x30;

    pub const EMMC_CMD_COMPLETE_POS: u16 = 0;
    pub const EMMC_CMD_COMPLETE_MASK: u16 = 0x01 << EMMC_CMD_COMPLETE_POS;
    pub const EMMC_CMD_COMPLETE: u16 = EMMC_CMD_COMPLETE_MASK;
    pub const EMMC_XFER_COMPLETE_POS: u16 = 1;
    pub const EMMC_XFER_COMPLETE_MASK: u16 = 0x01 << EMMC_XFER_COMPLETE_POS;
    pub const EMMC_XFER_COMPLETE: u16 = EMMC_XFER_COMPLETE_MASK;
    pub const EMMC_BGAP_EVENT_POS: u16 = 2;
    pub const EMMC_BGAP_EVENT_MASK: u16 = 0x01 << EMMC_BGAP_EVENT_POS;
    pub const EMMC_BGAP_EVENT: u16 = EMMC_BGAP_EVENT_MASK;
    pub const EMMC_DMA_INTERRUPT_POS: u16 = 3;
    pub const EMMC_DMA_INTERRUPT_MASK: u16 = 0x01 << EMMC_DMA_INTERRUPT_POS;
    pub const EMMC_DMA_INTERRUPT: u16 = EMMC_DMA_INTERRUPT_MASK;
    pub const EMMC_BUF_WR_READY_POS: u16 = 4;
    pub const EMMC_BUF_WR_READY_MASK: u16 = 0x01 << EMMC_BUF_WR_READY_POS;
    pub const EMMC_BUF_WR_READY: u16 = EMMC_BUF_WR_READY_MASK;
    pub const EMMC_BUF_RD_READY_POS: u16 = 5;
    pub const EMMC_BUF_RD_READY_MASK: u16 = 0x01 << EMMC_BUF_RD_READY_POS;
    pub const EMMC_BUF_RD_READY: u16 = EMMC_BUF_RD_READY_MASK;
    pub const EMMC_CARD_INSERTION_POS: u16 = 6;
    pub const EMMC_CARD_INSERTION_MASK: u16 = 0x01 << EMMC_CARD_INSERTION_POS;
    pub const EMMC_CARD_INSERTION: u16 = EMMC_CARD_INSERTION_MASK;
    pub const EMMC_CARD_REMOVAL_POS: u16 = 7;
    pub const EMMC_CARD_REMOVAL_MASK: u16 = 0x01 << EMMC_CARD_REMOVAL_POS;
    pub const EMMC_CARD_REMOVAL: u16 = EMMC_CARD_REMOVAL_MASK;
    pub const EMMC_CARD_INTERRUPT_POS: u16 = 8;
    pub const EMMC_CARD_INTERRUPT_MASK: u16 = 0x01 << EMMC_CARD_INTERRUPT_POS;
    pub const EMMC_CARD_INTERRUPT: u16 = EMMC_CARD_INTERRUPT_MASK;
    pub const EMMC_RE_TUNE_EVENT_POS: u16 = 12;
    pub const EMMC_RE_TUNE_EVENT_MASK: u16 = 0x01 << EMMC_RE_TUNE_EVENT_POS;
    pub const EMMC_RE_TUNE_EVENT: u16 = EMMC_RE_TUNE_EVENT_MASK;
    pub const EMMC_FX_EVENT_POS: u16 = 13;
    pub const EMMC_FX_EVENT_MASK: u16 = 0x01 << EMMC_FX_EVENT_POS;
    pub const EMMC_FX_EVENT: u16 = EMMC_FX_EVENT_MASK;
    pub const EMMC_CQE_EVENT_POS: u16 = 14;
    pub const EMMC_CQE_EVENT_MASK: u16 = 0x01 << EMMC_CQE_EVENT_POS;
    pub const EMMC_CQE_EVENT: u16 = EMMC_CQE_EVENT_MASK;
    pub const EMMC_ERROR_INT_POS: u16 = 15;
    pub const EMMC_ERROR_INT_MASK: u16 = 0x01 << EMMC_ERROR_INT_POS;
    pub const EMMC_ERROR_INT: u16 = EMMC_ERROR_INT_MASK;
}

/// This module implements read and write operations for the `EMMC_NORMAL_INT_STAT` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_normal_int_stat_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_NORMAL_INT_STAT` register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00
    pub fn emmc_get_normal_int_stat(&self) -> u16 {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.read_reg16(addr)
    }

    /// Set the entire value of the `EMMC_NORMAL_INT_STAT` register.
    ///
    /// # Arguments
    /// 
    /// - `normal_int_stat` - The value to be written to the register. It is a combination of individual bits defined in `emmc_normal_int_stat_bits`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_normal_int_stat(&self, normal_int_stat: u16) {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.write_reg16(addr, normal_int_stat);
    }

    /// clear all the bits of the `EMMC_NORMAL_INT_STAT` register.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_clear_all_normal_int_flags(&self) {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_normal_int_stat_bits::EMMC_CMD_COMPLETE
                | emmc_normal_int_stat_bits::EMMC_XFER_COMPLETE
                | emmc_normal_int_stat_bits::EMMC_BGAP_EVENT
                | emmc_normal_int_stat_bits::EMMC_DMA_INTERRUPT
                | emmc_normal_int_stat_bits::EMMC_BUF_WR_READY
                | emmc_normal_int_stat_bits::EMMC_BUF_RD_READY
                | emmc_normal_int_stat_bits::EMMC_CARD_INSERTION
                | emmc_normal_int_stat_bits::EMMC_CARD_REMOVAL
                | emmc_normal_int_stat_bits::EMMC_CQE_EVENT;
        self.write_reg16(addr, value);
    }

    /// Check the card interrput flag is actived or not.
    ///
    /// This bit reflects the synchronized value of `DAT[1]` Interrupt Input for SD Mode.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the card interrupt flag is actived.
    /// - false if the card interrupt flag is not actived.
    pub fn emmc_card_interrupt_is_actived(&self) -> bool {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.read_reg16(addr) & emmc_normal_int_stat_bits::EMMC_CARD_INTERRUPT == emmc_normal_int_stat_bits::EMMC_CARD_INTERRUPT
    }

    /// Check the re-tuning event flag is actived or not.
    ///
    /// This bit is set if the Re-Tuning Request changes from 0 to 1. Re-Tuning request is not supported
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the re-tuning event flag is actived.
    /// - false if the re-tuning event flag is not actived.
    pub fn emmc_re_tune_event_is_actived(&self) -> bool {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.read_reg16(addr) & emmc_normal_int_stat_bits::EMMC_RE_TUNE_EVENT == emmc_normal_int_stat_bits::EMMC_RE_TUNE_EVENT
    }

    /// Check the FX event flag is actived or not.
    ///
    /// This status is set when `R[14]` of response register is set to 1 
    /// and Response Type R1/R5 is set to 0 in Transfer Mode register. 
    /// This interrupt is used with response check function. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the FX event flag is actived.
    /// - false if the FX event flag is not actived.
    pub fn emmc_fx_event_is_actived(&self) -> bool {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.read_reg16(addr) & emmc_normal_int_stat_bits::EMMC_FX_EVENT == emmc_normal_int_stat_bits::EMMC_FX_EVENT
    }

    /// Check the error interrupt flag is actived or not.
    ///
    /// If any of the bits in the Error Interrupt Status register are set, then this bit is set. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - true if the error interrupt flag is actived.
    /// - false if the error interrupt flag is not actived.
    pub fn emmc_error_int_is_actived(&self) -> bool {
        let addr = self.base_addr + emmc_normal_int_stat_bits::EMMC_NORMAL_INT_STAT_OFFSET;
        self.read_reg16(addr) & emmc_normal_int_stat_bits::EMMC_ERROR_INT == emmc_normal_int_stat_bits::EMMC_ERROR_INT
    }
}

pub mod emmc_error_int_stat_bits {
    pub const EMMC_ERROR_INT_STAT_OFFSET: u64 = 0x32;

    pub const EMMC_CMD_TOUT_ERR_POS: u16 = 0;
    pub const EMMC_CMD_TOUT_ERR_MASK: u16 = 0x01 << EMMC_CMD_TOUT_ERR_POS;
    pub const EMMC_CMD_TOUT_ERR: u16 = EMMC_CMD_TOUT_ERR_MASK;
    pub const EMMC_CMD_CRC_ERR_POS: u16 = 1;
    pub const EMMC_CMD_CRC_ERR_MASK: u16 = 0x01 << EMMC_CMD_CRC_ERR_POS;
    pub const EMMC_CMD_CRC_ERR: u16 = EMMC_CMD_CRC_ERR_MASK;
    pub const EMMC_CMD_END_BIT_ERR_POS: u16 = 2;
    pub const EMMC_CMD_END_BIT_ERR_MASK: u16 = 0x01 << EMMC_CMD_END_BIT_ERR_POS;
    pub const EMMC_CMD_END_BIT_ERR: u16 = EMMC_CMD_END_BIT_ERR_MASK;
    pub const EMMC_CMD_IDX_ERR_POS: u16 = 3;
    pub const EMMC_CMD_IDX_ERR_MASK: u16 = 0x01 << EMMC_CMD_IDX_ERR_POS;
    pub const EMMC_CMD_IDX_ERR: u16 = EMMC_CMD_IDX_ERR_MASK;
    pub const EMMC_DATA_TOUT_ERR_POS: u16 = 4;
    pub const EMMC_DATA_TOUT_ERR_MASK: u16 = 0x01 << EMMC_DATA_TOUT_ERR_POS;
    pub const EMMC_DATA_TOUT_ERR: u16 = EMMC_DATA_TOUT_ERR_MASK;
    pub const EMMC_DATA_CRC_ERR_POS: u16 = 5;
    pub const EMMC_DATA_CRC_ERR_MASK: u16 = 0x01 << EMMC_DATA_CRC_ERR_POS;
    pub const EMMC_DATA_CRC_ERR: u16 = EMMC_DATA_CRC_ERR_MASK;
    pub const EMMC_DATA_END_BIT_ERR_POS: u16 = 6;
    pub const EMMC_DATA_END_BIT_ERR_MASK: u16 = 0x01 << EMMC_DATA_END_BIT_ERR_POS;
    pub const EMMC_DATA_END_BIT_ERR: u16 = EMMC_DATA_END_BIT_ERR_MASK;
    pub const EMMC_AUTO_CMD_ERR_POS: u16 = 8;
    pub const EMMC_AUTO_CMD_ERR_MASK: u16 = 0x01 << EMMC_AUTO_CMD_ERR_POS;
    pub const EMMC_AUTO_CMD_ERR: u16 = EMMC_AUTO_CMD_ERR_MASK;
    pub const EMMC_ADMA_ERR_POS: u16 = 9;
    pub const EMMC_ADMA_ERR_MASK: u16 = 0x01 << EMMC_ADMA_ERR_POS;
    pub const EMMC_ADMA_ERR: u16 = EMMC_ADMA_ERR_MASK;
    pub const EMMC_TUNING_ERR_POS: u16 = 10;
    pub const EMMC_TUNING_ERR_MASK: u16 = 0x01 << EMMC_TUNING_ERR_POS;
    pub const EMMC_TUNING_ERR: u16 = EMMC_TUNING_ERR_MASK;
    pub const EMMC_RESP_ERR_POS: u16 = 11;
    pub const EMMC_RESP_ERR_MASK: u16 = 0x01 << EMMC_RESP_ERR_POS;
    pub const EMMC_RESP_ERR: u16 = EMMC_RESP_ERR_MASK;
    pub const EMMC_BOOT_ACK_ERR_POS: u16 = 12;
    pub const EMMC_BOOT_ACK_ERR_MASK: u16 = 0x01 << EMMC_BOOT_ACK_ERR_POS;
    pub const EMMC_BOOT_ACK_ERR: u16 = EMMC_BOOT_ACK_ERR_MASK;
}

impl Reg {
    pub fn emmc_clear_all_error_int_flags(&self) {
        let addr = self.base_addr + emmc_error_int_stat_bits::EMMC_ERROR_INT_STAT_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_error_int_stat_bits::EMMC_CMD_TOUT_ERR
                | emmc_error_int_stat_bits::EMMC_CMD_CRC_ERR
                | emmc_error_int_stat_bits::EMMC_CMD_END_BIT_ERR
                | emmc_error_int_stat_bits::EMMC_CMD_IDX_ERR
                | emmc_error_int_stat_bits::EMMC_DATA_TOUT_ERR
                | emmc_error_int_stat_bits::EMMC_DATA_CRC_ERR
                | emmc_error_int_stat_bits::EMMC_DATA_END_BIT_ERR
                | emmc_error_int_stat_bits::EMMC_AUTO_CMD_ERR
                | emmc_error_int_stat_bits::EMMC_ADMA_ERR
                | emmc_error_int_stat_bits::EMMC_TUNING_ERR
                | emmc_error_int_stat_bits::EMMC_RESP_ERR
                | emmc_error_int_stat_bits::EMMC_BOOT_ACK_ERR;
        self.write_reg16(addr, value);
    }
}

pub mod emmc_normal_int_en_bits {
    pub const EMMC_NORMAL_INT_EN_OFFSET: u64 = 0x34;

    pub const EMMC_CMD_COMPLETE_EN_POS: u16 = 0;
    pub const EMMC_CMD_COMPLETE_EN_MASK: u16 = 0x01 << EMMC_CMD_COMPLETE_EN_POS;
    pub const EMMC_CMD_COMPLETE_EN: u16 = EMMC_CMD_COMPLETE_EN_MASK;
    pub const EMMC_XFER_COMPLETE_EN_POS: u16 = 1;
    pub const EMMC_XFER_COMPLETE_EN_MASK: u16 = 0x01 << EMMC_XFER_COMPLETE_EN_POS;
    pub const EMMC_XFER_COMPLETE_EN: u16 = EMMC_XFER_COMPLETE_EN_MASK;
    pub const EMMC_BGAP_EVENT_EN_POS: u16 = 2;
    pub const EMMC_BGAP_EVENT_EN_MASK: u16 = 0x01 << EMMC_BGAP_EVENT_EN_POS;
    pub const EMMC_BGAP_EVENT_EN: u16 = EMMC_BGAP_EVENT_EN_MASK;
    pub const EMMC_DMA_INTERRUPT_EN_POS: u16 = 3;
    pub const EMMC_DMA_INTERRUPT_EN_MASK: u16 = 0x01 << EMMC_DMA_INTERRUPT_EN_POS;
    pub const EMMC_DMA_INTERRUPT_EN: u16 = EMMC_DMA_INTERRUPT_EN_MASK;
    pub const EMMC_BUF_WR_READY_EN_POS: u16 = 4;
    pub const EMMC_BUF_WR_READY_EN_MASK: u16 = 0x01 << EMMC_BUF_WR_READY_EN_POS;
    pub const EMMC_BUF_WR_READY_EN: u16 = EMMC_BUF_WR_READY_EN_MASK;
    pub const EMMC_BUF_RD_READY_EN_POS: u16 = 5;
    pub const EMMC_BUF_RD_READY_EN_MASK: u16 = 0x01 << EMMC_BUF_RD_READY_EN_POS;
    pub const EMMC_BUF_RD_READY_EN: u16 = EMMC_BUF_RD_READY_EN_MASK;
    pub const EMMC_CARD_INSERTION_EN_POS: u16 = 6;
    pub const EMMC_CARD_INSERTION_EN_MASK: u16 = 0x01 << EMMC_CARD_INSERTION_EN_POS;
    pub const EMMC_CARD_INSERTION_EN: u16 = EMMC_CARD_INSERTION_EN_MASK;
    pub const EMMC_CARD_REMOVAL_EN_POS: u16 = 7;
    pub const EMMC_CARD_REMOVAL_EN_MASK: u16 = 0x01 << EMMC_CARD_REMOVAL_EN_POS;
    pub const EMMC_CARD_REMOVAL_EN: u16 = EMMC_CARD_REMOVAL_EN_MASK;
    pub const EMMC_CARD_INTERRUPT_EN_POS: u16 = 8;
    pub const EMMC_CARD_INTERRUPT_EN_MASK: u16 = 0x01 << EMMC_CARD_INTERRUPT_EN_POS;
    pub const EMMC_CARD_INTERRUPT_EN: u16 = EMMC_CARD_INTERRUPT_EN_MASK;
    pub const EMMC_CQE_EVENT_EN_POS: u16 = 14;
    pub const EMMC_CQE_EVENT_EN_MASK: u16 = 0x01 << EMMC_CQE_EVENT_EN_POS;
    pub const EMMC_CQE_EVENT_EN: u16 = EMMC_CQE_EVENT_EN_MASK;
}

impl Reg {
    pub fn emmc_set_normal_int_en(&self, normal_int_en: u16) {
        let addr = self.base_addr + emmc_normal_int_en_bits::EMMC_NORMAL_INT_EN_OFFSET;
        self.write_reg16(addr, normal_int_en);
    }

    pub fn emmc_get_normal_int_en(&self) -> u16{
        let addr = self.base_addr + emmc_normal_int_en_bits::EMMC_NORMAL_INT_EN_OFFSET;
        self.read_reg16(addr)
    }

    pub fn emmc_enable_all_normal_int(&self) {
        let addr = self.base_addr + emmc_normal_int_en_bits::EMMC_NORMAL_INT_EN_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_normal_int_en_bits::EMMC_CMD_COMPLETE_EN
                | emmc_normal_int_en_bits::EMMC_XFER_COMPLETE_EN
                | emmc_normal_int_en_bits::EMMC_BGAP_EVENT_EN
                | emmc_normal_int_en_bits::EMMC_DMA_INTERRUPT_EN
                | emmc_normal_int_en_bits::EMMC_BUF_WR_READY_EN
                | emmc_normal_int_en_bits::EMMC_BUF_RD_READY_EN
                | emmc_normal_int_en_bits::EMMC_CARD_INSERTION_EN
                | emmc_normal_int_en_bits::EMMC_CARD_REMOVAL_EN
                | emmc_normal_int_en_bits::EMMC_CARD_INTERRUPT_EN
                | emmc_normal_int_en_bits::EMMC_CQE_EVENT_EN;
        self.write_reg16(addr, value);
    }

    pub fn emmc_disable_all_normal_int(&self) {
        let addr = self.base_addr + emmc_normal_int_en_bits::EMMC_NORMAL_INT_EN_OFFSET;
        self.write_reg16(addr, 0);
    }
}

pub mod emmc_error_int_en_bits {
    pub const EMMC_ERROR_INT_EN_OFFSET: u64 = 0x36;

    pub const EMMC_CMD_TOUT_EN_POS: u16 = 0;
    pub const EMMC_CMD_TOUT_EN_MASK: u16 = 0x01 << EMMC_CMD_TOUT_EN_POS;
    pub const EMMC_CMD_TOUT_EN: u16 = EMMC_CMD_TOUT_EN_MASK;
    pub const EMMC_CMD_CRC_EN_POS: u16 = 1;
    pub const EMMC_CMD_CRC_EN_MASK: u16 = 0x01 << EMMC_CMD_CRC_EN_POS;
    pub const EMMC_CMD_CRC_EN: u16 = EMMC_CMD_CRC_EN_MASK;
    pub const EMMC_CMD_END_BIT_EN_POS: u16 = 2;
    pub const EMMC_CMD_END_BIT_EN_MASK: u16 = 0x01 << EMMC_CMD_END_BIT_EN_POS;
    pub const EMMC_CMD_END_BIT_EN: u16 = EMMC_CMD_END_BIT_EN_MASK;
    pub const EMMC_CMD_IDX_EN_POS: u16 = 3;
    pub const EMMC_CMD_IDX_EN_MASK: u16 = 0x01 << EMMC_CMD_IDX_EN_POS;
    pub const EMMC_CMD_IDX_EN: u16 = EMMC_CMD_IDX_EN_MASK;
    pub const EMMC_DATA_TOUT_EN_POS: u16 = 4;
    pub const EMMC_DATA_TOUT_EN_MASK: u16 = 0x01 << EMMC_DATA_TOUT_EN_POS;
    pub const EMMC_DATA_TOUT_EN: u16 = EMMC_DATA_TOUT_EN_MASK;
    pub const EMMC_DATA_CRC_EN_POS: u16 = 5;
    pub const EMMC_DATA_CRC_EN_MASK: u16 = 0x01 << EMMC_DATA_CRC_EN_POS;
    pub const EMMC_DATA_CRC_EN: u16 = EMMC_DATA_CRC_EN_MASK;
    pub const EMMC_DATA_END_BIT_EN_POS: u16 = 6;
    pub const EMMC_DATA_END_BIT_EN_MASK: u16 = 0x01 << EMMC_DATA_END_BIT_EN_POS;
    pub const EMMC_DATA_END_BIT_EN: u16 = EMMC_DATA_END_BIT_EN_MASK;
    pub const EMMC_AUTO_CMD_EN_POS: u16 = 8;
    pub const EMMC_AUTO_CMD_EN_MASK: u16 = 0x01 << EMMC_AUTO_CMD_EN_POS;
    pub const EMMC_AUTO_CMD_EN: u16 = EMMC_AUTO_CMD_EN_MASK;
    pub const EMMC_ADMA_EN_POS: u16 = 9;
    pub const EMMC_ADMA_EN_MASK: u16 = 0x01 << EMMC_ADMA_EN_POS;
    pub const EMMC_ADMA_EN: u16 = EMMC_ADMA_EN_MASK;
    pub const EMMC_TUNING_EN_POS: u16 = 10;
    pub const EMMC_TUNING_EN_MASK: u16 = 0x01 << EMMC_TUNING_EN_POS;
    pub const EMMC_TUNING_EN: u16 = EMMC_TUNING_EN_MASK;
    pub const EMMC_RESP_EN_POS: u16 = 11;
    pub const EMMC_RESP_EN_MASK: u16 = 0x01 << EMMC_RESP_EN_POS;
    pub const EMMC_RESP_EN: u16 = EMMC_RESP_EN_MASK;
    pub const EMMC_BOOT_ACK_EN_POS: u16 = 12;
    pub const EMMC_BOOT_ACK_EN_MASK: u16 = 0x01 << EMMC_BOOT_ACK_EN_POS;
    pub const EMMC_BOOT_ACK_EN: u16 = EMMC_BOOT_ACK_EN_MASK;
}

impl Reg {
    pub fn emmc_set_error_int_en(&self, error_int_en: u16) {
        let addr = self.base_addr + emmc_error_int_en_bits::EMMC_ERROR_INT_EN_OFFSET;
        self.write_reg16(addr, error_int_en);
    }

    pub fn emmc_get_error_int_en(&self) -> u16{
        let addr = self.base_addr + emmc_error_int_en_bits::EMMC_ERROR_INT_EN_OFFSET;
        self.read_reg16(addr)
    }

    pub fn emmc_enable_all_error_int(&self) {
        let addr = self.base_addr + emmc_error_int_en_bits::EMMC_ERROR_INT_EN_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_error_int_en_bits::EMMC_CMD_TOUT_EN
                | emmc_error_int_en_bits::EMMC_CMD_CRC_EN
                | emmc_error_int_en_bits::EMMC_CMD_END_BIT_EN
                | emmc_error_int_en_bits::EMMC_CMD_IDX_EN
                | emmc_error_int_en_bits::EMMC_DATA_TOUT_EN
                | emmc_error_int_en_bits::EMMC_DATA_CRC_EN
                | emmc_error_int_en_bits::EMMC_DATA_END_BIT_EN
                | emmc_error_int_en_bits::EMMC_AUTO_CMD_EN
                | emmc_error_int_en_bits::EMMC_ADMA_EN
                | emmc_error_int_en_bits::EMMC_TUNING_EN
                | emmc_error_int_en_bits::EMMC_RESP_EN
                | emmc_error_int_en_bits::EMMC_BOOT_ACK_EN;
        self.write_reg16(addr, value);
    }

    pub fn emmc_disable_all_error_int(&self) {
        let addr = self.base_addr + emmc_error_int_en_bits::EMMC_ERROR_INT_EN_OFFSET;
        self.write_reg16(addr, 0);
    }
}

pub mod emmc_normal_int_sig_en_bits {
    pub const EMMC_NORMAL_INT_SIG_EN_OFFSET: u64 = 0x38;

    pub const EMMC_CMD_COMPLETE_SIG_EN_POS: u16 = 0;
    pub const EMMC_CMD_COMPLETE_SIG_EN_MASK: u16 = 0x01 << EMMC_CMD_COMPLETE_SIG_EN_POS;
    pub const EMMC_CMD_COMPLETE_SIG_EN: u16 = EMMC_CMD_COMPLETE_SIG_EN_MASK;
    pub const EMMC_XFER_COMPLETE_SIG_EN_POS: u16 = 1;
    pub const EMMC_XFER_COMPLETE_SIG_EN_MASK: u16 = 0x01 << EMMC_XFER_COMPLETE_SIG_EN_POS;
    pub const EMMC_XFER_COMPLETE_SIG_EN: u16 = EMMC_XFER_COMPLETE_SIG_EN_MASK;
    pub const EMMC_BGAP_EVENT_SIG_EN_POS: u16 = 2;
    pub const EMMC_BGAP_EVENT_SIG_EN_MASK: u16 = 0x01 << EMMC_BGAP_EVENT_SIG_EN_POS;
    pub const EMMC_BGAP_EVENT_SIG_EN: u16 = EMMC_BGAP_EVENT_SIG_EN_MASK;
    pub const EMMC_DMA_INTERRUPT_SIG_EN_POS: u16 = 3;
    pub const EMMC_DMA_INTERRUPT_SIG_EN_MASK: u16 = 0x01 << EMMC_DMA_INTERRUPT_SIG_EN_POS;
    pub const EMMC_DMA_INTERRUPT_SIG_EN: u16 = EMMC_DMA_INTERRUPT_SIG_EN_MASK;
    pub const EMMC_BUF_WR_READY_SIG_EN_POS: u16 = 4;
    pub const EMMC_BUF_WR_READY_SIG_EN_MASK: u16 = 0x01 << EMMC_BUF_WR_READY_SIG_EN_POS;
    pub const EMMC_BUF_WR_READY_SIG_EN: u16 = EMMC_BUF_WR_READY_SIG_EN_MASK;
    pub const EMMC_BUF_RD_READY_SIG_EN_POS: u16 = 5;
    pub const EMMC_BUF_RD_READY_SIG_EN_MASK: u16 = 0x01 << EMMC_BUF_RD_READY_SIG_EN_POS;
    pub const EMMC_BUF_RD_READY_SIG_EN: u16 = EMMC_BUF_RD_READY_SIG_EN_MASK;
    pub const EMMC_CARD_INSERTION_SIG_EN_POS: u16 = 6;
    pub const EMMC_CARD_INSERTION_SIG_EN_MASK: u16 = 0x01 << EMMC_CARD_INSERTION_SIG_EN_POS;
    pub const EMMC_CARD_INSERTION_SIG_EN: u16 = EMMC_CARD_INSERTION_SIG_EN_MASK;
    pub const EMMC_CARD_REMOVAL_SIG_EN_POS: u16 = 7;
    pub const EMMC_CARD_REMOVAL_SIG_EN_MASK: u16 = 0x01 << EMMC_CARD_REMOVAL_SIG_EN_POS;
    pub const EMMC_CARD_REMOVAL_SIG_EN: u16 = EMMC_CARD_REMOVAL_SIG_EN_MASK;
    pub const EMMC_CARD_INTERRUPT_SIG_EN_POS: u16 = 8;
    pub const EMMC_CARD_INTERRUPT_SIG_EN_MASK: u16 = 0x01 << EMMC_CARD_INTERRUPT_SIG_EN_POS;
    pub const EMMC_CARD_INTERRUPT_SIG_EN: u16 = EMMC_CARD_INTERRUPT_SIG_EN_MASK;
    pub const EMMC_CQE_EVENT_SIG_EN_POS: u16 = 14;
    pub const EMMC_CQE_EVENT_SIG_EN_MASK: u16 = 0x01 << EMMC_CQE_EVENT_SIG_EN_POS;
    pub const EMMC_CQE_EVENT_SIG_EN: u16 = EMMC_CQE_EVENT_SIG_EN_MASK;
}

impl Reg {
    pub fn emmc_set_normal_int_sig_en(&self, normal_int_sig_en: u16) {
        let addr = self.base_addr + emmc_normal_int_sig_en_bits::EMMC_NORMAL_INT_SIG_EN_OFFSET;
        self.write_reg16(addr, normal_int_sig_en);
    }

    pub fn emmc_get_normal_int_sig_en(&self) -> u16{
        let addr = self.base_addr + emmc_normal_int_sig_en_bits::EMMC_NORMAL_INT_SIG_EN_OFFSET;
        self.read_reg16(addr)
    }

    pub fn emmc_enable_all_normal_int_sig(&self) {
        let addr = self.base_addr + emmc_normal_int_sig_en_bits::EMMC_NORMAL_INT_SIG_EN_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_normal_int_sig_en_bits::EMMC_CMD_COMPLETE_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_XFER_COMPLETE_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_BGAP_EVENT_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_DMA_INTERRUPT_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_BUF_WR_READY_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_BUF_RD_READY_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_CARD_INSERTION_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_CARD_REMOVAL_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_CARD_INTERRUPT_SIG_EN
                | emmc_normal_int_sig_en_bits::EMMC_CQE_EVENT_SIG_EN;
        self.write_reg16(addr, value);
    }

    pub fn emmc_disable_all_normal_int_sig(&self) {
        let addr = self.base_addr + emmc_normal_int_sig_en_bits::EMMC_NORMAL_INT_SIG_EN_OFFSET;
        self.write_reg16(addr, 0);
    }
}

pub mod emmc_error_int_sig_en_bits {
    pub const EMMC_ERROR_INT_SIG_EN_OFFSET: u64 = 0x3a;

    pub const EMMC_CMD_TOUT_SIG_EN_POS: u16 = 0;
    pub const EMMC_CMD_TOUT_SIG_EN_MASK: u16 = 0x01 << EMMC_CMD_TOUT_SIG_EN_POS;
    pub const EMMC_CMD_TOUT_SIG_EN: u16 = EMMC_CMD_TOUT_SIG_EN_MASK;
    pub const EMMC_CMD_CRC_SIG_EN_POS: u16 = 1;
    pub const EMMC_CMD_CRC_SIG_EN_MASK: u16 = 0x01 << EMMC_CMD_CRC_SIG_EN_POS;
    pub const EMMC_CMD_CRC_SIG_EN: u16 = EMMC_CMD_CRC_SIG_EN_MASK;
    pub const EMMC_CMD_END_BIT_SIG_EN_POS: u16 = 2;
    pub const EMMC_CMD_END_BIT_SIG_EN_MASK: u16 = 0x01 << EMMC_CMD_END_BIT_SIG_EN_POS;
    pub const EMMC_CMD_END_BIT_SIG_EN: u16 = EMMC_CMD_END_BIT_SIG_EN_MASK;
    pub const EMMC_CMD_IDX_SIG_EN_POS: u16 = 3;
    pub const EMMC_CMD_IDX_SIG_EN_MASK: u16 = 0x01 << EMMC_CMD_IDX_SIG_EN_POS;
    pub const EMMC_CMD_IDX_SIG_EN: u16 = EMMC_CMD_IDX_SIG_EN_MASK;
    pub const EMMC_DATA_TOUT_SIG_EN_POS: u16 = 4;
    pub const EMMC_DATA_TOUT_SIG_EN_MASK: u16 = 0x01 << EMMC_DATA_TOUT_SIG_EN_POS;
    pub const EMMC_DATA_TOUT_SIG_EN: u16 = EMMC_DATA_TOUT_SIG_EN_MASK;
    pub const EMMC_DATA_CRC_SIG_EN_POS: u16 = 5;
    pub const EMMC_DATA_CRC_SIG_EN_MASK: u16 = 0x01 << EMMC_DATA_CRC_SIG_EN_POS;
    pub const EMMC_DATA_CRC_SIG_EN: u16 = EMMC_DATA_CRC_SIG_EN_MASK;
    pub const EMMC_DATA_END_BIT_SIG_EN_POS: u16 = 6;
    pub const EMMC_DATA_END_BIT_SIG_EN_MASK: u16 = 0x01 << EMMC_DATA_END_BIT_SIG_EN_POS;
    pub const EMMC_DATA_END_BIT_SIG_EN: u16 = EMMC_DATA_END_BIT_SIG_EN_MASK;
    pub const EMMC_AUTO_CMD_SIG_EN_POS: u16 = 8;
    pub const EMMC_AUTO_CMD_SIG_EN_MASK: u16 = 0x01 << EMMC_AUTO_CMD_SIG_EN_POS;
    pub const EMMC_AUTO_CMD_SIG_EN: u16 = EMMC_AUTO_CMD_SIG_EN_MASK;
    pub const EMMC_ADMA_SIG_EN_POS: u16 = 9;
    pub const EMMC_ADMA_SIG_EN_MASK: u16 = 0x01 << EMMC_ADMA_SIG_EN_POS;
    pub const EMMC_ADMA_SIG_EN: u16 = EMMC_ADMA_SIG_EN_MASK;
    pub const EMMC_TUNING_SIG_EN_POS: u16 = 10;
    pub const EMMC_TUNING_SIG_EN_MASK: u16 = 0x01 << EMMC_TUNING_SIG_EN_POS;
    pub const EMMC_TUNING_SIG_EN: u16 = EMMC_TUNING_SIG_EN_MASK;
    pub const EMMC_RESP_SIG_EN_POS: u16 = 11;
    pub const EMMC_RESP_SIG_EN_MASK: u16 = 0x01 << EMMC_RESP_SIG_EN_POS;
    pub const EMMC_RESP_SIG_EN: u16 = EMMC_RESP_SIG_EN_MASK;
    pub const EMMC_BOOT_ACK_SIG_EN_POS: u16 = 12;
    pub const EMMC_BOOT_ACK_SIG_EN_MASK: u16 = 0x01 << EMMC_BOOT_ACK_SIG_EN_POS;
    pub const EMMC_BOOT_ACK_SIG_EN: u16 = EMMC_BOOT_ACK_SIG_EN_MASK;
}

impl Reg {
    pub fn emmc_set_error_int_sig_en(&self, error_int_sig_en: u16) {
        let addr = self.base_addr + emmc_error_int_sig_en_bits::EMMC_ERROR_INT_SIG_EN_OFFSET;
        self.write_reg16(addr, error_int_sig_en);
    }

    pub fn emmc_get_error_int_sig_en(&self) -> u16{
        let addr = self.base_addr + emmc_error_int_sig_en_bits::EMMC_ERROR_INT_SIG_EN_OFFSET;
        self.read_reg16(addr)
    }

    pub fn emmc_enable_all_error_int_sig(&self) {
        let addr = self.base_addr + emmc_error_int_sig_en_bits::EMMC_ERROR_INT_SIG_EN_OFFSET;
        let mut value = self.read_reg16(addr);
        value |= emmc_error_int_sig_en_bits::EMMC_CMD_TOUT_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_CMD_CRC_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_CMD_END_BIT_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_CMD_IDX_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_DATA_TOUT_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_DATA_CRC_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_DATA_END_BIT_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_AUTO_CMD_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_ADMA_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_TUNING_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_RESP_SIG_EN
                | emmc_error_int_sig_en_bits::EMMC_BOOT_ACK_SIG_EN;
        self.write_reg16(addr, value);
    }

    pub fn emmc_disable_all_error_int_sig(&self) {
        let addr = self.base_addr + emmc_error_int_sig_en_bits::EMMC_ERROR_INT_SIG_EN_OFFSET;
        self.write_reg16(addr, 0);
    }
}

/* TODO
 *
 * offset 0x50 - 0x6e
*/

/// This module contains the offset position of the `EMMC_ADMA_ID` register and the definitions of its individual bits.
/// The `EMMC_ADMA_ID` register is a 32-bit read-write register that contains the ADMA integrated descriptor address.
pub mod emmc_adma_id_bits {
    /// the offset of the `EMMC_ADMA_ID` register from the base address of the SDHCI controller.
    pub const EMMC_ADMA_ID_OFFSET: u64 = 0x78;
    /// ADMA Integrated Descriptor Address
    pub const EMMC_ADMA_ID_POS: u32 = 0;
    pub const EMMC_ADMA_ID_MASK: u32 = 0xffffffff << EMMC_ADMA_ID_POS;
    pub const EMMC_ADMA_ID: u32 = EMMC_ADMA_ID_MASK;
}

/// This module implements read and write operations for the `EMMC_ADMA_ID` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_adma_id_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_ADMA_ID` register.
    ///
    /// These bits indicate the 32-bit of the ADMA Integrated Descriptor 
    /// address. The start address of Integrated Descriptor is set to these 
    /// register bits. The ADMA3 fetches one Descriptor Address and 
    /// increments these bits to indicate the next Descriptor address.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_adma_id(&self) -> u32{
        let addr = self.base_addr + emmc_adma_id_bits::EMMC_ADMA_ID_OFFSET;
        self.read_reg(addr)
    }

    /// Set the value of the `EMMC_ADMA_ID` register.
    ///
    /// # Arguments
    /// 
    /// - `adma_id` - The value to be written to the register. The value should be in the range of 0x00000000 to 0xffffffff.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_adma_id(&self, adma_id: u32) {
        let addr = self.base_addr + emmc_adma_id_bits::EMMC_ADMA_ID_OFFSET;
        self.write_reg(addr, adma_id);
    }
}

/// This module contains the offset position of the `EMMC_SLOT_INTR_STATUS` register and the definitions of its individual bits.
/// The `EMMC_SLOT_INTR_STATUS` register is a 8-bit read-only register that contains the interrupt signal for each slot.
pub mod emmc_slot_int_status_bits {
    /// the offset of the `EMMC_SLOT_INTR_STATUS` register from the base address of the SDHCI controller.
    pub const EMMC_SLOT_INT_STATUS_OFFSET: u64 = 0xfc;
    /// Interrupt Signal for Each Slot
    pub const EMMC_INTR_SLOT_POS: u8 = 0;
    pub const EMMC_INTR_SLOT_MASK: u8 = 0xff << EMMC_INTR_SLOT_POS;
    pub const EMMC_INTR_SLOT: u8 = EMMC_INTR_SLOT_MASK;
}

/// This module implements read operations for the `EMMC_SLOT_INTR_STATUS` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_slot_int_status_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_SLOT_INTR_STATUS` register. 
    ///
    /// Host Controller support single card slot. This register shall always return 0.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00
    pub fn emmc_get_slot_int_status(&self) -> u8{
        let addr = self.base_addr + emmc_slot_int_status_bits::EMMC_SLOT_INT_STATUS_OFFSET;
        self.read_reg8(addr)
    }
}

/// This module contains the offset position of the `EMMC_HOST_CNTRL_VERS` register and the definitions of its individual bits.
/// The `EMMC_HOST_CNTRL_VERS` register is a 16-bit read-only register that contains the specification version number and the vendor version number.
pub mod emmc_host_ctrl_ver_bits {
    /// the offset of the `EMMC_HOST_CNTRL_VERS` register from the base address of the SDHCI controller.
    pub const EMMC_HOST_CTRL_VER_OFFSET: u64 = 0xfe;
    /// Specification Version Number
    pub const EMMC_SPEC_VERSION_POS: u16 = 0;
    pub const EMMC_SPEC_VERSION_MASK: u16 = 0xff << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION: u16 = EMMC_SPEC_VERSION_MASK;
    pub const EMMC_SPEC_VERSION_V100: u16 = 0x00 << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION_V200: u16 = 0x01 << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION_V300: u16 = 0x02 << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION_V400: u16 = 0x03 << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION_V410: u16 = 0x04 << EMMC_SPEC_VERSION_POS;
    pub const EMMC_SPEC_VERSION_V420: u16 = 0x05 << EMMC_SPEC_VERSION_POS;
    /// Vendor Version Number
    pub const EMMC_VENDOR_VERSION_POS: u16 = 8;
    pub const EMMC_VENDOR_VERSION_MASK: u16 = 0xff << EMMC_VENDOR_VERSION_POS;
    pub const EMMC_VENDOR_VERSION: u16 = EMMC_VENDOR_VERSION_MASK;
}

/// This module implements read operations for the `EMMC_HOST_CNTRL_VERS` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_host_ctrl_ver_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_HOST_CNTRL_VERS` register. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x1005
    /// 
    pub fn emmc_get_host_ctrl_ver(&self) -> u16{
        let addr = self.base_addr + emmc_host_ctrl_ver_bits::EMMC_HOST_CTRL_VER_OFFSET;
        self.read_reg16(addr)
    }

    /// Returns the spec version.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// one of the following values defined in `emmc_host_ctrl_ver_bits`:
    /// - `EMMC_SPEC_VERSION_V100`
    /// - `EMMC_SPEC_VERSION_V200`
    /// - `EMMC_SPEC_VERSION_V300`
    /// - `EMMC_SPEC_VERSION_V400`
    /// - `EMMC_SPEC_VERSION_V410`
    /// - `EMMC_SPEC_VERSION_V420`
    pub fn emmc_get_spec_version(&self) -> u16 {
        let addr = self.base_addr + emmc_host_ctrl_ver_bits::EMMC_HOST_CTRL_VER_OFFSET;
        (self.read_reg16(addr) & emmc_host_ctrl_ver_bits::EMMC_SPEC_VERSION) >> emmc_host_ctrl_ver_bits::EMMC_SPEC_VERSION_POS
    }

    /// returns the vendor version
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - the vendor version read from the register.
    pub fn emmc_get_vendor_version(&self) -> u16 {
        let addr = self.base_addr + emmc_host_ctrl_ver_bits::EMMC_HOST_CTRL_VER_OFFSET;
        (self.read_reg16(addr) & emmc_host_ctrl_ver_bits::EMMC_VENDOR_VERSION) >> emmc_host_ctrl_ver_bits::EMMC_VENDOR_VERSION_POS
    }
}

/* TODO
 *
 * offset 0x0180 - 0x01dc
*/

/// This module contains the offset position of the `EMMC_VER_ID` register and the definitions of its individual bits.
/// The `EMMC_VER_ID` register is a 32-bit read-only register that contains the current version number.
pub mod emmc_ver_id_bits {
    /// the offset of the `EMMC_VER_ID` register from the base address of the SDHCI controller.
    pub const EMMC_VER_ID_OFFSET: u64 = 0x500;
    /// version id
    pub const EMMC_VER_ID_POS: u32 = 0;
    pub const EMMC_VER_ID_MASK: u32 = 0xffffffff << EMMC_VER_ID_POS;
    pub const EMMC_VER_ID: u32 = EMMC_VER_ID_MASK;
}

/// This module implements read operations for the `EMMC_VER_ID` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_ver_id_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_VER_ID` register. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_ver_id(&self) -> u32 {
        let addr = self.base_addr + emmc_ver_id_bits::EMMC_VER_ID_OFFSET;
        self.read_reg(addr)
    }
}

/// This module contains the offset position of the `EMMC_VER_TYPE` register and the definitions of its individual bits.
/// The `EMMC_VER_TYPE` register is a 32-bit read-only register that contains the current version type.
pub mod emmc_ver_type_bits {
    /// the offset of the `EMMC_VER_TYPE` register from the base address of the SDHCI controller.
    pub const EMMC_VER_TYPE_OFFSET: u64 = 0x504;
    /// version type
    pub const EMMC_VER_TYPE_POS: u32 = 0;
    pub const EMMC_VER_TYPE_MASK: u32 = 0xffffffff << EMMC_VER_TYPE_POS;
    pub const EMMC_VER_TYPE: u32 = EMMC_VER_TYPE_MASK;
}

/// This module implements read operations for the `EMMC_VER_TYPE` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_ver_type_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_VER_TYPE` register. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_ver_type(&self) -> u32 {
        let addr = self.base_addr + emmc_ver_type_bits::EMMC_VER_TYPE_OFFSET;
        self.read_reg(addr)
    }
}

/// This module contains the offset position of the `EMMC_HOST_CTRL3` register and the definitions of its individual bits.
/// The `EMMC_HOST_CTRL3` register is a 8-bit read-write register that contains some control setinggs for host.
pub mod emmc_host_ctrl3_bits {
    /// the offset of the `EMMC_HOST_CTRL3` register from the base address of the SDHCI controller.
    pub const EMMC_HOST_CTRL3_OFFSET: u64 = 0x508;
    /// Command conflict check
    pub const EMMC_CMD_CONFLICT_CHECK_POS: u8 = 0;
    pub const EMMC_CMD_CONFLICT_CHECK_MASK: u8 = 0x01 << EMMC_CMD_CONFLICT_CHECK_POS;
    pub const EMMC_CMD_CONFLICT_CHECK: u8 = EMMC_CMD_CONFLICT_CHECK_MASK;
    /// Internal clock gating disable control
    pub const EMMC_SW_CG_DIS_POS: u8 = 4;
    pub const EMMC_SW_CG_DIS_MASK: u8 = 0x01 << EMMC_SW_CG_DIS_POS;
    pub const EMMC_SW_CG_DIS: u8 = EMMC_SW_CG_DIS_MASK;
}

/// This module implements read and write operations for the `EMMC_HOST_CTRL3` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_host_ctrl3_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_HOST_CTRL3` register. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x01
    pub fn emmc_get_host_ctrl3(&self) -> u8 {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        self.read_reg8(addr)
    }

    /// Set the entire value of the `EMMC_HOST_CTRL3` register.
    ///
    /// # Arguments
    /// 
    /// - `host_ctrl3` - The value to be written to the register. It is a combination of individual bits defined in `emmc_host_ctrl3_bits`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_host_ctrl3(&self, host_ctrl3: u8) {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        self.write_reg8(addr, host_ctrl3);
    }

    /// Enable the check for command conflict after 1 card clock cycle
    ///
    /// Host Controller monitors the CMD line whenever a command is 
    /// issued and checks whether the value driven on sd_cmd_out 
    /// matches the value on sd_cmd_in at next subsequent edge of 
    /// cclk_tx to determine command conflict error. 
    /// This bit is cleared only if the feed back delay (including IO Pad delay) is more than 
    /// (t_card_clk_period - t_setup), where t_setup is the setup time of 
    /// a flop in Host Controller.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_cmd_conflict_check(&self) {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_host_ctrl3_bits::EMMC_CMD_CONFLICT_CHECK);
    }

    /// Disable command conflict check
    ///
    /// Host Controller monitors the CMD line whenever a command is 
    /// issued and checks whether the value driven on sd_cmd_out 
    /// matches the value on sd_cmd_in at next subsequent edge of 
    /// cclk_tx to determine command conflict error. 
    /// This bit is cleared only if the feed back delay (including IO Pad delay) is more than 
    /// (t_card_clk_period - t_setup), where t_setup is the setup time of 
    /// a flop in Host Controller.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_cmd_conflict_check(&self) {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_host_ctrl3_bits::EMMC_CMD_CONFLICT_CHECK);
    }

    /// Enable internal clock gate.Internal clock gates are active and clock gating is controlled internally
    ///
    /// This bit must be used to disable IP's internal clock gating when 
    /// required. when disabled clocks are not gated. Clocks to the core 
    /// (except hclk) must be stopped when programming this bit. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_internal_clock_gate(&self) {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value | emmc_host_ctrl3_bits::EMMC_SW_CG_DIS);
    }

    /// Disable internal clock gate.the clocks are not gated internally 
    ///
    /// This bit must be used to disable IP's internal clock gating when 
    /// required. when disabled clocks are not gated. Clocks to the core 
    /// (except hclk) must be stopped when programming this bit. 
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_internal_clock_gate(&self) {
        let addr = self.base_addr + emmc_host_ctrl3_bits::EMMC_HOST_CTRL3_OFFSET;
        let value = self.read_reg8(addr);
        self.write_reg8(addr, value & !emmc_host_ctrl3_bits::EMMC_SW_CG_DIS);
    }
}

/* TODO
 *
 * EMMC_EMMC_CTRL 0x052C HW 0x0000000C EMMC Control Register 
 * EMMC_BOOT_CTRL 0x052E HW 0x00000000 Boot Control Register 
 * EMMC_AT_CTRL 0x0540 W 0x00000000 Boot Control Register 
 * EMMC_AT_STAT 
*/


/// This module contains the offset position of the `EMMC_DLL_CTRL` register and the definitions of its individual bits.
/// The `EMMC_DLL_CTRL` register is a 32-bit read-write register that contains some control setinggs for dll.
pub mod emmc_dll_ctrl_bits {
    /// Offset of the `EMMC_DLL_CTRL` register from the base address of the SDHCI controller.
    pub const EMMC_DLL_CTRL_OFFSET: u64 = 0x800;
    /// DLL working indication
    pub const EMMC_DLL_START_POS: u32 = 0;
    pub const EMMC_DLL_START_MASK: u32 = 0x01 << EMMC_DLL_START_POS;
    pub const EMMC_DLL_START: u32 = EMMC_DLL_START_MASK;
    /// DLL soft reset indication
    pub const EMMC_DLL_SRST_POS: u32 = 1;
    pub const EMMC_DLL_SRST_MASK: u32 = 0x01 << EMMC_DLL_SRST_POS;
    pub const EMMC_DLL_SRST: u32 = EMMC_DLL_SRST_MASK;
    /// DLL increment value
    pub const EMMC_DLL_INCRMENT_POS: u32 = 8;
    pub const EMMC_DLL_INCRMENT_MASK: u32 = 0xff << EMMC_DLL_INCRMENT_POS;
    pub const EMMC_DLL_INCRMENT: u32 = EMMC_DLL_INCRMENT_MASK;
    /// DLL start point for phase detect
    pub const EMMC_DLL_START_POINT_POS: u32 = 16;
    pub const EMMC_DLL_START_POINT_MASK: u32 = 0xff << EMMC_DLL_START_POINT_POS;
    pub const EMMC_DLL_START_POINT: u32 = EMMC_DLL_START_POINT_MASK;
    /// DLL bypass mode select.
    pub const EMMC_DLL_BYPASS_POS: u32 = 24;
    pub const EMMC_DLL_BYPASS_MASK: u32 = 0xff << EMMC_DLL_BYPASS_POS;
    pub const EMMC_DLL_BYPASS: u32 = EMMC_DLL_BYPASS_MASK;
}

/// This module implements read and write operations for the `EMMC_DLL_CTRL` register itself as well as its individual bits.
/// - The definition of the bit is in the `emmc_dll_ctrl_bits` module.
impl Reg {
    /// Return the entire value of the `EMMC_DLL_CTRL` register. 
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - The value read from the register. According to the TRM description, the default value is 0x00000000
    pub fn emmc_get_dll_ctrl(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        self.read_reg(addr)
    }

    /// Set the entire value of the `EMMC_DLL_CTRL` register.
    ///
    /// # Arguments
    /// 
    /// - `dll_ctrl` - The value to be written to the register. It is a combination of individual bits defined in `emmc_dll_ctrl_bits`.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_dll_ctrl(&self, dll_ctrl: u32) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        self.write_reg(addr, dll_ctrl);
    }

    /// Enable DLL start by setting the `EMMC_DLL_START` bit to `1`.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_dll_start(&self) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, value | emmc_dll_ctrl_bits::EMMC_DLL_START);
    }

    /// Disable DLL start by setting the `EMMC_DLL_START` bit to `0`
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_dll_start(&self) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, value & !emmc_dll_ctrl_bits::EMMC_DLL_START);
    }

    /// Reset the DLL by setting the `EMMC_DLL_SRST` bit to `1`.
    ///
    /// After the reset is completed, this bit will be cleared by hardware.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_reset_dll(&self) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, value | emmc_dll_ctrl_bits::EMMC_DLL_SRST);
    }

    /// Check the DLL resetting is finished or not.
    ///
    /// After the reset is completed, the `EMMC_DLL_SRST` bit will be cleared by hardware.
    /// 
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_dll_reset_is_finished(&self) -> bool {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        (self.read_reg(addr) & emmc_dll_ctrl_bits::EMMC_DLL_SRST) == 0
    }

    /// Set the DLL increment value.
    ///
    /// # Arguments
    /// 
    /// - `dll_incrment` - The value to be written to the register.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_dll_incrment(&self, dll_incrment: u8) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_ctrl_bits::EMMC_DLL_INCRMENT_MASK) | ((dll_incrment as u32) << emmc_dll_ctrl_bits::EMMC_DLL_INCRMENT_POS));
    }

    /// Set the DLL start point for phase detect.
    ///
    /// # Arguments
    /// 
    /// - `dll_incrment` - The value to be written to the register.
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_set_dll_start_point(&self, dll_start_point: u8) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_ctrl_bits::EMMC_DLL_START_POINT_MASK) | ((dll_start_point as u32) << emmc_dll_ctrl_bits::EMMC_DLL_START_POINT_POS));
    }

    /// Enable DLL bypass by setting the `EMMC_DLL_BYPASS` bit to `1`.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_enable_dll_bypass(&self) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, value | emmc_dll_ctrl_bits::EMMC_DLL_BYPASS);
    }

    /// Disable DLL bypass by setting the `EMMC_DLL_BYPASS` bit to `0`.
    ///
    /// # Arguments
    /// 
    /// - None
    /// 
    /// # Returns
    /// 
    /// - None
    pub fn emmc_disable_dll_bypass(&self) {
        let addr = self.base_addr + emmc_dll_ctrl_bits::EMMC_DLL_CTRL_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, value & !emmc_dll_ctrl_bits::EMMC_DLL_BYPASS);
    }
}

pub mod emmc_dll_rxclk_bits {
    pub const EMMC_DLL_RXCLK_OFFSET: u64 = 0x804;

    pub const EMMC_RX_TAP_NUM_POS: u32 = 0;
    pub const EMMC_RX_TAP_NUM_MASK: u32 = 0x1f << EMMC_RX_TAP_NUM_POS;
    pub const EMMC_RX_TAP_NUM: u32 = EMMC_RX_TAP_NUM_MASK;
    pub const EMMC_RX_TAP_VALUE_POS: u32 = 8;
    pub const EMMC_RX_TAP_VALUE_MASK: u32 = 0xff << EMMC_RX_TAP_VALUE_POS;
    pub const EMMC_RX_TAP_VALUE: u32 = EMMC_RX_TAP_VALUE_MASK;
    pub const EMMC_RX_DELAY_NUM_POS: u32 = 16;
    pub const EMMC_RX_DELAY_NUM_MASK: u32 = 0xff << EMMC_RX_DELAY_NUM_POS;
    pub const EMMC_RX_DELAY_NUM: u32 = EMMC_RX_DELAY_NUM_MASK;
    pub const EMMC_RX_TAP_NUM_SEL_POS: u32 = 24;
    pub const EMMC_RX_TAP_NUM_SEL_MASK: u32 = 0x01 << EMMC_RX_TAP_NUM_SEL_POS;
    pub const EMMC_RX_TAP_NUM_SEL: u32 = EMMC_RX_TAP_NUM_SEL_MASK;
    pub const EMMC_RX_TAP_VALUE_SEL_POS: u32 = 25;
    pub const EMMC_RX_TAP_VALUE_SEL_MASK: u32 = 0x01 << EMMC_RX_TAP_VALUE_SEL_POS;
    pub const EMMC_RX_TAP_VALUE_SEL: u32 = EMMC_RX_TAP_VALUE_SEL_MASK;
    pub const EMMC_RX_DELAY_NUM_SEL_POS: u32 = 26;
    pub const EMMC_RX_DELAY_NUM_SEL_MASK: u32 = 0x01 << EMMC_RX_DELAY_NUM_SEL_POS;
    pub const EMMC_RX_DELAY_NUM_SEL: u32 = EMMC_RX_DELAY_NUM_SEL_MASK;
    pub const EMMC_RX_CLK_OUT_SEL_POS: u32 = 27;
    pub const EMMC_RX_CLK_OUT_SEL_MASK: u32 = 0x01 << EMMC_RX_CLK_OUT_SEL_POS;
    pub const EMMC_RX_CLK_OUT_SEL: u32 = EMMC_RX_CLK_OUT_SEL_MASK;
    pub const EMMC_RX_CLK_CHANGE_WINDOW_POS: u32 = 28;
    pub const EMMC_RX_CLK_CHANGE_WINDOW_MASK: u32 = 0x01 << EMMC_RX_CLK_CHANGE_WINDOW_POS;
    pub const EMMC_RX_CLK_CHANGE_WINDOW: u32 = EMMC_RX_CLK_CHANGE_WINDOW_MASK;
    pub const EMMC_RX_CLK_SRC_SEL_POS: u32 = 29;
    pub const EMMC_RX_CLK_SRC_SEL_MASK: u32 = 0x01 << EMMC_RX_CLK_SRC_SEL_POS;
    pub const EMMC_RX_CLK_SRC_SEL: u32 = EMMC_RX_CLK_SRC_SEL_MASK;
}

impl Reg {
    pub fn emmc_set_dll_rxclk(&self, dll_rxclk: u32) {
        let addr = self.base_addr + emmc_dll_rxclk_bits::EMMC_DLL_RXCLK_OFFSET;
        self.write_reg(addr, dll_rxclk);
    }

    pub fn emmc_get_dll_rxclk(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_rxclk_bits::EMMC_DLL_RXCLK_OFFSET;
        self.read_reg(addr)
    }

    pub fn emmc_set_rx_tap_num(&self, rx_tap_num: u32) {
        let addr = self.base_addr + emmc_dll_rxclk_bits::EMMC_DLL_RXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_rxclk_bits::EMMC_RX_TAP_NUM_MASK) | rx_tap_num);
    }

    pub fn emmc_set_rx_tap_value(&self, rx_tap_value: u32) {
        let addr = self.base_addr + emmc_dll_rxclk_bits::EMMC_DLL_RXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_rxclk_bits::EMMC_RX_TAP_VALUE_MASK) | rx_tap_value);
    }

    pub fn emmc_set_rx_delay_num(&self, rx_delay_num: u32) {
        let addr = self.base_addr + emmc_dll_rxclk_bits::EMMC_DLL_RXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_rxclk_bits::EMMC_RX_DELAY_NUM_MASK) | rx_delay_num);
    }
}

pub mod emmc_dll_txclk_bits {
    pub const EMMC_DLL_TXCLK_OFFSET: u64 = 0x808;

    pub const EMMC_TX_TAP_NUM_POS: u32 = 0;
    pub const EMMC_TX_TAP_NUM_MASK: u32 = 0x1f << EMMC_TX_TAP_NUM_POS;
    pub const EMMC_TX_TAP_NUM: u32 = EMMC_TX_TAP_NUM_MASK;
    pub const EMMC_TX_TAP_VALUE_POS: u32 = 8;
    pub const EMMC_TX_TAP_VALUE_MASK: u32 = 0xff << EMMC_TX_TAP_VALUE_POS;
    pub const EMMC_TX_TAP_VALUE: u32 = EMMC_TX_TAP_VALUE_MASK;
    pub const EMMC_TX_DELAY_NUM_POS: u32 = 16;
    pub const EMMC_TX_DELAY_NUM_MASK: u32 = 0xff << EMMC_TX_DELAY_NUM_POS;
    pub const EMMC_TX_DELAY_NUM: u32 = EMMC_TX_DELAY_NUM_MASK;
    pub const EMMC_TX_TAP_NUM_SEL_POS: u32 = 24;
    pub const EMMC_TX_TAP_NUM_SEL_MASK: u32 = 0x01 << EMMC_TX_TAP_NUM_SEL_POS;
    pub const EMMC_TX_TAP_NUM_SEL: u32 = EMMC_TX_TAP_NUM_SEL_MASK;
    pub const EMMC_TX_TAP_VALUE_SEL_POS: u32 = 25;
    pub const EMMC_TX_TAP_VALUE_SEL_MASK: u32 = 0x01 << EMMC_TX_TAP_VALUE_SEL_POS;
    pub const EMMC_TX_TAP_VALUE_SEL: u32 = EMMC_TX_TAP_VALUE_SEL_MASK;
    pub const EMMC_TX_DELAY_NUM_SEL_POS: u32 = 26;
    pub const EMMC_TX_DELAY_NUM_SEL_MASK: u32 = 0x01 << EMMC_TX_DELAY_NUM_SEL_POS;
    pub const EMMC_TX_DELAY_NUM_SEL: u32 = EMMC_TX_DELAY_NUM_SEL_MASK;
    pub const EMMC_TX_CLK_OUT_SEL_POS: u32 = 27;
    pub const EMMC_TX_CLK_OUT_SEL_MASK: u32 = 0x01 << EMMC_TX_CLK_OUT_SEL_POS;
    pub const EMMC_TX_CLK_OUT_SEL: u32 = EMMC_TX_CLK_OUT_SEL_MASK;
}

impl Reg {
    pub fn emmc_set_dll_txclk(&self, dll_txclk: u32) {
        let addr = self.base_addr + emmc_dll_txclk_bits::EMMC_DLL_TXCLK_OFFSET;
        self.write_reg(addr, dll_txclk);
    }

    pub fn emmc_get_dll_txclk(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_txclk_bits::EMMC_DLL_TXCLK_OFFSET;
        self.read_reg(addr)
    }

    pub fn emmc_set_tx_tap_num(&self, tx_tap_num: u32) {
        let addr = self.base_addr + emmc_dll_txclk_bits::EMMC_DLL_TXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_txclk_bits::EMMC_TX_TAP_NUM_MASK) | tx_tap_num);
    }

    pub fn emmc_set_tx_tap_value(&self, tx_tap_value: u32) {
        let addr = self.base_addr + emmc_dll_txclk_bits::EMMC_DLL_TXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_txclk_bits::EMMC_TX_TAP_VALUE_MASK) | tx_tap_value);
    }

    pub fn emmc_set_tx_delay_num(&self, tx_delay_num: u32) {
        let addr = self.base_addr + emmc_dll_txclk_bits::EMMC_DLL_TXCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_txclk_bits::EMMC_TX_DELAY_NUM_MASK) | tx_delay_num);
    }
}

pub mod emmc_dll_strbin_bits {
    pub const EMMC_DLL_STRBINCLK_OFFSET: u64 = 0x80c;

    pub const EMMC_STRBIN_TAP_NUM_POS: u32 = 0;
    pub const EMMC_STRBIN_TAP_NUM_MASK: u32 = 0x1f << EMMC_STRBIN_TAP_NUM_POS;
    pub const EMMC_STRBIN_TAP_NUM: u32 = EMMC_STRBIN_TAP_NUM_MASK;
    pub const EMMC_STRBIN_TAP_VALUE_POS: u32 = 8;
    pub const EMMC_STRBIN_TAP_VALUE_MASK: u32 = 0xff << EMMC_STRBIN_TAP_VALUE_POS;
    pub const EMMC_STRBIN_TAP_VALUE: u32 = EMMC_STRBIN_TAP_VALUE_MASK;
    pub const EMMC_STRBIN_DELAY_NUM_POS: u32 = 16;
    pub const EMMC_STRBIN_DELAY_NUM_MASK: u32 = 0xff << EMMC_STRBIN_DELAY_NUM_POS;
    pub const EMMC_STRBIN_DELAY_NUM: u32 = EMMC_STRBIN_DELAY_NUM_MASK;
    pub const EMMC_STRBIN_TAP_NUM_SEL_POS: u32 = 24;
    pub const EMMC_STRBIN_TAP_NUM_SEL_MASK: u32 = 0x01 << EMMC_STRBIN_TAP_NUM_SEL_POS;
    pub const EMMC_STRBIN_TAP_NUM_SEL: u32 = EMMC_STRBIN_TAP_NUM_SEL_MASK;
    pub const EMMC_STRBIN_TAP_VALUE_SEL_POS: u32 = 25;
    pub const EMMC_STRBIN_TAP_VALUE_SEL_MASK: u32 = 0x01 << EMMC_STRBIN_TAP_VALUE_SEL_POS;
    pub const EMMC_STRBIN_TAP_VALUE_SEL: u32 = EMMC_STRBIN_TAP_VALUE_SEL_MASK;
    pub const EMMC_STRBIN_DELAY_NUM_SEL_POS: u32 = 26;
    pub const EMMC_STRBIN_DELAY_NUM_SEL_MASK: u32 = 0x01 << EMMC_STRBIN_DELAY_NUM_SEL_POS;
    pub const EMMC_STRBIN_DELAY_NUM_SEL: u32 = EMMC_STRBIN_DELAY_NUM_SEL_MASK;
    pub const EMMC_STRBIN_DELAY_EN_POS: u32 = 27;
    pub const EMMC_STRBIN_DELAY_EN_MASK: u32 = 0x01 << EMMC_STRBIN_DELAY_EN_POS;
    pub const EMMC_STRBIN_DELAY_EN: u32 = EMMC_STRBIN_DELAY_EN_MASK;
}

impl Reg {
    pub fn emmc_set_dll_strbin(&self, dll_strbin: u32) {
        let addr = self.base_addr + emmc_dll_strbin_bits::EMMC_DLL_STRBINCLK_OFFSET;
        self.write_reg(addr, dll_strbin);
    }

    pub fn emmc_get_dll_strbin(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_strbin_bits::EMMC_DLL_STRBINCLK_OFFSET;
        self.read_reg(addr)
    }

    pub fn emmc_set_strbin_tap_num(&self, strbin_tap_num: u32) {
        let addr = self.base_addr + emmc_dll_strbin_bits::EMMC_DLL_STRBINCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_strbin_bits::EMMC_STRBIN_TAP_NUM_MASK) | strbin_tap_num);
    }

    pub fn emmc_set_strbin_tap_value(&self, strbin_tap_value: u32) {
        let addr = self.base_addr + emmc_dll_strbin_bits::EMMC_DLL_STRBINCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_strbin_bits::EMMC_STRBIN_TAP_VALUE_MASK) | strbin_tap_value);
    }

    pub fn emmc_set_strbin_delay_num(&self, strbin_delay_num: u32) {
        let addr = self.base_addr + emmc_dll_strbin_bits::EMMC_DLL_STRBINCLK_OFFSET;
        let value = self.read_reg(addr);
        self.write_reg(addr, (value & !emmc_dll_strbin_bits::EMMC_STRBIN_DELAY_NUM_MASK) | strbin_delay_num);
    }
}

pub mod emmc_dll_status0_bits {
    pub const EMMC_DLL_STATUS0_OFFSET: u64 = 0x840;

    pub const EMMC_DLL_LOCK_VALUE_POS: u32 = 0;
    pub const EMMC_DLL_LOCK_VALUE_MASK: u32 = 0xff << EMMC_DLL_LOCK_VALUE_POS;
    pub const EMMC_DLL_LOCK_VALUE: u32 = EMMC_DLL_LOCK_VALUE_MASK;
    pub const EMMC_DLL_LOCK_POS: u32 = 8;
    pub const EMMC_DLL_LOCK_MASK: u32 = 0x01 << EMMC_DLL_LOCK_POS;
    pub const EMMC_DLL_LOCK: u32 = EMMC_DLL_LOCK_MASK;
    pub const EMMC_DLL_LOCK_TIMEOUT_POS: u32 = 9;
    pub const EMMC_DLL_LOCK_TIMEOUT_MASK: u32 = 0x01 << EMMC_DLL_LOCK_TIMEOUT_POS;
    pub const EMMC_DLL_LOCK_TIMEOUT: u32 = EMMC_DLL_LOCK_TIMEOUT_MASK;
}

impl Reg {
    pub fn emmc_get_dll_status0(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status0_bits::EMMC_DLL_STATUS0_OFFSET;
        self.read_reg(addr)
    }

    pub fn emmc_get_dll_lock_value(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status0_bits::EMMC_DLL_STATUS0_OFFSET;
        (self.read_reg(addr) & emmc_dll_status0_bits::EMMC_DLL_LOCK_VALUE) >> emmc_dll_status0_bits::EMMC_DLL_LOCK_VALUE_POS
    }

    pub fn emmc_dll_is_locked(&self) -> bool{
        let addr = self.base_addr + emmc_dll_status0_bits::EMMC_DLL_STATUS0_OFFSET;
        self.read_reg(addr) & emmc_dll_status0_bits::EMMC_DLL_LOCK == emmc_dll_status0_bits::EMMC_DLL_LOCK
    }

    pub fn emmc_dll_lock_is_timeout(&self) -> bool{
        let addr = self.base_addr + emmc_dll_status0_bits::EMMC_DLL_STATUS0_OFFSET;
        self.read_reg(addr) & emmc_dll_status0_bits::EMMC_DLL_LOCK_TIMEOUT == emmc_dll_status0_bits::EMMC_DLL_LOCK_TIMEOUT
    }
}

pub mod emmc_dll_status1_bits {
    pub const EMMC_DLL_STATUS1_OFFSET: u64 = 0x844;

    pub const EMMC_TXCLK_DELAY_VALUE_POS: u32 = 0;
    pub const EMMC_TXCLK_DELAY_VALUE_MASK: u32 = 0xff << EMMC_TXCLK_DELAY_VALUE_POS;
    pub const EMMC_TXCLK_DELAY_VALUE: u32 = EMMC_TXCLK_DELAY_VALUE_MASK;
    pub const EMMC_RXCLK_DELAY_VALUE_POS: u32 = 8;
    pub const EMMC_RXCLK_DELAY_VALUE_MASK: u32 = 0xff << EMMC_RXCLK_DELAY_VALUE_POS;
    pub const EMMC_RXCLK_DELAY_VALUE: u32 = EMMC_RXCLK_DELAY_VALUE_MASK;
    pub const EMMC_STRBIN_DELAY_VALUE_POS: u32 = 16;
    pub const EMMC_STRBIN_DELAY_VALUE_MASK: u32 = 0xff << EMMC_STRBIN_DELAY_VALUE_POS;
    pub const EMMC_STRBIN_DELAY_VALUE: u32 = EMMC_STRBIN_DELAY_VALUE_MASK;
}

impl Reg {
    pub fn emmc_get_dll_status1(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status1_bits::EMMC_DLL_STATUS1_OFFSET;
        self.read_reg(addr)
    }

    pub fn emmc_get_txclk_delay_value(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status1_bits::EMMC_DLL_STATUS1_OFFSET;
        (self.read_reg(addr) & emmc_dll_status1_bits::EMMC_TXCLK_DELAY_VALUE) >> emmc_dll_status1_bits::EMMC_TXCLK_DELAY_VALUE_POS
    }

    pub fn emmc_get_rxclk_delay_value(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status1_bits::EMMC_DLL_STATUS1_OFFSET;
        (self.read_reg(addr) & emmc_dll_status1_bits::EMMC_RXCLK_DELAY_VALUE) >> emmc_dll_status1_bits::EMMC_RXCLK_DELAY_VALUE_POS
    }

    pub fn emmc_get_strbin_delay_value(&self) -> u32 {
        let addr = self.base_addr + emmc_dll_status1_bits::EMMC_DLL_STATUS1_OFFSET;
        (self.read_reg(addr) & emmc_dll_status1_bits::EMMC_STRBIN_DELAY_VALUE) >> emmc_dll_status1_bits::EMMC_STRBIN_DELAY_VALUE_POS
    }
}
