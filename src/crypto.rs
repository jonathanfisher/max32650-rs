#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crypto Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Cipher Control Register."]
    pub cipher_ctrl: CIPHER_CTRL,
    #[doc = "0x08 - HASH Control Register."]
    pub hash_ctrl: HASH_CTRL,
    #[doc = "0x0c - CRC Control Register."]
    pub crc_ctrl: CRC_CTRL,
    #[doc = "0x10 - Crypto DMA Source Address."]
    pub dma_src: DMA_SRC,
    #[doc = "0x14 - Crypto DMA Destination Address."]
    pub dma_dest: DMA_DEST,
    #[doc = "0x18 - Crypto DMA Byte Count."]
    pub dma_cnt: DMA_CNT,
    #[doc = "0x1c - MAA Control Register."]
    pub maa_ctrl: MAA_CTRL,
    #[doc = "0x20 - Crypto Data Input. Data input can be written to this register instead of using the DMA. This register writes to the FIFO. This register occupies four successive words to allow the use of multi-store instructions. Words can be written to any location, they will be placed in the FIFO in the order they are written. The endian swap input control bit affects this register."]
    pub din: [DIN; 4],
    #[doc = "0x30 - Crypto Data Output. Resulting data from cipher calculation. Data is placed in the lower words of these four registers depending on the algorithm. For block cipher modes, this register holds the result of most recent encryption or decryption operation. These registers are affected by the endian swap bits."]
    pub dout: [DOUT; 4],
    #[doc = "0x40 - CRC Polynomial. The polynomial to be used for Galois Field calculations (CRC or LFSR) should be written to this register. This register is affected by the MSB control bit."]
    pub crc_poly: CRC_POLY,
    #[doc = "0x44 - CRC Value. This is the state for the Galois Field. This register holds the result of a CRC calculation or the current state of the LFSR. This register is affected by the MSB control bit."]
    pub crc_val: CRC_VAL,
    #[doc = "0x48 - Pseudo Random Value. Output of the Galois Field shift register. This holds the resulting pseudo-random number if entropy is disabled or true random number if entropy is enabled."]
    pub crc_prng: CRC_PRNG,
    #[doc = "0x4c - Hamming ECC Register."]
    pub ham_ecc: HAM_ECC,
    #[doc = "0x50 - Initial Vector. For block cipher operations that use CBC, CFB, OFB, or CNTR modes, this register holds the initial value. This register is updated with each encryption or decryption operation. This register is affected by the endian swap bits."]
    pub cipher_init: [CIPHER_INIT; 4],
    #[doc = "0x60 - Cipher Key. This register holds the key used for block cipher operations. The lower words are used for block ciphers that use shorter key lengths. This register is affected by the endian swap input control bits."]
    pub cipher_key: [CIPHER_KEY; 8],
    #[doc = "0x80 - This register holds the calculated hash value. This register is affected by the endian swap bits."]
    pub hash_digest: [HASH_DIGEST; 16],
    #[doc = "0xc0 - Message Size. This register holds the lowest 32-bit of message size in bytes."]
    pub hash_msg_sz: [HASH_MSG_SZ; 4],
    #[doc = "0xd0 - MAA Word Size. This register defines the number of bits for a modular operation. This register must be set to a valid value prior to the MAA operation start. Valid values are from 1 to 2048. Invalid values are ignored and will not initiate a MAA operation."]
    pub maa_maws: MAA_MAWS,
}
#[doc = "Crypto Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Crypto Control Register."]
pub mod ctrl;
#[doc = "Cipher Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cipher_ctrl](cipher_ctrl) module"]
pub type CIPHER_CTRL = crate::Reg<u32, _CIPHER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIPHER_CTRL;
#[doc = "`read()` method returns [cipher_ctrl::R](cipher_ctrl::R) reader structure"]
impl crate::Readable for CIPHER_CTRL {}
#[doc = "`write(|w| ..)` method takes [cipher_ctrl::W](cipher_ctrl::W) writer structure"]
impl crate::Writable for CIPHER_CTRL {}
#[doc = "Cipher Control Register."]
pub mod cipher_ctrl;
#[doc = "HASH Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_ctrl](hash_ctrl) module"]
pub type HASH_CTRL = crate::Reg<u32, _HASH_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CTRL;
#[doc = "`read()` method returns [hash_ctrl::R](hash_ctrl::R) reader structure"]
impl crate::Readable for HASH_CTRL {}
#[doc = "`write(|w| ..)` method takes [hash_ctrl::W](hash_ctrl::W) writer structure"]
impl crate::Writable for HASH_CTRL {}
#[doc = "HASH Control Register."]
pub mod hash_ctrl;
#[doc = "CRC Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctrl](crc_ctrl) module"]
pub type CRC_CTRL = crate::Reg<u32, _CRC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CTRL;
#[doc = "`read()` method returns [crc_ctrl::R](crc_ctrl::R) reader structure"]
impl crate::Readable for CRC_CTRL {}
#[doc = "`write(|w| ..)` method takes [crc_ctrl::W](crc_ctrl::W) writer structure"]
impl crate::Writable for CRC_CTRL {}
#[doc = "CRC Control Register."]
pub mod crc_ctrl;
#[doc = "Crypto DMA Source Address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_src](dma_src) module"]
pub type DMA_SRC = crate::Reg<u32, _DMA_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SRC;
#[doc = "`read()` method returns [dma_src::R](dma_src::R) reader structure"]
impl crate::Readable for DMA_SRC {}
#[doc = "`write(|w| ..)` method takes [dma_src::W](dma_src::W) writer structure"]
impl crate::Writable for DMA_SRC {}
#[doc = "Crypto DMA Source Address."]
pub mod dma_src;
#[doc = "Crypto DMA Destination Address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dest](dma_dest) module"]
pub type DMA_DEST = crate::Reg<u32, _DMA_DEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DEST;
#[doc = "`read()` method returns [dma_dest::R](dma_dest::R) reader structure"]
impl crate::Readable for DMA_DEST {}
#[doc = "`write(|w| ..)` method takes [dma_dest::W](dma_dest::W) writer structure"]
impl crate::Writable for DMA_DEST {}
#[doc = "Crypto DMA Destination Address."]
pub mod dma_dest;
#[doc = "Crypto DMA Byte Count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cnt](dma_cnt) module"]
pub type DMA_CNT = crate::Reg<u32, _DMA_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CNT;
#[doc = "`read()` method returns [dma_cnt::R](dma_cnt::R) reader structure"]
impl crate::Readable for DMA_CNT {}
#[doc = "`write(|w| ..)` method takes [dma_cnt::W](dma_cnt::W) writer structure"]
impl crate::Writable for DMA_CNT {}
#[doc = "Crypto DMA Byte Count."]
pub mod dma_cnt;
#[doc = "MAA Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maa_ctrl](maa_ctrl) module"]
pub type MAA_CTRL = crate::Reg<u32, _MAA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAA_CTRL;
#[doc = "`read()` method returns [maa_ctrl::R](maa_ctrl::R) reader structure"]
impl crate::Readable for MAA_CTRL {}
#[doc = "`write(|w| ..)` method takes [maa_ctrl::W](maa_ctrl::W) writer structure"]
impl crate::Writable for MAA_CTRL {}
#[doc = "MAA Control Register."]
pub mod maa_ctrl;
#[doc = "Crypto Data Input. Data input can be written to this register instead of using the DMA. This register writes to the FIFO. This register occupies four successive words to allow the use of multi-store instructions. Words can be written to any location, they will be placed in the FIFO in the order they are written. The endian swap input control bit affects this register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din](din) module"]
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
#[doc = "`write(|w| ..)` method takes [din::W](din::W) writer structure"]
impl crate::Writable for DIN {}
#[doc = "Crypto Data Input. Data input can be written to this register instead of using the DMA. This register writes to the FIFO. This register occupies four successive words to allow the use of multi-store instructions. Words can be written to any location, they will be placed in the FIFO in the order they are written. The endian swap input control bit affects this register."]
pub mod din;
#[doc = "Crypto Data Output. Resulting data from cipher calculation. Data is placed in the lower words of these four registers depending on the algorithm. For block cipher modes, this register holds the result of most recent encryption or decryption operation. These registers are affected by the endian swap bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout](dout) module"]
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
#[doc = "`read()` method returns [dout::R](dout::R) reader structure"]
impl crate::Readable for DOUT {}
#[doc = "Crypto Data Output. Resulting data from cipher calculation. Data is placed in the lower words of these four registers depending on the algorithm. For block cipher modes, this register holds the result of most recent encryption or decryption operation. These registers are affected by the endian swap bits."]
pub mod dout;
#[doc = "CRC Polynomial. The polynomial to be used for Galois Field calculations (CRC or LFSR) should be written to this register. This register is affected by the MSB control bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_poly](crc_poly) module"]
pub type CRC_POLY = crate::Reg<u32, _CRC_POLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_POLY;
#[doc = "`read()` method returns [crc_poly::R](crc_poly::R) reader structure"]
impl crate::Readable for CRC_POLY {}
#[doc = "`write(|w| ..)` method takes [crc_poly::W](crc_poly::W) writer structure"]
impl crate::Writable for CRC_POLY {}
#[doc = "CRC Polynomial. The polynomial to be used for Galois Field calculations (CRC or LFSR) should be written to this register. This register is affected by the MSB control bit."]
pub mod crc_poly;
#[doc = "CRC Value. This is the state for the Galois Field. This register holds the result of a CRC calculation or the current state of the LFSR. This register is affected by the MSB control bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_val](crc_val) module"]
pub type CRC_VAL = crate::Reg<u32, _CRC_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_VAL;
#[doc = "`read()` method returns [crc_val::R](crc_val::R) reader structure"]
impl crate::Readable for CRC_VAL {}
#[doc = "`write(|w| ..)` method takes [crc_val::W](crc_val::W) writer structure"]
impl crate::Writable for CRC_VAL {}
#[doc = "CRC Value. This is the state for the Galois Field. This register holds the result of a CRC calculation or the current state of the LFSR. This register is affected by the MSB control bit."]
pub mod crc_val;
#[doc = "Pseudo Random Value. Output of the Galois Field shift register. This holds the resulting pseudo-random number if entropy is disabled or true random number if entropy is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_prng](crc_prng) module"]
pub type CRC_PRNG = crate::Reg<u32, _CRC_PRNG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_PRNG;
#[doc = "`read()` method returns [crc_prng::R](crc_prng::R) reader structure"]
impl crate::Readable for CRC_PRNG {}
#[doc = "Pseudo Random Value. Output of the Galois Field shift register. This holds the resulting pseudo-random number if entropy is disabled or true random number if entropy is enabled."]
pub mod crc_prng;
#[doc = "Hamming ECC Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ham_ecc](ham_ecc) module"]
pub type HAM_ECC = crate::Reg<u32, _HAM_ECC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAM_ECC;
#[doc = "`read()` method returns [ham_ecc::R](ham_ecc::R) reader structure"]
impl crate::Readable for HAM_ECC {}
#[doc = "`write(|w| ..)` method takes [ham_ecc::W](ham_ecc::W) writer structure"]
impl crate::Writable for HAM_ECC {}
#[doc = "Hamming ECC Register."]
pub mod ham_ecc;
#[doc = "Initial Vector. For block cipher operations that use CBC, CFB, OFB, or CNTR modes, this register holds the initial value. This register is updated with each encryption or decryption operation. This register is affected by the endian swap bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cipher_init](cipher_init) module"]
pub type CIPHER_INIT = crate::Reg<u32, _CIPHER_INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIPHER_INIT;
#[doc = "`read()` method returns [cipher_init::R](cipher_init::R) reader structure"]
impl crate::Readable for CIPHER_INIT {}
#[doc = "`write(|w| ..)` method takes [cipher_init::W](cipher_init::W) writer structure"]
impl crate::Writable for CIPHER_INIT {}
#[doc = "Initial Vector. For block cipher operations that use CBC, CFB, OFB, or CNTR modes, this register holds the initial value. This register is updated with each encryption or decryption operation. This register is affected by the endian swap bits."]
pub mod cipher_init;
#[doc = "Cipher Key. This register holds the key used for block cipher operations. The lower words are used for block ciphers that use shorter key lengths. This register is affected by the endian swap input control bits.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cipher_key](cipher_key) module"]
pub type CIPHER_KEY = crate::Reg<u32, _CIPHER_KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIPHER_KEY;
#[doc = "`write(|w| ..)` method takes [cipher_key::W](cipher_key::W) writer structure"]
impl crate::Writable for CIPHER_KEY {}
#[doc = "Cipher Key. This register holds the key used for block cipher operations. The lower words are used for block ciphers that use shorter key lengths. This register is affected by the endian swap input control bits."]
pub mod cipher_key;
#[doc = "This register holds the calculated hash value. This register is affected by the endian swap bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_digest](hash_digest) module"]
pub type HASH_DIGEST = crate::Reg<u32, _HASH_DIGEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIGEST;
#[doc = "`read()` method returns [hash_digest::R](hash_digest::R) reader structure"]
impl crate::Readable for HASH_DIGEST {}
#[doc = "`write(|w| ..)` method takes [hash_digest::W](hash_digest::W) writer structure"]
impl crate::Writable for HASH_DIGEST {}
#[doc = "This register holds the calculated hash value. This register is affected by the endian swap bits."]
pub mod hash_digest;
#[doc = "Message Size. This register holds the lowest 32-bit of message size in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_msg_sz](hash_msg_sz) module"]
pub type HASH_MSG_SZ = crate::Reg<u32, _HASH_MSG_SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_MSG_SZ;
#[doc = "`read()` method returns [hash_msg_sz::R](hash_msg_sz::R) reader structure"]
impl crate::Readable for HASH_MSG_SZ {}
#[doc = "`write(|w| ..)` method takes [hash_msg_sz::W](hash_msg_sz::W) writer structure"]
impl crate::Writable for HASH_MSG_SZ {}
#[doc = "Message Size. This register holds the lowest 32-bit of message size in bytes."]
pub mod hash_msg_sz;
#[doc = "MAA Word Size. This register defines the number of bits for a modular operation. This register must be set to a valid value prior to the MAA operation start. Valid values are from 1 to 2048. Invalid values are ignored and will not initiate a MAA operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maa_maws](maa_maws) module"]
pub type MAA_MAWS = crate::Reg<u32, _MAA_MAWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAA_MAWS;
#[doc = "`read()` method returns [maa_maws::R](maa_maws::R) reader structure"]
impl crate::Readable for MAA_MAWS {}
#[doc = "`write(|w| ..)` method takes [maa_maws::W](maa_maws::W) writer structure"]
impl crate::Writable for MAA_MAWS {}
#[doc = "MAA Word Size. This register defines the number of bits for a modular operation. This register must be set to a valid value prior to the MAA operation start. Valid values are from 1 to 2048. Invalid values are ignored and will not initiate a MAA operation."]
pub mod maa_maws;
