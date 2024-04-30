// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

#![allow(missing_docs)]

//! PostgreSQL OID constants.

/// The OID 0 is reserved for any place in the code that requires an OID, however the OID should
/// never actually be looked at or used.
pub const INVALID_OID: u32 = 0;

// PostgreSQL builtin type OIDs
pub const TYPE_ANY_OID: u32 = 2276;
pub const TYPE_ANYARRAY_OID: u32 = 2277;
pub const TYPE_ANYCOMPATIBLE_OID: u32 = 5077;
pub const TYPE_ANYCOMPATIBLEARRAY_OID: u32 = 5078;
pub const TYPE_ANYCOMPATIBLENONARRAY_OID: u32 = 5079;
pub const TYPE_ANYELEMENT_OID: u32 = 2283;
pub const TYPE_ANYNONARRAY_OID: u32 = 2776;
pub const TYPE_BOOL_ARRAY_OID: u32 = 1000;
pub const TYPE_BOOL_OID: u32 = 16;
pub const TYPE_BPCHAR_ARRAY_OID: u32 = 1014;
pub const TYPE_BPCHAR_OID: u32 = 1042;
pub const TYPE_BYTEA_ARRAY_OID: u32 = 1001;
pub const TYPE_BYTEA_OID: u32 = 17;
pub const TYPE_CHAR_ARRAY_OID: u32 = 1002;
pub const TYPE_CHAR_OID: u32 = 18;
pub const TYPE_DATE_ARRAY_OID: u32 = 1182;
pub const TYPE_DATE_OID: u32 = 1082;
pub const TYPE_FLOAT4_ARRAY_OID: u32 = 1021;
pub const TYPE_FLOAT4_OID: u32 = 700;
pub const TYPE_FLOAT8_ARRAY_OID: u32 = 1022;
pub const TYPE_FLOAT8_OID: u32 = 701;
pub const TYPE_INT2_ARRAY_OID: u32 = 1005;
pub const TYPE_INT2_OID: u32 = 21;
pub const TYPE_INT2_VECTOR_ARRAY_OID: u32 = 1006;
pub const TYPE_INT2_VECTOR_OID: u32 = 22;
pub const TYPE_INT4_ARRAY_OID: u32 = 1007;
pub const TYPE_INT4_OID: u32 = 23;
pub const TYPE_INT8_ARRAY_OID: u32 = 1016;
pub const TYPE_INT8_OID: u32 = 20;
pub const TYPE_INTERVAL_ARRAY_OID: u32 = 1187;
pub const TYPE_INTERVAL_OID: u32 = 1186;
pub const TYPE_JSONB_ARRAY_OID: u32 = 3807;
pub const TYPE_JSONB_OID: u32 = 3802;
pub const TYPE_LIST_OID_OID: u32 = 16_384;
pub const TYPE_NAME_ARRAY_OID: u32 = 1003;
pub const TYPE_NAME_OID: u32 = 19;
pub const TYPE_NUMERIC_ARRAY_OID: u32 = 1231;
pub const TYPE_NUMERIC_OID: u32 = 1700;
pub const TYPE_OID_ARRAY_OID: u32 = 1028;
pub const TYPE_OID_OID: u32 = 26;
pub const TYPE_RECORD_ARRAY_OID: u32 = 2287;
pub const TYPE_RECORD_OID: u32 = 2249;
pub const TYPE_REGCLASS_ARRAY_OID: u32 = 2210;
pub const TYPE_REGCLASS_OID: u32 = 2205;
pub const TYPE_REGPROC_ARRAY_OID: u32 = 1008;
pub const TYPE_REGPROC_OID: u32 = 24;
pub const TYPE_REGTYPE_ARRAY_OID: u32 = 2211;
pub const TYPE_REGTYPE_OID: u32 = 2206;
pub const TYPE_TEXT_ARRAY_OID: u32 = 1009;
pub const TYPE_TEXT_OID: u32 = 25;
pub const TYPE_TIME_ARRAY_OID: u32 = 1183;
pub const TYPE_TIME_OID: u32 = 1083;
pub const TYPE_TIMESTAMP_ARRAY_OID: u32 = 1115;
pub const TYPE_TIMESTAMP_OID: u32 = 1114;
pub const TYPE_TIMESTAMPTZ_ARRAY_OID: u32 = 1185;
pub const TYPE_TIMESTAMPTZ_OID: u32 = 1184;
pub const TYPE_UUID_ARRAY_OID: u32 = 2951;
pub const TYPE_UUID_OID: u32 = 2950;
pub const TYPE_VARCHAR_ARRAY_OID: u32 = 1015;
pub const TYPE_VARCHAR_OID: u32 = 1043;
pub const TYPE_INT4RANGE_OID: u32 = 3904;
pub const TYPE_INT4RANGE_ARRAY_OID: u32 = 3905;
pub const TYPE_ANYRANGE_OID: u32 = 3831;
pub const TYPE_ANYCOMPATIBLERANGE_OID: u32 = 5080;
pub const TYPE_INT8RANGE_OID: u32 = 3926;
pub const TYPE_INT8RANGE_ARRAY_OID: u32 = 3927;
pub const TYPE_DATERANGE_OID: u32 = 3912;
pub const TYPE_DATERANGE_ARRAY_OID: u32 = 3913;
pub const TYPE_NUMRANGE_OID: u32 = 3906;
pub const TYPE_NUMRANGE_ARRAY_OID: u32 = 3907;
pub const TYPE_TSRANGE_OID: u32 = 3908;
pub const TYPE_TSRANGE_ARRAY_OID: u32 = 3909;
pub const TYPE_TSTZRANGE_OID: u32 = 3910;
pub const TYPE_TSTZRANGE_ARRAY_OID: u32 = 3911;

/// The first OID in PostgreSQL's system catalog that is not pinned during
/// bootstrapping.
///
/// See: <https://github.com/postgres/postgres/blob/aa0105141/src/include/access/transam.h#L173-L175>
pub const FIRST_UNPINNED_OID: u32 = 12000;

/// The first OID that is assigned by Materialize rather than PostgreSQL.
pub const FIRST_MATERIALIZE_OID: u32 = 16384;

/// The first OID that is assigned to user objects rather than system builtins.
pub const FIRST_USER_OID: u32 = 20_000;

// Postgres builtins in the "unpinned" OID range. We get to choose whatever OIDs
// we like for these builtins.
pub const FUNC_PG_EXPAND_ARRAY: u32 = 12000;
pub const FUNC_PG_DIGEST_STRING: u32 = 12001;
pub const FUNC_PG_DIGEST_BYTES: u32 = 12002;
pub const FUNC_PG_HMAC_STRING: u32 = 12003;
pub const FUNC_PG_HMAC_BYTES: u32 = 12004;

// Materialize-specific builtin OIDs.
pub const TYPE_LIST_OID: u32 = 16_384;
pub const TYPE_MAP_OID: u32 = 16_385;
pub const FUNC_CEIL_F32_OID: u32 = 16_386;
pub const FUNC_CONCAT_AGG_OID: u32 = 16_387;
pub const FUNC_CSV_EXTRACT_OID: u32 = 16_388;
pub const FUNC_CURRENT_TIMESTAMP_OID: u32 = 16_389;
pub const FUNC_FLOOR_F32_OID: u32 = 16_390;
pub const FUNC_LIST_APPEND_OID: u32 = 16_392;
pub const FUNC_LIST_CAT_OID: u32 = 16_393;
pub const FUNC_LIST_LENGTH_MAX_OID: u32 = 16_394;
pub const FUNC_LIST_LENGTH_OID: u32 = 16_395;
pub const FUNC_LIST_N_LAYERS_OID: u32 = 16_396;
pub const FUNC_LIST_PREPEND_OID: u32 = 16_397;
pub const FUNC_MAX_BOOL_OID: u32 = 16_398;
pub const FUNC_MIN_BOOL_OID: u32 = 16_399;
pub const FUNC_MZ_ALL_OID: u32 = 16_400;
pub const FUNC_MZ_ANY_OID: u32 = 16_401;
pub const FUNC_MZ_AVG_PROMOTION_DECIMAL_OID: u32 = 16_402;
pub const FUNC_MZ_AVG_PROMOTION_F32_OID_INTERNAL_V1: u32 = 16_403;
pub const FUNC_MZ_AVG_PROMOTION_F64_OID_INTERNAL_V1: u32 = 16_404;
pub const FUNC_MZ_AVG_PROMOTION_I32_OID_INTERNAL_V1: u32 = 16_405;
pub const FUNC_MZ_ENVIRONMENT_ID_OID: u32 = 16_407;
pub const FUNC_MZ_LOGICAL_TIMESTAMP_OID: u32 = 16_409;
pub const FUNC_MZ_RENDER_TYPMOD_OID: u32 = 16_410;
pub const FUNC_MZ_VERSION_OID: u32 = 16_411;
pub const FUNC_REGEXP_EXTRACT_OID: u32 = 16_412;
pub const FUNC_REPEAT_OID: u32 = 16_413;
pub const FUNC_ROUND_F32_OID: u32 = 16_414;
pub const FUNC_UNNEST_LIST_OID: u32 = 16_416;
pub const OP_CONCAT_ELEMENY_LIST_OID: u32 = 16_417;
pub const OP_CONCAT_LIST_ELEMENT_OID: u32 = 16_418;
pub const OP_CONCAT_LIST_LIST_OID: u32 = 16_419;
pub const OP_CONTAINED_JSONB_STRING_OID: u32 = 16_420;
pub const OP_CONTAINED_MAP_MAP_OID: u32 = 16_421;
pub const OP_CONTAINED_STRING_JSONB_OID: u32 = 16_422;
pub const OP_CONTAINS_ALL_KEYS_MAP_OID: u32 = 16_423;
pub const OP_CONTAINS_ANY_KEYS_MAP_OID: u32 = 16_424;
pub const OP_CONTAINS_JSONB_STRING_OID: u32 = 16_425;
pub const OP_CONTAINS_KEY_MAP_OID: u32 = 16_426;
pub const OP_CONTAINS_MAP_MAP_OID: u32 = 16_427;
pub const OP_CONTAINS_STRING_JSONB_OID: u32 = 16_428;
pub const OP_GET_VALUE_MAP_OID: u32 = 16_429;
pub const OP_GET_VALUES_MAP_OID: u32 = 16_430;
pub const OP_MOD_F32_OID: u32 = 16_431;
pub const OP_MOD_F64_OID: u32 = 16_432;
pub const OP_UNARY_PLUS_OID: u32 = 16_433;
pub const FUNC_MZ_SLEEP_OID: u32 = 16_434;
pub const FUNC_MZ_SESSION_ID_OID: u32 = 16_435;
pub const FUNC_MZ_UPTIME_OID: u32 = 16_436;
pub const FUNC_MZ_WORKERS_OID: u32 = 16_437;
pub const __DEPRECATED_TYPE_APD_OID: u32 = 16_438;
pub const FUNC_LIST_EQ_OID: u32 = 16_439;
pub const FUNC_MZ_ROW_SIZE: u32 = 16_440;
pub const FUNC_MAX_NUMERIC_OID: u32 = 16_441;
pub const FUNC_MIN_NUMERIC_OID: u32 = 16_442;
pub const FUNC_MZ_AVG_PROMOTION_I16_OID_INTERNAL_V1: u32 = 16_443;
pub const FUNC_LIST_AGG_OID: u32 = 16_444;
pub const FUNC_MZ_ERROR_IF_NULL_OID: u32 = 16_445;
pub const FUNC_MZ_DATE_BIN_UNIX_EPOCH_TS_OID: u32 = 16_446;
pub const FUNC_MZ_DATE_BIN_UNIX_EPOCH_TSTZ_OID: u32 = 16_447;
pub const FUNC_LIST_REMOVE_OID: u32 = 16_448;
pub const FUNC_MZ_DATE_BIN_HOPPING_UNIX_EPOCH_TS_OID: u32 = 16_449;
pub const FUNC_MZ_DATE_BIN_HOPPING_UNIX_EPOCH_TSTZ_OID: u32 = 16_450;
pub const FUNC_MZ_DATE_BIN_HOPPING_TS_OID: u32 = 16_451;
pub const FUNC_MZ_DATE_BIN_HOPPING_TSTZ_OID: u32 = 16_452;
pub const FUNC_MZ_TYPE_NAME: u32 = 16_453;
pub const TYPE_ANYCOMPATIBLELIST_OID: u32 = 16_454;
pub const TYPE_ANYCOMPATIBLEMAP_OID: u32 = 16_455;
pub const FUNC_MAP_LENGTH_OID: u32 = 16_456;
pub const FUNC_MZ_PANIC_OID: u32 = 16_457;
pub const FUNC_MZ_VERSION_NUM_OID: u32 = 16_458;
pub const FUNC_TRUNC_F32_OID: u32 = 16_459;
pub const TYPE_UINT2_OID: u32 = 16_460;
pub const TYPE_UINT2_ARRAY_OID: u32 = 16_461;
pub const TYPE_UINT4_OID: u32 = 16_462;
pub const TYPE_UINT4_ARRAY_OID: u32 = 16_463;
pub const TYPE_UINT8_OID: u32 = 16_464;
pub const TYPE_UINT8_ARRAY_OID: u32 = 16_465;
pub const FUNC_ADD_UINT16: u32 = 16_466;
pub const FUNC_ADD_UINT32: u32 = 16_467;
pub const FUNC_ADD_UINT64: u32 = 16_468;
pub const FUNC_SUB_UINT16: u32 = 16_469;
pub const FUNC_SUB_UINT32: u32 = 16_470;
pub const FUNC_SUB_UINT64: u32 = 16_471;
pub const FUNC_MUL_UINT16: u32 = 16_472;
pub const FUNC_MUL_UINT32: u32 = 16_473;
pub const FUNC_MUL_UINT64: u32 = 16_474;
pub const FUNC_DIV_UINT16: u32 = 16_475;
pub const FUNC_DIV_UINT32: u32 = 16_476;
pub const FUNC_DIV_UINT64: u32 = 16_477;
pub const FUNC_MOD_UINT16: u32 = 16_478;
pub const FUNC_MOD_UINT32: u32 = 16_479;
pub const FUNC_MOD_UINT64: u32 = 16_480;
pub const FUNC_AND_UINT16: u32 = 16_481;
pub const FUNC_AND_UINT32: u32 = 16_482;
pub const FUNC_AND_UINT64: u32 = 16_483;
pub const FUNC_OR_UINT16: u32 = 16_484;
pub const FUNC_OR_UINT32: u32 = 16_485;
pub const FUNC_OR_UINT64: u32 = 16_486;
pub const FUNC_XOR_UINT16: u32 = 16_487;
pub const FUNC_XOR_UINT32: u32 = 16_488;
pub const FUNC_XOR_UINT64: u32 = 16_489;
pub const FUNC_SHIFT_LEFT_UINT16: u32 = 16_490;
pub const FUNC_SHIFT_LEFT_UINT32: u32 = 16_491;
pub const FUNC_SHIFT_LEFT_UINT64: u32 = 16_492;
pub const FUNC_SHIFT_RIGHT_UINT16: u32 = 16_493;
pub const FUNC_SHIFT_RIGHT_UINT32: u32 = 16_494;
pub const FUNC_SHIFT_RIGHT_UINT64: u32 = 16_495;
pub const FUNC_MAX_UINT16_OID: u32 = 16_496;
pub const FUNC_MAX_UINT32_OID: u32 = 16_497;
pub const FUNC_MAX_UINT64_OID: u32 = 16_498;
pub const FUNC_MIN_UINT16_OID: u32 = 16_499;
pub const FUNC_MIN_UINT32_OID: u32 = 16_500;
pub const FUNC_MIN_UINT64_OID: u32 = 16_501;
pub const FUNC_SUM_UINT16_OID: u32 = 16_502;
pub const FUNC_SUM_UINT32_OID: u32 = 16_503;
pub const FUNC_SUM_UINT64_OID: u32 = 16_504;
pub const FUNC_AVG_UINT16_OID: u32 = 16_505;
pub const FUNC_AVG_UINT32_OID: u32 = 16_506;
pub const FUNC_AVG_UINT64_OID: u32 = 16_507;
pub const FUNC_MOD_UINT16_OID: u32 = 16_508;
pub const FUNC_MOD_UINT32_OID: u32 = 16_509;
pub const FUNC_MOD_UINT64_OID: u32 = 16_510;
pub const FUNC_STDDEV_UINT16_OID: u32 = 16_511;
pub const FUNC_STDDEV_UINT32_OID: u32 = 16_512;
pub const FUNC_STDDEV_UINT64_OID: u32 = 16_513;
pub const FUNC_STDDEV_POP_UINT16_OID: u32 = 16_514;
pub const FUNC_STDDEV_POP_UINT32_OID: u32 = 16_515;
pub const FUNC_STDDEV_POP_UINT64_OID: u32 = 16_516;
pub const FUNC_STDDEV_SAMP_UINT16_OID: u32 = 16_517;
pub const FUNC_STDDEV_SAMP_UINT32_OID: u32 = 16_518;
pub const FUNC_STDDEV_SAMP_UINT64_OID: u32 = 16_519;
pub const FUNC_VARIANCE_UINT16_OID: u32 = 16_520;
pub const FUNC_VARIANCE_UINT32_OID: u32 = 16_521;
pub const FUNC_VARIANCE_UINT64_OID: u32 = 16_522;
pub const FUNC_VAR_POP_UINT16_OID: u32 = 16_523;
pub const FUNC_VAR_POP_UINT32_OID: u32 = 16_524;
pub const FUNC_VAR_POP_UINT64_OID: u32 = 16_525;
pub const FUNC_VAR_SAMP_UINT16_OID: u32 = 16_526;
pub const FUNC_VAR_SAMP_UINT32_OID: u32 = 16_527;
pub const FUNC_VAR_SAMP_UINT64_OID: u32 = 16_528;
pub const FUNC_BIT_NOT_UINT16_OID: u32 = 16_529;
pub const FUNC_BIT_NOT_UINT32_OID: u32 = 16_530;
pub const FUNC_BIT_NOT_UINT64_OID: u32 = 16_531;
pub const FUNC_LT_UINT16_OID: u32 = 16_532;
pub const FUNC_LT_UINT32_OID: u32 = 16_533;
pub const FUNC_LT_UINT64_OID: u32 = 16_534;
pub const FUNC_LTE_UINT16_OID: u32 = 16_535;
pub const FUNC_LTE_UINT32_OID: u32 = 16_536;
pub const FUNC_LTE_UINT64_OID: u32 = 16_537;
pub const FUNC_GT_UINT16_OID: u32 = 16_538;
pub const FUNC_GT_UINT32_OID: u32 = 16_539;
pub const FUNC_GT_UINT64_OID: u32 = 16_540;
pub const FUNC_GTE_UINT16_OID: u32 = 16_541;
pub const FUNC_GTE_UINT32_OID: u32 = 16_542;
pub const FUNC_GTE_UINT64_OID: u32 = 16_543;
pub const FUNC_EQ_UINT16_OID: u32 = 16_544;
pub const FUNC_EQ_UINT32_OID: u32 = 16_545;
pub const FUNC_EQ_UINT64_OID: u32 = 16_546;
pub const FUNC_NOT_EQ_UINT16_OID: u32 = 16_547;
pub const FUNC_NOT_EQ_UINT32_OID: u32 = 16_548;
pub const FUNC_NOT_EQ_UINT64_OID: u32 = 16_549;
pub const FUNC_MZ_AVG_PROMOTION_U16_OID_INTERNAL_V1: u32 = 16_550;
pub const FUNC_MZ_AVG_PROMOTION_U32_OID_INTERNAL_V1: u32 = 16_551;
pub const TYPE_MZ_TIMESTAMP_OID: u32 = 16_552;
pub const TYPE_MZ_TIMESTAMP_ARRAY_OID: u32 = 16_553;
pub const FUNC_MZ_TIMESTAMP_EQ_MZ_TIMESTAMP_OID: u32 = 16_554;
pub const FUNC_MZ_TIMESTAMP_NOT_EQ_MZ_TIMESTAMP_OID: u32 = 16_555;
pub const FUNC_MZ_TIMESTAMP_LT_MZ_TIMESTAMP_OID: u32 = 16_556;
pub const FUNC_MZ_TIMESTAMP_LTE_MZ_TIMESTAMP_OID: u32 = 16_557;
pub const FUNC_MZ_TIMESTAMP_GT_MZ_TIMESTAMP_OID: u32 = 16_558;
pub const FUNC_MZ_TIMESTAMP_GTE_MZ_TIMESTAMP_OID: u32 = 16_559;
pub const FUNC_MZ_NOW_OID: u32 = 16_560;
pub const FUNC_MAX_MZ_TIMESTAMP_OID: u32 = 16_561;
pub const FUNC_MIN_MZ_TIMESTAMP_OID: u32 = 16_562;
pub const FUNC_DATE_FROM_TEXT: u32 = 16_563;
pub const FUNC_CEILING_F32_OID: u32 = 16_564;
pub const FUNC_PG_UUID_GENERATE_V5: u32 = 16_565;
pub const TYPE_MZ_ACL_ITEM_OID: u32 = 16_566;
pub const TYPE_MZ_ACL_ITEM_ARRAY_OID: u32 = 16_567;
pub const FUNC_MZ_ACL_ITEM_EQ_MZ_ACL_ITEM_OID: u32 = 16_568;
pub const FUNC_MZ_ACL_ITEM_NOT_EQ_MZ_ACL_ITEM_OID: u32 = 16_569;
pub const FUNC_MAKE_MZ_ACL_ITEM_OID: u32 = 16_570;
pub const FUNC_MZ_ACL_ITEM_GRANTOR_OID: u32 = 16_571;
pub const FUNC_MZ_ACL_ITEM_GRANTEE_OID: u32 = 16_572;
pub const FUNC_MZ_ACL_ITEM_PRIVILEGES_OID: u32 = 16_573;
pub const FUNC_IS_RBAC_ENABLED_OID: u32 = 16_574;
pub const FUNC_MZ_ACL_ITEM_CONTAINS_PRIVILEGE_OID: u32 = 16_575;
pub const FUNC_MZ_VALIDATE_PRIVILEGES_OID: u32 = 16_576;
pub const FUNC_ROLE_OID_OID: u32 = 16_577;
pub const FUNC_MZ_MINIMINAL_NAME_QUALIFICATION: u32 = 16_578;
pub const _FUNC_MZ_NAME_TO_GLOBAL_ID_ANY_SCHEMA: u32 = 16_579;
pub const _FUNC_MZ_NAME_TO_GLOBAL_ID_ONE_SCHEMA: u32 = 16_580;
pub const FUNC_MZ_RESOLVE_OBJECT_NAME: u32 = 16_581;
pub const FUNC_MZ_GLOBAL_ID_TO_NAME: u32 = 16_582;
pub const _FUNC_MZ_GET_SUBSOURCES: u32 = 16_583;
pub const FUNC_PARSE_IDENT_DEFAULT_STRICT: u32 = 16_584;
pub const FUNC_DATABASE_OID_OID: u32 = 16_585;
pub const FUNC_SCHEMA_OID_OID: u32 = 16_586;
pub const FUNC_HAS_CLUSTER_PRIVILEGE_TEXT_TEXT_TEXT_OID: u32 = 16_587;
pub const FUNC_HAS_CLUSTER_PRIVILEGE_OID_TEXT_TEXT_OID: u32 = 16_588;
pub const FUNC_HAS_CLUSTER_PRIVILEGE_TEXT_TEXT_OID: u32 = 16_589;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_TEXT_TEXT_TEXT_OID: u32 = 16_591;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_TEXT_OID_TEXT_OID: u32 = 16_592;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_OID_TEXT_TEXT_OID: u32 = 16_593;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_OID_OID_TEXT_OID: u32 = 16_594;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_TEXT_TEXT_OID: u32 = 16_595;
pub const FUNC_HAS_CONNECTION_PRIVILEGE_OID_TEXT_OID: u32 = 16_596;
pub const FUNC_HAS_SECRET_PRIVILEGE_TEXT_TEXT_TEXT_OID: u32 = 16_597;
pub const FUNC_HAS_SECRET_PRIVILEGE_TEXT_OID_TEXT_OID: u32 = 16_598;
pub const FUNC_HAS_SECRET_PRIVILEGE_OID_TEXT_TEXT_OID: u32 = 16_599;
pub const FUNC_HAS_SECRET_PRIVILEGE_OID_OID_TEXT_OID: u32 = 16_600;
pub const FUNC_HAS_SECRET_PRIVILEGE_TEXT_TEXT_OID: u32 = 16_601;
pub const FUNC_HAS_SECRET_PRIVILEGE_OID_TEXT_OID: u32 = 16_602;
pub const FUNC_MZ_NORMALIZE_OBJECT_NAME: u32 = 16_603;
pub const FUNC_DATEDIFF_DATE: u32 = 16_604;
pub const FUNC_DATEDIFF_TIME: u32 = 16_605;
pub const FUNC_DATEDIFF_TIMESTAMP: u32 = 16_606;
pub const FUNC_DATEDIFF_TIMESTAMPTZ: u32 = 16_607;
pub const FUNC_HAS_SYSTEM_PRIVILEGE_TEXT_TEXT_OID: u32 = 16_608;
pub const FUNC_HAS_SYSTEM_PRIVILEGE_OID_TEXT_OID: u32 = 16_609;
pub const FUNC_HAS_SYSTEM_PRIVILEGE_TEXT_OID: u32 = 16_610;
pub const FUNC_TRY_PARSE_MONOTONIC_ISO8601_TIMESTAMP: u32 = 16_611;
pub const FUNC_MZ_ROLE_OID_MEMBERSHIPS: u32 = 16_612;
pub const FUNC_MZ_VALIDATE_ROLE_PRIVILEGE_OID: u32 = 16_613;
pub const FUNC_MZ_IS_SUPERUSER: u32 = 16_614;
pub const FUNC_MZ_FORMAT_PRIVILEGES_OID: u32 = 16_615;
pub const FUNC_ACL_ITEM_GRANTOR_OID: u32 = 16_616;
pub const FUNC_ACL_ITEM_GRANTEE_OID: u32 = 16_617;
pub const FUNC_ACL_ITEM_PRIVILEGES_OID: u32 = 16_618;
pub const FUNC_MZ_ACL_ITEM_EXPLODE_OID: u32 = 16_619;
pub const FUNC_HAS_ROLE_TEXT_TEXT_TEXT_OID: u32 = 16_620;
pub const FUNC_HAS_ROLE_TEXT_OID_TEXT_OID: u32 = 16_621;
pub const FUNC_HAS_ROLE_OID_TEXT_TEXT_OID: u32 = 16_622;
pub const FUNC_HAS_ROLE_OID_OID_TEXT_OID: u32 = 16_623;
pub const FUNC_HAS_ROLE_TEXT_TEXT_OID: u32 = 16_624;
pub const FUNC_HAS_ROLE_OID_TEXT_OID: u32 = 16_625;
pub const FUNC_MZ_AVG_PROMOTION_F32_OID: u32 = 16_626;
pub const FUNC_MZ_AVG_PROMOTION_F64_OID: u32 = 16_627;
pub const FUNC_MZ_AVG_PROMOTION_I16_OID: u32 = 16_628;
pub const FUNC_MZ_AVG_PROMOTION_I32_OID: u32 = 16_629;
pub const FUNC_MZ_AVG_PROMOTION_I64_OID: u32 = 16_630;
pub const FUNC_MZ_AVG_PROMOTION_U16_OID: u32 = 16_631;
pub const FUNC_MZ_AVG_PROMOTION_U32_OID: u32 = 16_632;
pub const FUNC_MZ_AVG_PROMOTION_U64_OID: u32 = 16_633;
pub const FUNC_MZ_AVG_PROMOTION_NUMERIC_OID: u32 = 16_634;
pub const FUNC_AVG_INTERNAL_V1_INT64_OID: u32 = 16_635;
pub const FUNC_AVG_INTERNAL_V1_INT32_OID: u32 = 16_636;
pub const FUNC_AVG_INTERNAL_V1_INT16_OID: u32 = 16_637;
pub const FUNC_AVG_INTERNAL_V1_UINT64_OID: u32 = 16_638;
pub const FUNC_AVG_INTERNAL_V1_UINT32_OID: u32 = 16_639;
pub const FUNC_AVG_INTERNAL_V1_UINT16_OID: u32 = 16_640;
pub const FUNC_AVG_INTERNAL_V1_FLOAT32_OID: u32 = 16_641;
pub const FUNC_AVG_INTERNAL_V1_FLOAT64_OID: u32 = 16_642;
pub const FUNC_AVG_INTERNAL_V1_INTERVAL_OID: u32 = 16_643;
pub const FUNC_CONSTANT_TIME_EQ_BYTES_OID: u32 = 16_644;
pub const FUNC_CONSTANT_TIME_EQ_STRING_OID: u32 = 16_645;
pub const FUNC_TIMEZONE_OFFSET: u32 = 16_646;
pub const FUNC_PRETTY_SQL: u32 = 16_647;
pub const FUNC_PRETTY_SQL_NOWIDTH: u32 = 16_648;
pub const FUNC_MZ_NAME_RANK: u32 = 16_649;
pub const FUNC_CONNECTION_OID_OID: u32 = 16_650;
pub const FUNC_SECRET_OID_OID: u32 = 16_651;
pub const FUNC_MAP_BUILD: u32 = 16_652;
pub const FUNC_MAP_AGG: u32 = 16_653;
pub const FUNC_UNNEST_MAP_OID: u32 = 16_654;
pub const FUNC_MZ_NORMALIZE_SCHEMA_NAME: u32 = 16_655;
pub const SCHEMA_MZ_CATALOG_OID: u32 = 16_656;
pub const SCHEMA_PG_CATALOG_OID: u32 = 16_657;
pub const SCHEMA_MZ_INTERNAL_OID: u32 = 16_658;
pub const SCHEMA_INFORMATION_SCHEMA_OID: u32 = 16_659;
pub const SCHEMA_MZ_UNSAFE_OID: u32 = 16_660;
pub const ROLE_MZ_SYSTEM_OID: u32 = 16_661;
pub const ROLE_MZ_SUPPORT_OID: u32 = 16_662;
pub const ROLE_MZ_MONITOR_OID: u32 = 16_663;
pub const ROLE_MZ_MONITOR_REDACTED_OID: u32 = 16_664;
pub const LOG_MZ_DATAFLOW_OPERATORS_PER_WORKER_OID: u32 = 16665;
pub const LOG_MZ_DATAFLOW_ADDRESSES_PER_WORKER_OID: u32 = 16666;
pub const LOG_MZ_DATAFLOW_CHANNELS_PER_WORKER_OID: u32 = 16667;
pub const LOG_MZ_SCHEDULING_ELAPSED_RAW_OID: u32 = 16668;
pub const LOG_MZ_COMPUTE_OPERATOR_DURATIONS_HISTOGRAM_RAW_OID: u32 = 16669;
pub const LOG_MZ_SCHEDULING_PARKS_HISTOGRAM_RAW_OID: u32 = 16670;
pub const LOG_MZ_ARRANGEMENT_RECORDS_RAW_OID: u32 = 16671;
pub const LOG_MZ_ARRANGEMENT_BATCHES_RAW_OID: u32 = 16672;
pub const LOG_MZ_ARRANGEMENT_SHARING_RAW_OID: u32 = 16673;
pub const LOG_MZ_ARRANGEMENT_BATCHER_RECORDS_RAW_OID: u32 = 16674;
pub const LOG_MZ_ARRANGEMENT_BATCHER_SIZE_RAW_OID: u32 = 16675;
pub const LOG_MZ_ARRANGEMENT_BATCHER_CAPACITY_RAW_OID: u32 = 16676;
pub const LOG_MZ_ARRANGEMENT_BATCHER_ALLOCATIONS_RAW_OID: u32 = 16677;
pub const LOG_MZ_COMPUTE_EXPORTS_PER_WORKER_OID: u32 = 16678;
pub const LOG_MZ_COMPUTE_FRONTIERS_PER_WORKER_OID: u32 = 16679;
pub const LOG_MZ_COMPUTE_IMPORT_FRONTIERS_PER_WORKER_OID: u32 = 16680;
pub const LOG_MZ_COMPUTE_ERROR_COUNTS_RAW_OID: u32 = 16682;
pub const LOG_MZ_ACTIVE_PEEKS_PER_WORKER_OID: u32 = 16683;
pub const LOG_MZ_PEEK_DURATIONS_HISTOGRAM_RAW_OID: u32 = 16684;
pub const LOG_MZ_DATAFLOW_SHUTDOWN_DURATIONS_HISTOGRAM_RAW_OID: u32 = 16685;
pub const LOG_MZ_ARRANGEMENT_HEAP_SIZE_RAW_OID: u32 = 16686;
pub const LOG_MZ_ARRANGEMENT_HEAP_CAPACITY_RAW_OID: u32 = 16687;
pub const LOG_MZ_ARRANGEMENT_HEAP_ALLOCATIONS_RAW_OID: u32 = 16688;
pub const LOG_MZ_MESSAGE_BATCH_COUNTS_RECEIVED_RAW_OID: u32 = 16689;
pub const LOG_MZ_MESSAGE_BATCH_COUNTS_SENT_RAW_OID: u32 = 16690;
pub const LOG_MZ_MESSAGE_COUNTS_RECEIVED_RAW_OID: u32 = 16691;
pub const LOG_MZ_MESSAGE_COUNTS_SENT_RAW_OID: u32 = 16692;
pub const LOG_MZ_DATAFLOW_OPERATOR_REACHABILITY_RAW_OID: u32 = 16693;
pub const TABLE_MZ_KAFKA_SINKS_OID: u32 = 16694;
pub const TABLE_MZ_KAFKA_CONNECTIONS_OID: u32 = 16695;
pub const TABLE_MZ_KAFKA_SOURCES_OID: u32 = 16696;
pub const TABLE_MZ_POSTGRES_SOURCES_OID: u32 = 16697;
pub const TABLE_MZ_OBJECT_DEPENDENCIES_OID: u32 = 16698;
pub const SOURCE_MZ_COMPUTE_DEPENDENCIES_OID: u32 = 16699;
pub const SOURCE_MZ_COMPUTE_HYDRATION_STATUSES_OID: u32 = 16700;
pub const SOURCE_MZ_COMPUTE_OPERATOR_HYDRATION_STATUSES_PER_WORKER_OID: u32 = 16701;
pub const TABLE_MZ_DATABASES_OID: u32 = 16702;
pub const TABLE_MZ_SCHEMAS_OID: u32 = 16703;
pub const TABLE_MZ_COLUMNS_OID: u32 = 16704;
pub const TABLE_MZ_INDEXES_OID: u32 = 16705;
pub const TABLE_MZ_INDEX_COLUMNS_OID: u32 = 16706;
pub const TABLE_MZ_TABLES_OID: u32 = 16707;
pub const TABLE_MZ_CONNECTIONS_OID: u32 = 16708;
pub const TABLE_MZ_SSH_TUNNEL_CONNECTIONS_OID: u32 = 16709;
pub const TABLE_MZ_SOURCES_OID: u32 = 16710;
pub const TABLE_MZ_SINKS_OID: u32 = 16711;
pub const TABLE_MZ_VIEWS_OID: u32 = 16712;
pub const TABLE_MZ_MATERIALIZED_VIEWS_OID: u32 = 16713;
pub const TABLE_MZ_TYPES_OID: u32 = 16714;
pub const TABLE_MZ_TYPE_PG_METADATA_OID: u32 = 16715;
pub const TABLE_MZ_ARRAY_TYPES_OID: u32 = 16716;
pub const TABLE_MZ_BASE_TYPES_OID: u32 = 16717;
pub const TABLE_MZ_LIST_TYPES_OID: u32 = 16718;
pub const TABLE_MZ_MAP_TYPES_OID: u32 = 16719;
pub const TABLE_MZ_ROLES_OID: u32 = 16720;
pub const TABLE_MZ_ROLE_MEMBERS_OID: u32 = 16721;
pub const TABLE_MZ_PSEUDO_TYPES_OID: u32 = 16722;
pub const TABLE_MZ_FUNCTIONS_OID: u32 = 16723;
pub const TABLE_MZ_OPERATORS_OID: u32 = 16724;
pub const TABLE_MZ_AGGREGATES_OID: u32 = 16725;
pub const TABLE_MZ_CLUSTERS_OID: u32 = 16726;
pub const TABLE_MZ_SECRETS_OID: u32 = 16727;
pub const TABLE_MZ_CLUSTER_REPLICAS_OID: u32 = 16728;
pub const TABLE_MZ_INTERNAL_CLUSTER_REPLICAS_OID: u32 = 16729;
pub const TABLE_MZ_CLUSTER_REPLICA_STATUSES_OID: u32 = 16730;
pub const TABLE_MZ_CLUSTER_REPLICA_SIZES_OID: u32 = 16731;
pub const SOURCE_MZ_CLUSTER_REPLICA_HEARTBEATS_OID: u32 = 16732;
pub const TABLE_MZ_AUDIT_EVENTS_OID: u32 = 16733;
pub const SOURCE_MZ_SOURCE_STATUS_HISTORY_OID: u32 = 16734;
pub const SOURCE_MZ_AWS_PRIVATELINK_CONNECTION_STATUS_HISTORY_OID: u32 = 16735;
pub const VIEW_MZ_AWS_PRIVATELINK_CONNECTION_STATUSES_OID: u32 = 16736;
pub const SOURCE_MZ_STATEMENT_EXECUTION_HISTORY_OID: u32 = 16737;
pub const VIEW_MZ_STATEMENT_EXECUTION_HISTORY_REDACTED_OID: u32 = 16738;
pub const SOURCE_MZ_PREPARED_STATEMENT_HISTORY_OID: u32 = 16739;
pub const SOURCE_MZ_SQL_TEXT_OID: u32 = 16740;
pub const VIEW_MZ_SQL_TEXT_REDACTED_OID: u32 = 16741;
pub const VIEW_MZ_RECENT_SQL_TEXT_OID: u32 = 16742;
pub const VIEW_MZ_RECENT_SQL_TEXT_REDACTED_OID: u32 = 16743;
pub const INDEX_MZ_RECENT_SQL_TEXT_IND_OID: u32 = 16744;
pub const SOURCE_MZ_SESSION_HISTORY_OID: u32 = 16745;
pub const VIEW_MZ_ACTIVITY_LOG_THINNED_OID: u32 = 16746;
pub const VIEW_MZ_RECENT_ACTIVITY_LOG_THINNED_OID: u32 = 16747;
pub const VIEW_MZ_RECENT_ACTIVITY_LOG_OID: u32 = 16748;
pub const VIEW_MZ_RECENT_ACTIVITY_LOG_REDACTED_OID: u32 = 16749;
pub const SOURCE_MZ_STATEMENT_LIFECYCLE_HISTORY_OID: u32 = 16750;
pub const VIEW_MZ_SOURCE_STATUSES_OID: u32 = 16751;
pub const SOURCE_MZ_SINK_STATUS_HISTORY_OID: u32 = 16752;
pub const VIEW_MZ_SINK_STATUSES_OID: u32 = 16753;
pub const TABLE_MZ_STORAGE_USAGE_BY_SHARD_OID: u32 = 16754;
pub const TABLE_MZ_EGRESS_IPS_OID: u32 = 16755;
pub const TABLE_MZ_AWS_PRIVATELINK_CONNECTIONS_OID: u32 = 16756;
pub const TABLE_MZ_AWS_CONNECTIONS_OID: u32 = 16757;
pub const TABLE_MZ_CLUSTER_REPLICA_METRICS_OID: u32 = 16758;
pub const SOURCE_MZ_CLUSTER_REPLICA_FRONTIERS_OID: u32 = 16759;
pub const SOURCE_MZ_FRONTIERS_OID: u32 = 16760;
pub const VIEW_MZ_GLOBAL_FRONTIERS_OID: u32 = 16761;
pub const TABLE_MZ_SUBSCRIPTIONS_OID: u32 = 16762;
pub const TABLE_MZ_SESSIONS_OID: u32 = 16763;
pub const TABLE_MZ_DEFAULT_PRIVILEGES_OID: u32 = 16764;
pub const TABLE_MZ_SYSTEM_PRIVILEGES_OID: u32 = 16765;
pub const TABLE_MZ_COMMENTS_OID: u32 = 16766;
pub const TABLE_MZ_WEBHOOK_SOURCES_OID: u32 = 16767;
pub const SOURCE_MZ_SOURCE_STATISTICS_RAW_OID: u32 = 16768;
pub const SOURCE_MZ_SINK_STATISTICS_RAW_OID: u32 = 16769;
pub const SOURCE_MZ_STORAGE_SHARDS_OID: u32 = 16770;
pub const VIEW_MZ_STORAGE_USAGE_OID: u32 = 16771;
pub const VIEW_MZ_RELATIONS_OID: u32 = 16772;
pub const VIEW_MZ_OBJECT_OID_ALIAS_OID: u32 = 16773;
pub const VIEW_MZ_OBJECTS_OID: u32 = 16774;
pub const VIEW_MZ_OBJECT_FULLY_QUALIFIED_NAMES_OID: u32 = 16775;
pub const VIEW_MZ_OBJECT_LIFETIMES_OID: u32 = 16776;
pub const VIEW_MZ_DATAFLOWS_PER_WORKER_OID: u32 = 16777;
pub const VIEW_MZ_DATAFLOWS_OID: u32 = 16778;
pub const VIEW_MZ_DATAFLOW_ADDRESSES_OID: u32 = 16779;
pub const VIEW_MZ_DATAFLOW_CHANNELS_OID: u32 = 16780;
pub const VIEW_MZ_DATAFLOW_OPERATORS_OID: u32 = 16781;
pub const VIEW_MZ_DATAFLOW_OPERATOR_DATAFLOWS_PER_WORKER_OID: u32 = 16782;
pub const VIEW_MZ_DATAFLOW_OPERATOR_DATAFLOWS_OID: u32 = 16783;
pub const VIEW_MZ_OBJECT_TRANSITIVE_DEPENDENCIES_OID: u32 = 16784;
pub const VIEW_MZ_COMPUTE_EXPORTS_OID: u32 = 16785;
pub const VIEW_MZ_COMPUTE_FRONTIERS_OID: u32 = 16786;
pub const VIEW_MZ_DATAFLOW_CHANNEL_OPERATORS_PER_WORKER_OID: u32 = 16787;
pub const VIEW_MZ_DATAFLOW_CHANNEL_OPERATORS_OID: u32 = 16788;
pub const VIEW_MZ_COMPUTE_IMPORT_FRONTIERS_OID: u32 = 16789;
pub const VIEW_MZ_RECORDS_PER_DATAFLOW_OPERATOR_PER_WORKER_OID: u32 = 16790;
pub const VIEW_MZ_RECORDS_PER_DATAFLOW_OPERATOR_OID: u32 = 16791;
pub const VIEW_MZ_RECORDS_PER_DATAFLOW_PER_WORKER_OID: u32 = 16792;
pub const VIEW_MZ_RECORDS_PER_DATAFLOW_OID: u32 = 16793;
pub const VIEW_PG_NAMESPACE_OID: u32 = 16794;
pub const VIEW_PG_CLASS_OID: u32 = 16795;
pub const VIEW_PG_DEPEND_OID: u32 = 16796;
pub const VIEW_PG_DATABASE_OID: u32 = 16797;
pub const VIEW_PG_INDEX_OID: u32 = 16798;
pub const VIEW_PG_INDEXES_OID: u32 = 16799;
pub const VIEW_PG_DESCRIPTION_OID: u32 = 16800;
pub const VIEW_PG_TYPE_OID: u32 = 16801;
pub const VIEW_PG_ATTRIBUTE_OID: u32 = 16802;
pub const VIEW_PG_PROC_OID: u32 = 16803;
pub const VIEW_PG_OPERATOR_OID: u32 = 16804;
pub const VIEW_PG_RANGE_OID: u32 = 16805;
pub const VIEW_PG_ENUM_OID: u32 = 16806;
pub const VIEW_PG_ATTRDEF_OID: u32 = 16807;
pub const VIEW_PG_SETTINGS_OID: u32 = 16808;
pub const VIEW_PG_AUTH_MEMBERS_OID: u32 = 16809;
pub const VIEW_PG_EVENT_TRIGGER_OID: u32 = 16810;
pub const VIEW_PG_LANGUAGE_OID: u32 = 16811;
pub const VIEW_PG_SHDESCRIPTION_OID: u32 = 16812;
pub const VIEW_PG_TIMEZONE_ABBREVS_OID: u32 = 16813;
pub const VIEW_PG_TIMEZONE_NAMES_OID: u32 = 16814;
pub const VIEW_MZ_TIMEZONE_ABBREVIATIONS_OID: u32 = 16815;
pub const VIEW_MZ_TIMEZONE_NAMES_OID: u32 = 16816;
pub const VIEW_MZ_PEEK_DURATIONS_HISTOGRAM_PER_WORKER_OID: u32 = 16817;
pub const VIEW_MZ_PEEK_DURATIONS_HISTOGRAM_OID: u32 = 16818;
pub const VIEW_MZ_DATAFLOW_SHUTDOWN_DURATIONS_HISTOGRAM_PER_WORKER_OID: u32 = 16819;
pub const VIEW_MZ_DATAFLOW_SHUTDOWN_DURATIONS_HISTOGRAM_OID: u32 = 16820;
pub const VIEW_MZ_SCHEDULING_ELAPSED_PER_WORKER_OID: u32 = 16821;
pub const VIEW_MZ_SCHEDULING_ELAPSED_OID: u32 = 16822;
pub const VIEW_MZ_COMPUTE_OPERATOR_DURATIONS_HISTOGRAM_PER_WORKER_OID: u32 = 16823;
pub const VIEW_MZ_COMPUTE_OPERATOR_DURATIONS_HISTOGRAM_OID: u32 = 16824;
pub const VIEW_MZ_SCHEDULING_PARKS_HISTOGRAM_PER_WORKER_OID: u32 = 16825;
pub const VIEW_MZ_SCHEDULING_PARKS_HISTOGRAM_OID: u32 = 16826;
pub const VIEW_MZ_COMPUTE_ERROR_COUNTS_PER_WORKER_OID: u32 = 16829;
pub const VIEW_MZ_COMPUTE_ERROR_COUNTS_OID: u32 = 16830;
pub const VIEW_MZ_COMPUTE_OPERATOR_HYDRATION_STATUSES_OID: u32 = 16831;
pub const VIEW_MZ_MESSAGE_COUNTS_PER_WORKER_OID: u32 = 16832;
pub const VIEW_MZ_MESSAGE_COUNTS_OID: u32 = 16833;
pub const VIEW_MZ_ACTIVE_PEEKS_OID: u32 = 16834;
pub const VIEW_MZ_DATAFLOW_OPERATOR_REACHABILITY_PER_WORKER_OID: u32 = 16835;
pub const VIEW_MZ_DATAFLOW_OPERATOR_REACHABILITY_OID: u32 = 16836;
pub const VIEW_MZ_ARRANGEMENT_SIZES_PER_WORKER_OID: u32 = 16837;
pub const VIEW_MZ_ARRANGEMENT_SIZES_OID: u32 = 16838;
pub const VIEW_MZ_ARRANGEMENT_SHARING_PER_WORKER_OID: u32 = 16839;
pub const VIEW_MZ_ARRANGEMENT_SHARING_OID: u32 = 16840;
pub const VIEW_MZ_CLUSTER_REPLICA_UTILIZATION_OID: u32 = 16841;
pub const VIEW_MZ_DATAFLOW_OPERATOR_PARENTS_PER_WORKER_OID: u32 = 16842;
pub const VIEW_MZ_DATAFLOW_OPERATOR_PARENTS_OID: u32 = 16843;
pub const VIEW_MZ_DATAFLOW_ARRANGEMENT_SIZES_OID: u32 = 16844;
pub const VIEW_MZ_EXPECTED_GROUP_SIZE_ADVICE_OID: u32 = 16845;
pub const VIEW_PG_CONSTRAINT_OID: u32 = 16846;
pub const VIEW_PG_TABLES_OID: u32 = 16847;
pub const VIEW_PG_TABLESPACE_OID: u32 = 16848;
pub const VIEW_PG_AM_OID: u32 = 16849;
pub const VIEW_PG_ROLES_OID: u32 = 16850;
pub const VIEW_PG_VIEWS_OID: u32 = 16851;
pub const VIEW_PG_MATVIEWS_OID: u32 = 16852;
pub const VIEW_APPLICABLE_ROLES_OID: u32 = 16853;
pub const VIEW_COLUMNS_OID: u32 = 16854;
pub const VIEW_ENABLED_ROLES_OID: u32 = 16855;
pub const VIEW_ROLE_TABLE_GRANTS_OID: u32 = 16856;
pub const VIEW_KEY_COLUMN_USAGE_OID: u32 = 16857;
pub const VIEW_REFERENTIAL_CONSTRAINTS_OID: u32 = 16858;
pub const VIEW_ROUTINES_OID: u32 = 16859;
pub const VIEW_SCHEMATA_OID: u32 = 16860;
pub const VIEW_TABLES_OID: u32 = 16861;
pub const VIEW_TABLE_CONSTRAINTS_OID: u32 = 16862;
pub const VIEW_TABLE_PRIVILEGES_OID: u32 = 16863;
pub const VIEW_TRIGGERS_OID: u32 = 16864;
pub const VIEW_VIEWS_OID: u32 = 16865;
pub const VIEW_CHARACTER_SETS_OID: u32 = 16866;
pub const VIEW_PG_COLLATION_OID: u32 = 16867;
pub const VIEW_PG_POLICY_OID: u32 = 16868;
pub const VIEW_PG_INHERITS_OID: u32 = 16869;
pub const VIEW_PG_LOCKS_OID: u32 = 16870;
pub const VIEW_PG_AUTHID_OID: u32 = 16871;
pub const VIEW_PG_AGGREGATE_OID: u32 = 16872;
pub const VIEW_PG_TRIGGER_OID: u32 = 16873;
pub const VIEW_PG_REWRITE_OID: u32 = 16874;
pub const VIEW_PG_EXTENSION_OID: u32 = 16875;
pub const VIEW_MZ_SHOW_SOURCES_OID: u32 = 16876;
pub const VIEW_MZ_SHOW_SINKS_OID: u32 = 16877;
pub const VIEW_MZ_SHOW_MATERIALIZED_VIEWS_OID: u32 = 16878;
pub const VIEW_MZ_SHOW_INDEXES_OID: u32 = 16879;
pub const VIEW_MZ_SHOW_CLUSTER_REPLICAS_OID: u32 = 16880;
pub const VIEW_MZ_SHOW_ROLE_MEMBERS_OID: u32 = 16881;
pub const VIEW_MZ_SHOW_MY_ROLE_MEMBERS_OID: u32 = 16882;
pub const VIEW_MZ_SHOW_SYSTEM_PRIVILEGES_OID: u32 = 16883;
pub const VIEW_MZ_SHOW_MY_SYSTEM_PRIVILEGES_OID: u32 = 16884;
pub const VIEW_MZ_SHOW_CLUSTER_PRIVILEGES_OID: u32 = 16885;
pub const VIEW_MZ_SHOW_MY_CLUSTER_PRIVILEGES_OID: u32 = 16886;
pub const VIEW_MZ_SHOW_DATABASE_PRIVILEGES_OID: u32 = 16887;
pub const VIEW_MZ_SHOW_MY_DATABASE_PRIVILEGES_OID: u32 = 16888;
pub const VIEW_MZ_SHOW_SCHEMA_PRIVILEGES_OID: u32 = 16889;
pub const VIEW_MZ_SHOW_MY_SCHEMA_PRIVILEGES_OID: u32 = 16890;
pub const VIEW_MZ_SHOW_OBJECT_PRIVILEGES_OID: u32 = 16891;
pub const VIEW_MZ_SHOW_MY_OBJECT_PRIVILEGES_OID: u32 = 16892;
pub const VIEW_MZ_SHOW_ALL_PRIVILEGES_OID: u32 = 16893;
pub const VIEW_MZ_SHOW_ALL_MY_PRIVILEGES_OID: u32 = 16894;
pub const VIEW_MZ_SHOW_DEFAULT_PRIVILEGES_OID: u32 = 16895;
pub const VIEW_MZ_SHOW_MY_DEFAULT_PRIVILEGES_OID: u32 = 16896;
pub const VIEW_MZ_CLUSTER_REPLICA_HISTORY_OID: u32 = 16897;
pub const VIEW_MZ_HYDRATION_STATUSES_OID: u32 = 16898;
pub const VIEW_MZ_MATERIALIZATION_LAG_OID: u32 = 16899;
pub const INDEX_MZ_SHOW_DATABASES_IND_OID: u32 = 16900;
pub const INDEX_MZ_SHOW_SCHEMAS_IND_OID: u32 = 16901;
pub const INDEX_MZ_SHOW_CONNECTIONS_IND_OID: u32 = 16902;
pub const INDEX_MZ_SHOW_TABLES_IND_OID: u32 = 16903;
pub const INDEX_MZ_SHOW_SOURCES_IND_OID: u32 = 16904;
pub const INDEX_MZ_SHOW_VIEWS_IND_OID: u32 = 16905;
pub const INDEX_MZ_SHOW_MATERIALIZED_VIEWS_IND_OID: u32 = 16906;
pub const INDEX_MZ_SHOW_SINKS_IND_OID: u32 = 16907;
pub const INDEX_MZ_SHOW_TYPES_IND_OID: u32 = 16908;
pub const INDEX_MZ_SHOW_ALL_OBJECTS_IND_OID: u32 = 16909;
pub const INDEX_MZ_SHOW_INDEXES_IND_OID: u32 = 16910;
pub const INDEX_MZ_SHOW_COLUMNS_IND_OID: u32 = 16911;
pub const INDEX_MZ_SHOW_CLUSTERS_IND_OID: u32 = 16912;
pub const INDEX_MZ_SHOW_CLUSTER_REPLICAS_IND_OID: u32 = 16913;
pub const INDEX_MZ_SHOW_SECRETS_IND_OID: u32 = 16914;
pub const INDEX_MZ_CLUSTERS_IND_OID: u32 = 16915;
pub const INDEX_MZ_INDEXES_IND_OID: u32 = 16916;
pub const INDEX_MZ_ROLES_IND_OID: u32 = 16917;
pub const INDEX_MZ_SOURCES_IND_OID: u32 = 16918;
pub const INDEX_MZ_SINKS_IND_OID: u32 = 16919;
pub const INDEX_MZ_MATERIALIZED_VIEWS_IND_OID: u32 = 16920;
pub const INDEX_MZ_SOURCE_STATUSES_IND_OID: u32 = 16921;
pub const INDEX_MZ_SINK_STATUSES_IND_OID: u32 = 16922;
pub const INDEX_MZ_SOURCE_STATUS_HISTORY_IND_OID: u32 = 16923;
pub const INDEX_MZ_SINK_STATUS_HISTORY_IND_OID: u32 = 16924;
pub const VIEW_MZ_SOURCE_STATISTICS_OID: u32 = 16925;
pub const INDEX_MZ_SOURCE_STATISTICS_IND_OID: u32 = 16926;
pub const VIEW_MZ_SINK_STATISTICS_OID: u32 = 16927;
pub const INDEX_MZ_SINK_STATISTICS_IND_OID: u32 = 16928;
pub const INDEX_MZ_CLUSTER_REPLICAS_IND_OID: u32 = 16929;
pub const INDEX_MZ_CLUSTER_REPLICA_SIZES_IND_OID: u32 = 16930;
pub const INDEX_MZ_CLUSTER_REPLICA_STATUSES_IND_OID: u32 = 16931;
pub const INDEX_MZ_CLUSTER_REPLICA_METRICS_IND_OID: u32 = 16932;
pub const INDEX_MZ_CLUSTER_REPLICA_HISTORY_IND_OID: u32 = 16933;
pub const INDEX_MZ_OBJECT_LIFETIMES_IND_OID: u32 = 16934;
pub const INDEX_MZ_OBJECT_DEPENDENCIES_IND_OID: u32 = 16935;
pub const INDEX_MZ_COMPUTE_DEPENDENCIES_IND_OID: u32 = 16936;
pub const INDEX_MZ_OBJECT_TRANSITIVE_DEPENDENCIES_IND_OID: u32 = 16937;
pub const INDEX_MZ_FRONTIERS_IND_OID: u32 = 16938;
pub const INDEX_MZ_RECENT_ACTIVITY_LOG_THINNED_IND_OID: u32 = 16939;
pub const TABLE_MZ_OPTIMIZER_NOTICES_OID: u32 = 16940;
pub const VIEW_MZ_NOTICES_OID: u32 = 16941;
pub const VIEW_MZ_NOTICES_REDACTED_OID: u32 = 16942;
pub const INDEX_MZ_NOTICES_IND_OID: u32 = 16943;
pub const ROLE_PUBLIC_OID: u32 = 16944;
pub const TABLE_MZ_ROLE_PARAMETERS_OID: u32 = 16945;
pub const TABLE_MZ_MATERIALIZED_VIEW_REFRESH_STRATEGIES_OID: u32 = 16946;
pub const TABLE_MZ_CLUSTER_SCHEDULES_OID: u32 = 16947;
pub const TABLE_MZ_POSTGRES_SOURCE_TABLES_OID: u32 = 16948;
pub const TABLE_MZ_MYSQL_SOURCE_TABLES_OID: u32 = 16949;
pub const INDEX_MZ_KAFKA_SOURCES_IND_OID: u32 = 16950;
