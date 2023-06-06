// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_from_sb_v1 {
    () => {
        icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
            map: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x1C\0\0\0\0\0\x02\0\x07\0\t\0\x0E\0\x10\0\x12\0\x18\0\x1A\0 \0\"\0$\0&\0+\0-\x004\0;\0@\0B\0K\0M\0P\0R\0T\0V\0[\0]\0b\0ATATermCLCloseCREXExtendFOFormatLELFLOLowerNUNumericOLetterOtherSCSContinueSESepSPSpSTSTermUPUpperXX") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x01\0\x02\0\x02\0\x0B\0\x0C\0\x0C\0\x03\0\x03\0\x06\0\r\0\x04\0\x04\0\x05\0\x05\0\x06\0\0\0\x0E\0\x0E\0\x07\0\x07\0\x08\0\x08\0\t\0\t\0\n\0\n\0\0\0") })
            },
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_propnames_from_sb_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_from_sb_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<SentenceBreakNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_sb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::SentenceBreakNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::SentenceBreakNameToValueV1Marker>, icu_provider::DataError> {
                let lookup = lookup_propnames_from_sb_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}
