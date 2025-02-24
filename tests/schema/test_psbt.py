def test_taproot_schema_enforcement():
    valid_psbt = load_taproot_psbt()
    assert validate_psbt_schema(valid_psbt)

    legacy_psbt = load_legacy_psbt()
    with pytest.raises(ValidationError):
        validate_psbt_schema(legacy_psbt) 