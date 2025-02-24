def test_legacy_psbt_migration():
    v1_psbt = load_legacy_psbt("v1_transaction.psbt")
    validator = PSBTValidator(version=2)
    
    with pytest.raises(ValidationError):
        validator.validate(v1_psbt)
    
    migrated = migrate_psbt(v1_psbt)
    assert validator.validate(migrated) 