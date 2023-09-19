// @generated
/// Implement `DataProvider<CanonicalCompositionsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_normalizer_comp_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_NORMALIZER_COMP_V1: &'static <icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::DataMarker>::Yokeable = &icu_normalizer::provider::CanonicalCompositionsV1 { canonical_compositions: icu_collections::char16trie::Char16Trie { data: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\08\0B\x03\x80\x02\xC2\x0CA\x01\xDF\r\x14\x01\x9A0k\0\x9A0\x0B\0\x04\xD8 \0\x05\xD8E\0\x06\xD82\x000\xDD\x06\xD85\xDD\x01\xC08\x19\t\0\xCF0\n\0\xCF0\xD1\xB0\xD20\xD4\xB0\xD50\xD7\xB0\xD80\xDA\xB0\xDB0\xDD\xB0o0q\xB0r0t\xB0u0w\xB0x0z\xB0{0}\xB0\x03\0\xBA\xDC\n\0'\xDD\x14\0>\xDF\x1B\0W\xDF1\0\x04\xD8G\xDF\x01\xC0L\x130\0\x04\xD8\x02\0\x99\xDC\x01\xC0\x9A\x10\x9B\xDC\x01\xC0\x9C\x10\xA5\xDC\x01\xC0\xAB\x100\0\x04\xD8\x01\x001\xDD\x01\xC0.\x112\xDD\x01\xC0/\x111\0\x04\xD8G\xDF\x01\xC0K\x13\x03\0\xB0\xDC\x0E\0\xBA\xDC\x11\0\xBD\xDC\x14\0\xAF\xDD0\0\x05\xD8\x01\0\xB8\xDD\x01\xC0\xBA\x15\xB9\xDD\x01\xC0\xBB\x151\0\x05\xD8\xB9\xDC\x01\xC0\xBC\x141\0\x05\xD8\xB9\xDC\x01\xC0\xBB\x141\0\x05\xD8\xB9\xDC\x01\xC0\xBE\x14\xDF\r\x84\0.\x10\x85\x005\x1B\x86\0\x990/\0\xAD0>\0\xC60\x1E\0\xDB0\x0E\0\xF10\x06\0\xF10\xF9\xB0\xF20\xFA\xB0\xFD0\xFE\xB0\xDB0\xDC\xB0\xEF0\xF7\xB0\xF00\xF8\xB0\xD20\x06\0\xD20\xD3\xB0\xD50\xD6\xB0\xD80\xD9\xB0\xC60\xC7\xB0\xC80\xC9\xB0\xCF0\xD0\xB0\xB90\x0E\0\xBF0\x06\0\xBF0\xC0\xB0\xC10\xC2\xB0\xC40\xC5\xB0\xB90\xBA\xB0\xBB0\xBC\xB0\xBD0\xBE\xB0\xB30\x06\0\xB30\xB4\xB0\xB50\xB6\xB0\xB70\xB8\xB0\xAD0\xAE\xB0\xAF0\xB0\xB0\xB10\xB2\xB0a0\x1E\0u0\x0E\0\x9D0\x06\0\x9D0\x9E\xB0\xA60\xF4\xB0\xAB0\xAC\xB0u0v\xB0x0y\xB0{0|\xB0h0\x06\0h0i\xB0o0p\xB0r0s\xB0a0b\xB0d0e\xB0f0g\xB0U0\x0E\0[0\x06\0[0\\\xB0]0^\xB0_0`\xB0U0V\xB0W0X\xB0Y0Z\xB0O0\x06\0O0P\xB0Q0R\xB0S0T\xB0F0\x94\xB0K0L\xB0M0N\xB00\0\xD9\r\xDE\x8D0\0%\x10&\x90\n\0\x11\x1B\x0E\0>\x1B\x06\0>\x1B@\x9B?\x1BA\x9BB\x1BC\x9B\x11\x1B\x12\x9B:\x1B;\x9B<\x1B=\x9B\x05\x1B\x06\x9B\x07\x1B\x08\x9B\t\x1B\n\x9B\x0B\x1B\x0C\x9B\r\x1B\x0E\x9B>\r\x17\0>\r\x08\0W\r\x0B\0\xCA\r\x0C\0\xCF\r0\0\xD9\r\xDC\x8D\x01\0F\rJ\x8DG\rK\x8D0\0F\rL\x8D\x01\0\xD9\r\xDA\x8D\xDC\r\xDD\x8D\xC2\x0C\x06\0\xD5\x0C\x07\0\xD6\x0C0\0\xC6\x0C\xC8\x8C0\0\xC6\x0C\xCA\x8C\x02\0\xBF\x0C\xC0\x8C\xC6\x0C\xC7\x8C\xCA\x0C\xCB\x8C\xD7\t'\0W\x0B\x17\0W\x0B\x08\0\xBE\x0B\t\0\xD7\x0B\x0C\0V\x0C0\0F\x0CH\x8C0\0G\x0BL\x8B\x01\0\xC6\x0B\xCA\x8B\xC7\x0B\xCB\x8B\x01\0\x92\x0B\x94\x8B\xC6\x0B\xCC\x8B\xD7\t\x06\0>\x0B\x07\0V\x0B0\0G\x0BH\x8B0\0\xC7\t\xCC\x890\0G\x0BK\x8BT\x06#\0T\x06\x08\0U\x06\x15\0<\t\x16\0\xBE\t0\0\xC7\t\xCB\x89\x05\0\xC1\x06\x06\0\xC1\x06\xC2\x86\xD2\x06\xD3\x86\xD5\x06\xC0\x86'\x06#\x86H\x06$\x86J\x06&\x860\0'\x06%\x86\x02\0(\t)\x890\t1\x893\t4\x89B\x03\x06\0E\x03M\0S\x060\0'\x06\"\x86\x1C\0(\x1F$\0Q\x1F\x12\0h\x1F\x08\0h\x1Fn\x9Fi\x1Fo\x9F\xBF\x1F\xCF\x9F\xFE\x1F\xDF\x9FQ\x1FW\x9FY\x1F_\x9F`\x1Ff\x9Fa\x1Fg\x9F1\x1F\x08\x001\x1F7\x9F8\x1F>\x9F9\x1F?\x9FP\x1FV\x9F(\x1F.\x9F)\x1F/\x9F0\x1F6\x9F\xCB\x03\x10\0\x08\x1F\x08\0\x08\x1F\x0E\x9F\t\x1F\x0F\x9F \x1F&\x9F!\x1F'\x9F\xCB\x03\xE7\x9F\0\x1F\x06\x9F\x01\x1F\x07\x9F\xB9\x03\x08\0\xB9\x03\xD6\x9F\xC5\x03\xE6\x9F\xC9\x03\xF6\x9F\xCA\x03\xD7\x9F\xA8\0\xC1\x9F\xB1\x03\xB6\x9F\xB7\x03\xC6\x9F\0\0>\0&\x1FN\0f\x1F&\0n\x1F\x12\0|\x1F\x08\0|\x1F\xF2\x9F\xB6\x1F\xB7\x9F\xC6\x1F\xC7\x9F\xF6\x1F\xF7\x9Fn\x1F\xAE\x9Fo\x1F\xAF\x9Fp\x1F\xB2\x9Ft\x1F\xC2\x9Fj\x1F\x08\0j\x1F\xAA\x9Fk\x1F\xAB\x9Fl\x1F\xAC\x9Fm\x1F\xAD\x9Ff\x1F\xA6\x9Fg\x1F\xA7\x9Fh\x1F\xA8\x9Fi\x1F\xA9\x9F.\x1F\x12\0b\x1F\x08\0b\x1F\xA2\x9Fc\x1F\xA3\x9Fd\x1F\xA4\x9Fe\x1F\xA5\x9F.\x1F\x9E\x9F/\x1F\x9F\x9F`\x1F\xA0\x9Fa\x1F\xA1\x9F*\x1F\x08\0*\x1F\x9A\x9F+\x1F\x9B\x9F,\x1F\x9C\x9F-\x1F\x9D\x9F&\x1F\x96\x9F'\x1F\x97\x9F(\x1F\x98\x9F)\x1F\x99\x9F\x06\x1F&\0\x0E\x1F\x12\0\"\x1F\x08\0\"\x1F\x92\x9F#\x1F\x93\x9F$\x1F\x94\x9F%\x1F\x95\x9F\x0E\x1F\x8E\x9F\x0F\x1F\x8F\x9F \x1F\x90\x9F!\x1F\x91\x9F\n\x1F\x08\0\n\x1F\x8A\x9F\x0B\x1F\x8B\x9F\x0C\x1F\x8C\x9F\r\x1F\x8D\x9F\x06\x1F\x86\x9F\x07\x1F\x87\x9F\x08\x1F\x88\x9F\t\x1F\x89\x9F\xC9\x03\x12\0\x02\x1F\x08\0\x02\x1F\x82\x9F\x03\x1F\x83\x9F\x04\x1F\x84\x9F\x05\x1F\x85\x9F\xC9\x03\xF3\x9F\xCE\x03\xF4\x9F\0\x1F\x80\x9F\x01\x1F\x81\x9F\xAC\x03\x08\0\xAC\x03\xB4\x9F\xAE\x03\xC4\x9F\xB1\x03\xB3\x9F\xB7\x03\xC3\x9F\x91\x03\xBC\x9F\x97\x03\xCC\x9F\xA9\x03\xFC\x9F\x13\x03\x04\x02'\x03'\x01.\x03\xB3\0.\x03t\x000\x03w\x001\x03\x84\08\x03+\0r\"6\0\x87\"\x1A\0\xA9\"\x0E\0\xB3\"\x06\0\xB3\"\xEB\xA2\xB4\"\xEC\xA2\xB5\"\xED\xA2\xA9\"\xAE\xA2\xAB\"\xAF\xA2\xB2\"\xEA\xA2\x87\"\x89\xA2\x91\"\xE2\xA2\x92\"\xE3\xA2\xA2\"\xAC\xA2\xA8\"\xAD\xA2{\"\x0E\0\x82\"\x06\0\x82\"\x84\xA2\x83\"\x85\xA2\x86\"\x88\xA2{\"\x81\xA2|\"\xE0\xA2}\"\xE1\xA2r\"t\xA2s\"u\xA2v\"x\xA2w\"y\xA2z\"\x80\xA2\x0B\"\x1A\0E\"\x0E\0a\"\x06\0a\"b\xA2d\"p\xA2e\"q\xA2E\"G\xA2H\"I\xA2M\"m\xA2\x0B\"\x0C\xA2#\"$\xA2%\"&\xA2<\"A\xA2C\"D\xA2\x94!\x0E\0\xD4!\x06\0\xD4!\xCE\xA1\x03\"\x04\xA2\x08\"\t\xA2\x94!\xAE\xA1\xD0!\xCD\xA1\xD2!\xCF\xA1<\0n\xA2=\0`\xA2>\0o\xA2\x90!\x9A\xA1\x92!\x9B\xA1\x01\0H\0*\x9Eh\0+\x9E\x05\0e\0\x06\0e\0\x1B\x9Ei\0-\x9Eu\0u\x9EE\0\x1A\x9EI\0,\x9EU\0t\x9E\x10\0b\0\x14\0l\0\n\0l\0;\x9En\0I\x9Er\0_\x9Et\0o\x9Ez\0\x95\x9Eb\0\x07\x9Ed\0\x0F\x9Eh\0\x96\x9Ek\x005\x9EN\0\x08\0N\0H\x9ER\0^\x9ET\0n\x9EZ\0\x94\x9EB\0\x06\x9ED\0\x0E\x9EK\x004\x9EL\0:\x9E'\x03\"\0(\x03W\0-\x03\x0B\0d\0\x0E\0n\0\x06\0n\0K\x9Et\0q\x9Eu\0w\x9Ed\0\x13\x9Ee\0\x19\x9El\0=\x9EN\0\x06\0N\0J\x9ET\0p\x9EU\0v\x9ED\0\x12\x9EE\0\x18\x9EL\0<\x9E\x15\0c\0\x1A\0k\0\x0E\0r\0\x06\0r\0W\x81s\0_\x81t\0c\x81k\x007\x81l\0<\x81n\0F\x81c\0\xE7\x80d\0\x11\x9Ee\0)\x82g\0#\x81h\0)\x9EK\0\x0E\0R\0\x06\0R\0V\x81S\0^\x81T\0b\x81K\x006\x81L\0;\x81N\0E\x81C\0\xC7\x80D\0\x10\x9EE\0(\x82G\0\"\x81H\0(\x9E\t\0a\0\n\0a\0\x05\x81e\0\x19\x81i\0/\x81o\0\xEB\x81u\0s\x81A\0\x04\x81E\0\x18\x81I\0.\x81O\0\xEA\x81U\0r\x81#\x03\x81\0#\x03\x0E\0$\x03s\0%\x03v\0&\x03\x03\0S\0\x18\x82T\0\x1A\x82s\0\x19\x82t\0\x1B\x82)\0d\x002\0s\0\x1A\0y\0\x0E\0\xA1\x01\x06\0\xA1\x01\xE3\x9E\xAF\x01\xF0\x9E\xB0\x01\xF1\x9Ey\0\xF5\x9Ez\0\x93\x9E\xA0\x01\xE2\x9Es\0c\x9Et\0m\x9Eu\0\xE5\x9Ev\0\x7F\x9Ew\0\x89\x9El\0\n\0l\x007\x9Em\0C\x9En\0G\x9Eo\0\xCD\x9Er\0[\x9Ed\0\r\x9Ee\0\xB9\x9Eh\0%\x9Ei\0\xCB\x9Ek\x003\x9EO\0\x1A\0V\0\x0E\0Z\0\x06\0Z\0\x92\x9Ea\0\xA1\x9Eb\0\x05\x9EV\0~\x9EW\0\x88\x9EY\0\xF4\x9EO\0\xCC\x9ER\0Z\x9ES\0b\x9ET\0l\x9EU\0\xE4\x9EI\0\n\0I\0\xCA\x9EK\x002\x9EL\x006\x9EM\0B\x9EN\0F\x9EA\0\xA0\x9EB\0\x04\x9ED\0\x0C\x9EE\0\xB8\x9EH\0$\x9E\x01\0U\0r\x9Eu\0s\x9E\x01\0A\0\0\x9Ea\0\x01\x9E\x13\x03\x0C\0\x14\x03-\0\x1B\x03\x03\0O\0\xA0\x81U\0\xAF\x81o\0\xA1\x81u\0\xB0\x81\r\0\xB5\x03\x10\0\xBF\x03\x08\0\xBF\x03@\x9F\xC1\x03\xE4\x9F\xC5\x03P\x9F\xC9\x03`\x9F\xB5\x03\x10\x9F\xB7\x03 \x9F\xB9\x030\x9F\x99\x03\x08\0\x99\x038\x9F\x9F\x03H\x9F\xA9\x03h\x9F\xB1\x03\0\x9F\x91\x03\x08\x9F\x95\x03\x18\x9F\x97\x03(\x9F\x0F\0\xB1\x03\x12\0\xBF\x03\x08\0\xBF\x03A\x9F\xC1\x03\xE5\x9F\xC5\x03Q\x9F\xC9\x03a\x9F\xB1\x03\x01\x9F\xB5\x03\x11\x9F\xB7\x03!\x9F\xB9\x031\x9F\x9F\x03\x08\0\x9F\x03I\x9F\xA1\x03\xEC\x9F\xA5\x03Y\x9F\xA9\x03i\x9F\x91\x03\t\x9F\x95\x03\x19\x9F\x97\x03)\x9F\x99\x039\x9F\x08\x03\x92\x01\x0B\x03\xB1\0\x0B\x03$\0\x0C\x031\0\x0F\x03\x88\0\x11\x03\x0B\0a\0\x0E\0o\0\x06\0o\0\x0F\x82r\0\x13\x82u\0\x17\x82a\0\x03\x82e\0\x07\x82i\0\x0B\x82O\0\x06\0O\0\x0E\x82R\0\x12\x82U\0\x16\x82A\0\x02\x82E\0\x06\x82I\0\n\x82\x05\0u\0\x06\0u\0q\x81#\x04\xF2\x84C\x04\xF3\x84O\0P\x81U\0p\x81o\0Q\x81$\0d\0,\0o\0\x16\0z\0\n\0z\0~\x81\xDC\0\xD9\x81\xFC\0\xDA\x81\xB7\x01\xEE\x81\x92\x02\xEF\x81o\0\xD2\x81r\0Y\x81s\0a\x81t\0e\x81u\0\xD4\x81i\0\n\0i\0\xD0\x81j\0\xF0\x81k\0\xE9\x81l\0>\x81n\0H\x81d\0\x0F\x81e\0\x1B\x81g\0\xE7\x81h\0\x1F\x82N\0\x14\0T\0\n\0T\0d\x81U\0\xD3\x81Z\0}\x81a\0\xCE\x81c\0\r\x81N\0G\x81O\0\xD1\x81R\0X\x81S\0`\x81G\0\n\0G\0\xE6\x81H\0\x1E\x82I\0\xCF\x81K\0\xE8\x81L\0=\x81A\0\xCD\x81C\0\x0C\x81D\0\x0E\x81E\0\x1A\x81\r\0e\0\x10\0r\0\x08\0r\0\x11\x82u\0\x15\x82t\x04v\x84u\x04w\x84e\0\x05\x82i\0\t\x82o\0\r\x82O\0\x08\0O\0\x0C\x82R\0\x10\x82U\0\x14\x82a\0\x01\x82A\0\0\x82E\0\x04\x82I\0\x08\x82\x08\x03\x12\0\t\x03\x9C\0\n\x03\x05\0u\0\x06\0u\0o\x81w\0\x98\x9Ey\0\x99\x9EA\0\xC5\x80U\0n\x81a\0\xE5\x80\0\x005\0\xD2\x03D\x005\x04\"\0K\x04\x10\0\xD8\x04\x08\0\xD8\x04\xDA\x84\xD9\x04\xDB\x84\xE8\x04\xEA\x84\xE9\x04\xEB\x84K\x04\xF9\x84M\x04\xED\x84V\x04W\x848\x04\x08\08\x04\xE5\x84>\x04\xE7\x84C\x04\xF1\x84G\x04\xF5\x845\x04Q\x846\x04\xDD\x847\x04\xDF\x84\x18\x04\x10\0'\x04\x08\0'\x04\xF4\x84+\x04\xF8\x84-\x04\xEC\x840\x04\xD3\x84\x18\x04\xE4\x84\x1E\x04\xE6\x84#\x04\xF0\x84\x15\x04\x06\0\x15\x04\x01\x84\x16\x04\xDC\x84\x17\x04\xDE\x84\xD2\x03\xD4\x83\x06\x04\x07\x84\x10\x04\xD2\x84o\0\"\0\xF5\0\x10\0\x99\x03\x08\0\x99\x03\xAA\x83\xA5\x03\xAB\x83\xB9\x03\xCA\x83\xC5\x03\xCB\x83\xF5\0O\x9Ej\x01z\x9Ek\x01{\x9Ew\0\x08\0w\0\x85\x9Ex\0\x8D\x9Ey\0\xFF\x80\xD5\0N\x9Eo\0\xF6\x80t\0\x97\x9Eu\0\xFC\x80W\0\x10\0a\0\x08\0a\0\xE4\x80e\0\xEB\x80h\0'\x9Ei\0\xEF\x80W\0\x84\x9EX\0\x8C\x9EY\0x\x81I\0\x06\0I\0\xCF\x80O\0\xD6\x80U\0\xDC\x80A\0\xC4\x80E\0\xCB\x80H\0&\x9E\x17\0\xC2\0\x1E\0\x02\x01\x0E\0\xA1\x01\x06\0\xA1\x01\xDF\x9E\xAF\x01\xEC\x9E\xB0\x01\xED\x9E\x02\x01\xB2\x9E\x03\x01\xB3\x9E\xA0\x01\xDE\x9E\xE2\0\x06\0\xE2\0\xA9\x9E\xEA\0\xC3\x9E\xF4\0\xD5\x9E\xC2\0\xA8\x9E\xCA\0\xC2\x9E\xD4\0\xD4\x9Ea\0\x0E\0o\0\x06\0o\0\xCF\x9Eu\0\xE7\x9Ey\0\xF7\x9Ea\0\xA3\x9Ee\0\xBB\x9Ei\0\xC9\x9EO\0\x06\0O\0\xCE\x9EU\0\xE6\x9EY\0\xF6\x9EA\0\xA2\x9EE\0\xBA\x9EI\0\xC8\x9E\x03\x03\x83\x01\x03\x03|\0\x04\x03\xC1\0\x06\x03.\x01\x07\x03-\0d\0:\0t\0\x1E\0[\x01\x0E\0\x7F\x01\x06\0\x7F\x01\x9B\x9Eb\x1Eh\x9Ec\x1Ei\x9E[\x01e\x9E`\x01f\x9Ea\x01g\x9Ey\0\x06\0y\0\x8F\x9Ez\0|\x81Z\x01d\x9Et\0k\x9Ew\0\x87\x9Ex\0\x8B\x9Em\0\x0E\0p\0\x06\0p\0W\x9Er\0Y\x9Es\0a\x9Em\0A\x9En\0E\x9Eo\0/\x82d\0\x0B\x9Ee\0\x17\x81f\0\x1F\x9Eg\0!\x81h\0#\x9EO\0\x1E\0X\0\x0E\0a\0\x06\0a\0'\x82b\0\x03\x9Ec\0\x0B\x81X\0\x8A\x9EY\0\x8E\x9EZ\0{\x81S\0\x06\0S\0`\x9ET\0j\x9EW\0\x86\x9EO\0.\x82P\0V\x9ER\0X\x9EF\0\x0E\0I\0\x06\0I\x000\x81M\0@\x9EN\0D\x9EF\0\x1E\x9EG\0 \x81H\0\"\x9EA\0&\x82B\0\x02\x9EC\0\n\x81D\0\n\x9EE\0\x16\x81\x1B\0v\0\"\0\xF4\0\x10\0\xA0\x01\x08\0\xA0\x01\xE0\x9E\xA1\x01\xE1\x9E\xAF\x01\xEE\x9E\xB0\x01\xEF\x9E\xF4\0\xD7\x9E\x02\x01\xB4\x9E\x03\x01\xB5\x9E\xCA\0\x08\0\xCA\0\xC4\x9E\xD4\0\xD6\x9E\xE2\0\xAB\x9E\xEA\0\xC5\x9Ev\0}\x9Ey\0\xF9\x9E\xC2\0\xAA\x9EY\0\x10\0i\0\x08\0i\0)\x81n\0\xF1\x80o\0\xF5\x80u\0i\x81Y\0\xF8\x9Ea\0\xE3\x80e\0\xBD\x9EN\0\x08\0N\0\xD1\x80O\0\xD5\x80U\0h\x81V\0|\x9EA\0\xC3\x80E\0\xBC\x9EI\0(\x81+\0\xF6\x006\0\xB1\x03\x1A\08\x04\x0E\x007\x1E\x06\x007\x1E9\x9EZ\x1E\\\x9E[\x1E]\x9E8\x04\xE3\x84C\x04\xEF\x846\x1E8\x9E\xB1\x03\xB1\x9F\xB9\x03\xD1\x9F\xC5\x03\xE1\x9F\x18\x04\xE2\x84#\x04\xEE\x84'\x02\x0E\0\x91\x03\x06\0\x91\x03\xB9\x9F\x99\x03\xD9\x9F\xA5\x03\xE9\x9F'\x02\xE1\x81.\x020\x82/\x021\x82\xF6\0+\x82\xFC\0\xD6\x81\xEA\x01\xEC\x81\xEB\x01\xED\x81&\x02\xE0\x81o\0\x1A\0\xD5\0\x0E\0\xE4\0\x06\0\xE4\0\xDF\x81\xE6\0\xE3\x81\xF5\0-\x82\xD5\0,\x82\xD6\0*\x82\xDC\0\xD5\x81o\0M\x81u\0k\x81y\x003\x82\xC4\0\xDE\x81\xC6\0\xE2\x81U\0\x0E\0e\0\x06\0e\0\x13\x81g\0!\x9Ei\0+\x81U\0j\x81Y\x002\x82a\0\x01\x81A\0\0\x81E\0\x12\x81G\0 \x9EI\0*\x81O\0L\x81\x1F\0\xA5\x03&\0#\x04\x12\08\x04\x08\08\x049\x84C\x04^\x84\xA0\x1E\xB6\x9E\xA1\x1E\xB7\x9E#\x04\x0E\x840\x04\xD1\x845\x04\xD7\x846\x04\xC2\x84\x10\x04\x08\0\x10\x04\xD0\x84\x15\x04\xD6\x84\x16\x04\xC1\x84\x18\x04\x19\x84\xA5\x03\xE8\x9F\xB1\x03\xB0\x9F\xB9\x03\xD0\x9F\xC5\x03\xE0\x9Fg\0\x12\0(\x02\x08\0(\x02\x1C\x9E)\x02\x1D\x9E\x91\x03\xB8\x9F\x99\x03\xD8\x9Fg\0\x1F\x81i\0-\x81o\0O\x81u\0m\x81O\0\x08\0O\0N\x81U\0l\x81a\0\x03\x81e\0\x15\x81A\0\x02\x81E\0\x14\x81G\0\x1E\x81I\0,\x81\0\x03R\0\x01\x03 \x01\x02\x03\x1F\0g\0&\0y\0\x12\0\xB8\x1E\x08\0\xB8\x1E\xC6\x9E\xB9\x1E\xC7\x9E\xCC\x1E\xD8\x9E\xCD\x1E\xD9\x9Ey\0w\x81z\0\x91\x9E\xA0\x1E\xAC\x9E\xA1\x1E\xAD\x9Eo\0\x08\0o\0\xF4\x80s\0]\x81u\0\xFB\x80w\0u\x81g\0\x1D\x81h\0%\x81i\0\xEE\x80j\x005\x81S\0\x12\0Z\0\x08\0Z\0\x90\x9Ea\0\xE2\x80c\0\t\x81e\0\xEA\x80S\0\\\x81U\0\xDB\x80W\0t\x81Y\0v\x81H\0\x08\0H\0$\x81I\0\xCE\x80J\x004\x81O\0\xD4\x80A\0\xC2\x80C\0\x08\x81E\0\xCA\x80G\0\x1C\x81\0\0S\0\xB1\x03f\0 \x1F2\0H\x1F\x1A\0`\x1F\x0E\0i\x1F\x06\0i\x1Fk\x9F\xBF\x1F\xCD\x9F\xFE\x1F\xDD\x9F`\x1Fb\x9Fa\x1Fc\x9Fh\x1Fj\x9FH\x1FJ\x9FI\x1FK\x9FP\x1FR\x9FQ\x1FS\x9FY\x1F[\x9F1\x1F\n\x001\x1F3\x9F8\x1F:\x9F9\x1F;\x9F@\x1FB\x9FA\x1FC\x9F \x1F\"\x9F!\x1F#\x9F(\x1F*\x9F)\x1F+\x9F0\x1F2\x9F\x18\x04\x1A\0\x08\x1F\x0E\0\x11\x1F\x06\0\x11\x1F\x13\x9F\x18\x1F\x1A\x9F\x19\x1F\x1B\x9F\x08\x1F\n\x9F\t\x1F\x0B\x9F\x10\x1F\x12\x9F\x18\x04\r\x845\x04P\x848\x04]\x84\0\x1F\x02\x9F\x01\x1F\x03\x9F\xC5\x03\n\0\xC5\x03z\x9F\xC9\x03|\x9F\xCA\x03\xD2\x9F\xCB\x03\xE2\x9F\x15\x04\0\x84\xB1\x03p\x9F\xB5\x03r\x9F\xB7\x03t\x9F\xB9\x03v\x9F\xBF\x03x\x9F\xE2\x002\0\xA0\x01\x1A\0\x95\x03\x0E\0\x9F\x03\x06\0\x9F\x03\xF8\x9F\xA5\x03\xEA\x9F\xA9\x03\xFA\x9F\x95\x03\xC8\x9F\x97\x03\xCA\x9F\x99\x03\xDA\x9F\xA0\x01\xDC\x9E\xA1\x01\xDD\x9E\xAF\x01\xEA\x9E\xB0\x01\xEB\x9E\x91\x03\xBA\x9F\x03\x01\n\0\x03\x01\xB1\x9E\x12\x01\x14\x9E\x13\x01\x15\x9EL\x01P\x9EM\x01Q\x9E\xE2\0\xA7\x9E\xEA\0\xC1\x9E\xF4\0\xD3\x9E\xFC\0\xDC\x81\x02\x01\xB0\x9Ei\0\x1A\0y\0\x0E\0\xCA\0\x06\0\xCA\0\xC0\x9E\xD4\0\xD2\x9E\xDC\0\xDB\x81y\0\xF3\x9E\xA8\0\xED\x9F\xC2\0\xA6\x9Ei\0\xEC\x80n\0\xF9\x81o\0\xF2\x80u\0\xF9\x80w\0\x81\x9EU\0\n\0U\0\xD9\x80W\0\x80\x9EY\0\xF2\x9Ea\0\xE0\x80e\0\xE8\x80A\0\xC0\x80E\0\xC8\x80I\0\xCC\x80N\0\xF8\x81O\0\xD2\x80\0\0t\0\x13\x01\x94\0:\x04J\08\x1F$\0Q\x1F\x12\0h\x1F\x08\0h\x1Fl\x9Fi\x1Fm\x9F\xBF\x1F\xCE\x9F\xFE\x1F\xDE\x9FQ\x1FU\x9FY\x1F]\x9F`\x1Fd\x9Fa\x1Fe\x9FA\x1F\x08\0A\x1FE\x9FH\x1FL\x9FI\x1FM\x9FP\x1FT\x9F8\x1F<\x9F9\x1F=\x9F@\x1FD\x9F\x18\x1F\x12\0(\x1F\x08\0(\x1F,\x9F)\x1F-\x9F0\x1F4\x9F1\x1F5\x9F\x18\x1F\x1C\x9F\x19\x1F\x1D\x9F \x1F$\x9F!\x1F%\x9F\x08\x1F\x08\0\x08\x1F\x0C\x9F\t\x1F\r\x9F\x10\x1F\x14\x9F\x11\x1F\x15\x9F:\x04\\\x84\0\x1F\x04\x9F\x01\x1F\x05\x9F\xA5\x03$\0\xC5\x03\x12\0\xD2\x03\x08\0\xD2\x03\xD3\x83\x13\x04\x03\x84\x1A\x04\x0C\x843\x04S\x84\xC5\x03\xCD\x83\xC9\x03\xCE\x83\xCA\x03\x90\x83\xCB\x03\xB0\x83\xB5\x03\x08\0\xB5\x03\xAD\x83\xB7\x03\xAE\x83\xB9\x03\xAF\x83\xBF\x03\xCC\x83\xA5\x03\x8E\x83\xA9\x03\x8F\x83\xB1\x03\xAC\x83\xAF\x01\x10\0\x95\x03\x08\0\x95\x03\x88\x83\x97\x03\x89\x83\x99\x03\x8A\x83\x9F\x03\x8C\x83\xAF\x01\xE8\x9E\xB0\x01\xE9\x9E\x91\x03\x86\x83h\x01\x08\0h\x01x\x9Ei\x01y\x9E\xA0\x01\xDA\x9E\xA1\x01\xDB\x9E\x13\x01\x17\x9EL\x01R\x9EM\x01S\x9Es\0H\0\xD8\0$\0\xEF\0\x12\0\xFC\0\x08\0\xFC\0\xD8\x81\x02\x01\xAE\x9E\x03\x01\xAF\x9E\x12\x01\x16\x9E\xEF\0/\x9E\xF4\0\xD1\x9E\xF5\0M\x9E\xF8\0\xFF\x81\xE5\0\x08\0\xE5\0\xFB\x81\xE6\0\xFD\x81\xE7\0\t\x9E\xEA\0\xBF\x9E\xD8\0\xFE\x81\xDC\0\xD7\x81\xE2\0\xA5\x9E\xC5\0\x10\0\xCA\0\x08\0\xCA\0\xBE\x9E\xCF\0.\x9E\xD4\0\xD0\x9E\xD5\0L\x9E\xC5\0\xFA\x81\xC6\0\xFC\x81\xC7\0\x08\x9Ey\0\x08\0y\0\xFD\x80z\0z\x81\xA8\0\x85\x83\xC2\0\xA4\x9Es\0[\x81u\0\xFA\x80w\0\x83\x9EW\0$\0i\0\x12\0n\0\x08\0n\0D\x81o\0\xF3\x80p\0U\x9Er\0U\x81i\0\xED\x80k\x001\x9El\0:\x81m\0?\x9Ea\0\x08\0a\0\xE1\x80c\0\x07\x81e\0\xE9\x80g\0\xF5\x81W\0\x82\x9EY\0\xDD\x80Z\0y\x81M\0\x10\0P\0\x08\0P\0T\x9ER\0T\x81S\0Z\x81U\0\xDA\x80M\0>\x9EN\0C\x81O\0\xD3\x80G\0\x08\0G\0\xF4\x81I\0\xCD\x80K\x000\x9EL\09\x81A\0\xC1\x80C\0\x06\x81E\0\xC9\x80") } } };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu_normalizer::provider::CanonicalCompositionsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_normalizer::provider::CanonicalCompositionsV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_NORMALIZER_COMP_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
