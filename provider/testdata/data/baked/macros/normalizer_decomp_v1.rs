// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_normalizer_decomp_v1 {
    () => {
        icu_normalizer::provider::NonRecursiveDecompositionSupplementV1 { trie: icu_collections::codepointtrie::CodePointTrie::from_parts(icu_collections::codepointtrie::CodePointTrieHeader { high_start: 119296u32, shifted12_high_start: 30u16, index3_null_offset: 15u16, data_null_offset: 0u32, null_value: 0u32, trie_type: icu_collections::codepointtrie::TrieType::Small }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0@\0\0\0\0\0\0\0\0\0\0\0k\0\xA7\0\0\0\0\0\0\0\0\0\0\0\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0E\x01\0\0\0\0\0\x001\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0B\x01\x1B\x01\x1B\x01\x1E\x01\x1B\x01\x1B\x01\x1B\x01<\x01\0\0\x10\0 \x000\0@\0P\0`\0p\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0k\0{\0\x8B\0\x9B\0\xA7\0\xB7\0\xC7\0\xD7\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\xD9\0\xE9\0\xF9\0\t\x01\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\x0E\x01\x1E\x01.\x01>\x01\0\0\x10\0 \x000\0\0\0\x10\0 \x000\0\0\0\x10\0 \x000\x001\x01A\x01Q\x01a\x01i\x01u\x01\x83\x01\x93\x01\x9D\x01\xAD\x01\xBB\x01\xC5\x01\0\0\0\0\xD1\x01\xE1\x01\xF1\x01\x01\x02\x11\x02!\x02/\x02?\x02M\x02]\x02m\x02{\x02\x8B\x02\x9B\x02\xAB\x02\xBB\x02\xCB\x02\xDB\x02\xEA\x02\xFA\x02\n\x03\x1A\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0&\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x006\x03\0\0\0\0\0\0\0\0;\x03K\x03\0\0\0\0\0\0H\0h\0\x84\0\x84\0\x84\0\x84\0\x98\0\x84\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\xB8\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\xD8\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\x0F\0\xEB\0\xEE\xFF") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\0\x04\x03\xFC\0\x04\x03\xDC\0\x01\x03\xFC\0\x01\x03\xDC\0\x0C\x03\xFC\0\x0C\x03\xDC\0\0\x03\xFC\0\0\x03\0\0\0\0\xC4\0\x04\x03\xE4\0\x04\x03&\x02\x04\x03'\x02\x04\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xEA\x01\x04\x03\xEB\x01\x04\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC5\0\x01\x03\xE5\0\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD6\0\x04\x03\xF6\0\x04\x03\xD5\0\x04\x03\xF5\0\x04\x03\0\0\0\0\0\0\0\0.\x02\x04\x03/\x02\x04\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xCA\x03\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xCB\x03\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xCA\x0C\xD5\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDC\r\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC7\0\x01\x03\xE7\0\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x12\x01\0\x03\x13\x01\0\x03\x12\x01\x01\x03\x13\x01\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0(\x02\x06\x03)\x02\x06\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xCF\0\x01\x03\xEF\0\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x006\x1E\x04\x037\x1E\x04\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD5\0\x01\x03\xF5\0\x01\x03\xD5\0\x08\x03\xF5\0\x08\x03L\x01\0\x03M\x01\0\x03L\x01\x01\x03M\x01\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Z\x1E\x04\x03[\x1E\x04\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Z\x01\x07\x03[\x01\x07\x03`\x01\x07\x03a\x01\x07\x03b\x1E\x07\x03c\x1E\x07\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0h\x01\x01\x03i\x01\x01\x03j\x01\x08\x03k\x01\x08\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC2\0\x01\x03\xE2\0\x01\x03\xC2\0\0\x03\xE2\0\0\x03\xC2\0\t\x03\xE2\0\t\x03\xC2\0\x03\x03\xE2\0\x03\x03\xA0\x1E\x02\x03\xA1\x1E\x02\x03\x02\x01\x01\x03\x03\x01\x01\x03\x02\x01\0\x03\x03\x01\0\x03\x02\x01\t\x03\x03\x01\t\x03\x02\x01\x03\x03\x03\x01\x03\x03\xA0\x1E\x06\x03\xA1\x1E\x06\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xCA\0\x01\x03\xEA\0\x01\x03\xCA\0\0\x03\xEA\0\0\x03\xCA\0\t\x03\xEA\0\t\x03\xCA\0\x03\x03\xEA\0\x03\x03\xB8\x1E\x02\x03\xB9\x1E\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD4\0\x01\x03\xF4\0\x01\x03\xD4\0\0\x03\xF4\0\0\x03\xD4\0\t\x03\xF4\0\t\x03\xD4\0\x03\x03\xF4\0\x03\x03\xCC\x1E\x02\x03\xCD\x1E\x02\x03\xA0\x01\x01\x03\xA1\x01\x01\x03\xA0\x01\0\x03\xA1\x01\0\x03\xA0\x01\t\x03\xA1\x01\t\x03\xA0\x01\x03\x03\xA1\x01\x03\x03\xA0\x01#\x03\xA1\x01#\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAF\x01\x01\x03\xB0\x01\x01\x03\xAF\x01\0\x03\xB0\x01\0\x03\xAF\x01\t\x03\xB0\x01\t\x03\xAF\x01\x03\x03\xB0\x01\x03\x03\xAF\x01#\x03\xB0\x01#\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1F\0\x03\x01\x1F\0\x03\0\x1F\x01\x03\x01\x1F\x01\x03\0\x1FB\x03\x01\x1FB\x03\0\0\0\0\0\0\0\0\x08\x1F\0\x03\t\x1F\0\x03\x08\x1F\x01\x03\t\x1F\x01\x03\x08\x1FB\x03\t\x1FB\x03\0\0\0\0\0\0\0\0\x10\x1F\0\x03\x11\x1F\0\x03\x10\x1F\x01\x03\x11\x1F\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18\x1F\0\x03\x19\x1F\0\x03\x18\x1F\x01\x03\x19\x1F\x01\x03\0\0\0\0\0\0\0\0 \x1F\0\x03!\x1F\0\x03 \x1F\x01\x03!\x1F\x01\x03 \x1FB\x03!\x1FB\x03\0\0\0\0\0\0\0\0(\x1F\0\x03)\x1F\0\x03(\x1F\x01\x03)\x1F\x01\x03(\x1FB\x03)\x1FB\x03\0\0\0\0\0\0\0\x000\x1F\0\x031\x1F\0\x030\x1F\x01\x031\x1F\x01\x030\x1FB\x031\x1FB\x03\0\0\0\0\0\0\0\08\x1F\0\x039\x1F\0\x038\x1F\x01\x039\x1F\x01\x038\x1FB\x039\x1FB\x03\0\0\0\0\0\0\0\0@\x1F\0\x03A\x1F\0\x03@\x1F\x01\x03A\x1F\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0H\x1F\0\x03I\x1F\0\x03H\x1F\x01\x03I\x1F\x01\x03\0\0\0\0\0\0\0\0P\x1F\0\x03Q\x1F\0\x03P\x1F\x01\x03Q\x1F\x01\x03P\x1FB\x03Q\x1FB\x03\0\0\0\0\0\0\0\0\0\0\0\0Y\x1F\0\x03\0\0\0\0Y\x1F\x01\x03\0\0\0\0Y\x1FB\x03\0\0\0\0\0\0\0\0`\x1F\0\x03a\x1F\0\x03`\x1F\x01\x03a\x1F\x01\x03`\x1FB\x03a\x1FB\x03\0\0\0\0\0\0\0\0h\x1F\0\x03i\x1F\0\x03h\x1F\x01\x03i\x1F\x01\x03h\x1FB\x03i\x1FB\x03\0\0\0\0\xAC\x03\0\0\xB5\x03\0\x03\xAD\x03\0\0\xB7\x03\0\x03\xAE\x03\0\0\xB9\x03\0\x03\xAF\x03\0\0\xBF\x03\0\x03\xCC\x03\0\0\xC5\x03\0\x03\xCD\x03\0\0\xC9\x03\0\x03\xCE\x03\0\0\0\0\0\0\0\0\0\0\0\x1FE\x03\x01\x1FE\x03\x02\x1FE\x03\x03\x1FE\x03\x04\x1FE\x03\x05\x1FE\x03\x06\x1FE\x03\x07\x1FE\x03\x08\x1FE\x03\t\x1FE\x03\n\x1FE\x03\x0B\x1FE\x03\x0C\x1FE\x03\r\x1FE\x03\x0E\x1FE\x03\x0F\x1FE\x03 \x1FE\x03!\x1FE\x03\"\x1FE\x03#\x1FE\x03$\x1FE\x03%\x1FE\x03&\x1FE\x03'\x1FE\x03(\x1FE\x03)\x1FE\x03*\x1FE\x03+\x1FE\x03,\x1FE\x03-\x1FE\x03.\x1FE\x03/\x1FE\x03`\x1FE\x03a\x1FE\x03b\x1FE\x03c\x1FE\x03d\x1FE\x03e\x1FE\x03f\x1FE\x03g\x1FE\x03h\x1FE\x03i\x1FE\x03j\x1FE\x03k\x1FE\x03l\x1FE\x03m\x1FE\x03n\x1FE\x03o\x1FE\x03\xB1\x03\x06\x03\xB1\x03\x04\x03p\x1FE\x03\xB1\x03E\x03\xAC\x03E\x03\0\0\0\0\xB1\x03B\x03\xB6\x1FE\x03\x91\x03\x06\x03\x91\x03\x04\x03\x91\x03\0\x03\x86\x03\0\0\x91\x03E\x03\0\0\0\0\xB9\x03\0\0\0\0\0\0\xA8\0B\x03t\x1FE\x03\xB7\x03E\x03\xAE\x03E\x03\0\0\0\0\xB7\x03B\x03\xC6\x1FE\x03\x95\x03\0\x03\x88\x03\0\0\x97\x03\0\x03\x89\x03\0\0\x97\x03E\x03\xBF\x1F\0\x03\xBF\x1F\x01\x03\xBF\x1FB\x03\xB9\x03\x06\x03\xB9\x03\x04\x03\xCA\x03\0\x03\x90\x03\0\0\0\0\0\0\0\0\0\0\xB9\x03B\x03\xCA\x03B\x03\x99\x03\x06\x03\x99\x03\x04\x03\x99\x03\0\x03\x8A\x03\0\0\0\0\0\0\xFE\x1F\0\x03\xFE\x1F\x01\x03\xFE\x1FB\x03\xC5\x03\x06\x03\xC5\x03\x04\x03\xCB\x03\0\x03\xB0\x03\0\0\xC1\x03\x13\x03\xC1\x03\x14\x03\xC5\x03B\x03\xCB\x03B\x03\xA5\x03\x06\x03\xA5\x03\x04\x03\xA5\x03\0\x03\x8E\x03\0\0\xA1\x03\x14\x03\xA8\0\0\x03\x85\x03\0\0`\0\0\0\0\0\0\0\0\0\0\0|\x1FE\x03\xC9\x03E\x03\xCE\x03E\x03\0\0\0\0\xC9\x03B\x03\xF6\x1FE\x03\x9F\x03\0\x03\x8C\x03\0\0\xA9\x03\0\x03\x8F\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0I\xFB\xC1\x05I\xFB\xC2\x05\0\0\0\0\0\0\0\0\0\0\x11\0\0\0\x0F\0\0\0\r\0\0\0\x0B\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\x05\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, 0u32), scalars24: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xBC\xD1\x01o\xD1\x01\xBB\xD1\x01o\xD1\x01\xBC\xD1\x01n\xD1\x01\xBB\xD1\x01n\xD1\x01_\xD1\x01r\xD1\x01_\xD1\x01q\xD1\x01_\xD1\x01p\xD1\x01_\xD1\x01o\xD1\x01_\xD1\x01n\xD1\x01") } }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_normalizer_decomp_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::DataMarker>::Yokeable = singleton_normalizer_decomp_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<NonRecursiveDecompositionSupplementV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_normalizer_decomp_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>, icu_provider::DataError> {
                let lookup = lookup_normalizer_decomp_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
