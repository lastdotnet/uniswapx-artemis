pub use universal_router_executor::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod universal_router_executor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_whitelistedCallers"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IReactor"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_universalRouter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_permit2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactor"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReactor"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactorCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactorCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("universalRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("universalRouter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerNotWhitelisted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MsgSenderNotReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MsgSenderNotReactor",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NativeTransferFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNIVERSALROUTEREXECUTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x16\x9A8\x03\x80b\0\x16\x9A\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x016V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x82U`@Q\x85\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`\0[\x85Q\x81\x10\x15b\0\0\xDFW`\x01\x80`\0\x88\x84\x81Q\x81\x10b\0\0\xA4Wb\0\0\xA4b\0\x02TV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x01\x01b\0\0\x80V[P`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xA0R\x90\x83\x16`\x80R\x90\x91\x16`\xC0RPb\0\x02j\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x011W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01OW`\0\x80\xFD[\x85Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01gW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12b\0\x01|W`\0\x80\xFD[\x81Q` \x82\x82\x11\x15b\0\x01\x93Wb\0\x01\x93b\0\x01\x03V[\x81`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x86\x82\x11\x17\x15b\0\x01\xBBWb\0\x01\xBBb\0\x01\x03V[`@R\x92\x83R\x81\x83\x01\x93P\x84\x81\x01\x82\x01\x92\x8C\x84\x11\x15b\0\x01\xDAW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15b\0\x02\x03Wb\0\x01\xF3\x86b\0\x01\x19V[\x85R\x94\x82\x01\x94\x93\x82\x01\x93b\0\x01\xDFV[\x99Pb\0\x02\x14\x90P\x8A\x82\x01b\0\x01\x19V[\x97PPPPPb\0\x02(`@\x87\x01b\0\x01\x19V[\x92Pb\0\x028``\x87\x01b\0\x01\x19V[\x91Pb\0\x02H`\x80\x87\x01b\0\x01\x19V[\x90P\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Qa\x13\xC2b\0\x02\xD8`\09`\0\x81\x81`\xD3\x01R\x81\x81a\x03\x0B\x01Ra\x03\x8C\x01R`\0\x81\x81a\x01\xF3\x01R\x81\x81a\x02\x8D\x01R\x81\x81a\x04\xBB\x01R\x81\x81a\x05\xBD\x01R\x81\x81a\x08A\x01Ra\t;\x01R`\0\x81\x81a\x010\x01R\x81\x81a\x04.\x01Ra\x05\x1B\x01Ra\x13\xC2`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xB5W`\x005`\xE0\x1C\x80c\x94V\xFB\xCC\x11a\0iW\x80c\xD0\xF2\xD8\xAC\x11a\0NW\x80c\xD0\xF2\xD8\xAC\x14a\x02\x15W\x80c\xE5\x13^\xC6\x14a\x025W\x80c\xF2\xFD\xE3\x8B\x14a\x02UW`\0\x80\xFD[\x80c\x94V\xFB\xCC\x14a\x01\xC1W\x80c\xABW&P\x14a\x01\xE1W`\0\x80\xFD[\x80cX]\xA6(\x11a\0\x9AW\x80cX]\xA6(\x14a\x01RW\x80ci\r\x83 \x14a\x01tW\x80c\x8D\xA5\xCB[\x14a\x01\x94W`\0\x80\xFD[\x80c\x12&\x1E\xE7\x14a\0\xC1W\x80c5\xA9\xE4\xDF\x14a\x01\x1EW`\0\x80\xFD[6a\0\xBCW\0[`\0\x80\xFD[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01*W`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01^W`\0\x80\xFD[Pa\x01ra\x01m6`\x04a\r\xE5V[a\x02uV[\0[4\x80\x15a\x01\x80W`\0\x80\xFD[Pa\x01ra\x01\x8F6`\x04a\x0EsV[a\x05\xEDV[4\x80\x15a\x01\xA0W`\0\x80\xFD[P`\0Ta\0\xF5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\x01ra\x01\xDC6`\x04a\x0E\x97V[a\x06\x80V[4\x80\x15a\x01\xEDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02!W`\0\x80\xFD[Pa\x01ra\x0206`\x04a\x0E\xD0V[a\x07\xB7V[4\x80\x15a\x02AW`\0\x80\xFD[Pa\x01ra\x02P6`\x04a\r\xE5V[a\x08\xB1V[4\x80\x15a\x02aW`\0\x80\xFD[Pa\x01ra\x02p6`\x04a\x0EsV[a\t\xAEV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xE4W`@Q\x7F\x93?\xE5/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x02\xF4\x84\x86\x01\x86a\x10HV[\x92P\x92P\x92P`\0[\x83Q\x81\x10\x15a\x04\xA9Wa\x03\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x84\x81Q\x81\x10a\x03]Wa\x03]a\x11@V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\x9F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x87Q|E\x85\x83\x81Q\x81\x10a\x03\xD8Wa\x03\xD8a\x11@V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x02\xFD\x90PV[P`\0[\x82Q\x81\x10\x15a\x05\x15Wa\x05\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x84\x81Q\x81\x10a\x03]Wa\x03]a\x11@V[`\x01\x01a\x04\xADV[P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\x05^\x91\x90a\x11oV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05\x9BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xA0V[``\x91P[P\x91P\x91P\x81a\x05\xB2W\x80Q` \x82\x01\xFD[G\x15a\x05\xE2Wa\x05\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ga\x0BtV[PPPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06sW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06}\x81Ga\x0C\x13V[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\x01W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Ra\x07\xB3\x90\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a\x11\x9EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91\x90a\x0C\x88V[PPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x15\x15\x90\x03a\x08\x04W`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x08z\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x12\xAFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA8W=`\0\x80>=`\0\xFD[PPPPPPPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x15\x15\x90\x03a\x08\xFEW`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\tv\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x12\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xA4W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\n/W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FAPPROVE_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[PPPPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xD3V[``\x91P[PP\x90P\x80a\x0C\x0EW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x0C\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FETH_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0\x80\x83`\x1F\x84\x01\x12a\riW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x81W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\r\x9CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\r\xB5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xCDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x9CW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\r\xFBW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x13W`\0\x80\xFD[a\x0E\x1F\x88\x83\x89\x01a\rWV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x0E8W`\0\x80\xFD[Pa\x0EE\x87\x82\x88\x01a\r\xA3V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06}W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x85W`\0\x80\xFD[\x815a\x0E\x90\x81a\x0EQV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xAAW`\0\x80\xFD[\x825a\x0E\xB5\x81a\x0EQV[\x91P` \x83\x015a\x0E\xC5\x81a\x0EQV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xE5W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xFDW`\0\x80\xFD[\x90\x85\x01\x90`@\x82\x88\x03\x12\x15a\x0F\x11W`\0\x80\xFD[\x90\x93P` \x85\x015\x90\x80\x82\x11\x15a\x0F'W`\0\x80\xFD[Pa\x0F4\x86\x82\x87\x01a\r\xA3V[\x94\x97\x90\x96P\x93\x94PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xB7Wa\x0F\xB7a\x0FAV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F\xD0W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xECWa\x0F\xECa\x0FAV[\x81`\x05\x1Ba\x0F\xFB\x82\x82\x01a\x0FpV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x10\x15W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x10=W\x825a\x10.\x81a\x0EQV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x10\x1BV[\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10]W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10uW`\0\x80\xFD[a\x10\x81\x87\x83\x88\x01a\x0F\xBFV[\x94P` \x91P\x81\x86\x015\x81\x81\x11\x15a\x10\x98W`\0\x80\xFD[a\x10\xA4\x88\x82\x89\x01a\x0F\xBFV[\x94PP`@\x86\x015\x81\x81\x11\x15a\x10\xB9W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x10\xCAW`\0\x80\xFD[\x805\x82\x81\x11\x15a\x10\xDCWa\x10\xDCa\x0FAV[a\x11\x0C\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x0FpV[\x92P\x80\x83R\x88\x84\x82\x84\x01\x01\x11\x15a\x11\"W`\0\x80\xFD[\x80\x84\x83\x01\x85\x85\x017`\0\x84\x82\x85\x01\x01RPP\x80\x92PPP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x11\x90W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x11vV[P`\0\x92\x01\x91\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xB0W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x11\xECW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0CW`\0\x80\xFD[\x806\x03\x82\x13\x15a\r\x9CW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\x12p\x82\x83a\x11\xB7V[`@\x85Ra\x12\x82`@\x86\x01\x82\x84a\x12\x1BV[\x91PPa\x12\x92` \x84\x01\x84a\x11\xB7V[\x85\x83\x03` \x87\x01Ra\x12\xA5\x83\x82\x84a\x12\x1BV[\x96\x95PPPPPPV[`@\x81R`\0a\x12\xC2`@\x83\x01\x86a\x12dV[\x82\x81\x03` \x84\x01Ra\x12\xA5\x81\x85\x87a\x12\x1BV[`@\x80\x82R\x81\x01\x84\x90R`\0```\x05\x86\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x87\x83\x80[\x89\x81\x10\x15a\x13uW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x8C6\x03\x01\x81\x12a\x13SW\x82\x83\xFD[a\x13_\x86\x8D\x83\x01a\x12dV[\x95PP` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a\x12\xF3V[PPPP\x82\x81\x03` \x84\x01Ra\x10=\x81\x85\x87a\x12\x1BV\xFE\xA2dipfsX\"\x12 H\x91\xC7\xFA1\0\xCE2K\xDA) 7lh\xD9H0\xC8\xC5OS<[\x81\x97@\xAA\x05\xE6\x95pdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static UNIVERSALROUTEREXECUTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xB5W`\x005`\xE0\x1C\x80c\x94V\xFB\xCC\x11a\0iW\x80c\xD0\xF2\xD8\xAC\x11a\0NW\x80c\xD0\xF2\xD8\xAC\x14a\x02\x15W\x80c\xE5\x13^\xC6\x14a\x025W\x80c\xF2\xFD\xE3\x8B\x14a\x02UW`\0\x80\xFD[\x80c\x94V\xFB\xCC\x14a\x01\xC1W\x80c\xABW&P\x14a\x01\xE1W`\0\x80\xFD[\x80cX]\xA6(\x11a\0\x9AW\x80cX]\xA6(\x14a\x01RW\x80ci\r\x83 \x14a\x01tW\x80c\x8D\xA5\xCB[\x14a\x01\x94W`\0\x80\xFD[\x80c\x12&\x1E\xE7\x14a\0\xC1W\x80c5\xA9\xE4\xDF\x14a\x01\x1EW`\0\x80\xFD[6a\0\xBCW\0[`\0\x80\xFD[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01*W`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01^W`\0\x80\xFD[Pa\x01ra\x01m6`\x04a\r\xE5V[a\x02uV[\0[4\x80\x15a\x01\x80W`\0\x80\xFD[Pa\x01ra\x01\x8F6`\x04a\x0EsV[a\x05\xEDV[4\x80\x15a\x01\xA0W`\0\x80\xFD[P`\0Ta\0\xF5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\x01ra\x01\xDC6`\x04a\x0E\x97V[a\x06\x80V[4\x80\x15a\x01\xEDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02!W`\0\x80\xFD[Pa\x01ra\x0206`\x04a\x0E\xD0V[a\x07\xB7V[4\x80\x15a\x02AW`\0\x80\xFD[Pa\x01ra\x02P6`\x04a\r\xE5V[a\x08\xB1V[4\x80\x15a\x02aW`\0\x80\xFD[Pa\x01ra\x02p6`\x04a\x0EsV[a\t\xAEV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xE4W`@Q\x7F\x93?\xE5/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x02\xF4\x84\x86\x01\x86a\x10HV[\x92P\x92P\x92P`\0[\x83Q\x81\x10\x15a\x04\xA9Wa\x03\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x84\x81Q\x81\x10a\x03]Wa\x03]a\x11@V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\n\x9F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x87Q|E\x85\x83\x81Q\x81\x10a\x03\xD8Wa\x03\xD8a\x11@V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16`$\x82\x01R`D\x81\x01\x91\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x02\xFD\x90PV[P`\0[\x82Q\x81\x10\x15a\x05\x15Wa\x05\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x84\x81Q\x81\x10a\x03]Wa\x03]a\x11@V[`\x01\x01a\x04\xADV[P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\x05^\x91\x90a\x11oV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05\x9BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xA0V[``\x91P[P\x91P\x91P\x81a\x05\xB2W\x80Q` \x82\x01\xFD[G\x15a\x05\xE2Wa\x05\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ga\x0BtV[PPPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06sW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06}\x81Ga\x0C\x13V[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\x01W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Ra\x07\xB3\x90\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a\x11\x9EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91\x90a\x0C\x88V[PPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x15\x15\x90\x03a\x08\x04W`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x08z\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x12\xAFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA8W=`\0\x80>=`\0\xFD[PPPPPPPV[3`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x15\x15\x90\x03a\x08\xFEW`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\tv\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x12\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xA4W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\n/W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FAPPROVE_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[PPPPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xD3V[``\x91P[PP\x90P\x80a\x0C\x0EW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x0C\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FETH_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BnW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06jV[`\0\x80\x83`\x1F\x84\x01\x12a\riW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x81W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\r\x9CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\r\xB5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xCDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x9CW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\r\xFBW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x13W`\0\x80\xFD[a\x0E\x1F\x88\x83\x89\x01a\rWV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x0E8W`\0\x80\xFD[Pa\x0EE\x87\x82\x88\x01a\r\xA3V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06}W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x85W`\0\x80\xFD[\x815a\x0E\x90\x81a\x0EQV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xAAW`\0\x80\xFD[\x825a\x0E\xB5\x81a\x0EQV[\x91P` \x83\x015a\x0E\xC5\x81a\x0EQV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xE5W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xFDW`\0\x80\xFD[\x90\x85\x01\x90`@\x82\x88\x03\x12\x15a\x0F\x11W`\0\x80\xFD[\x90\x93P` \x85\x015\x90\x80\x82\x11\x15a\x0F'W`\0\x80\xFD[Pa\x0F4\x86\x82\x87\x01a\r\xA3V[\x94\x97\x90\x96P\x93\x94PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xB7Wa\x0F\xB7a\x0FAV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F\xD0W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F\xECWa\x0F\xECa\x0FAV[\x81`\x05\x1Ba\x0F\xFB\x82\x82\x01a\x0FpV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x10\x15W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x10=W\x825a\x10.\x81a\x0EQV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x10\x1BV[\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10]W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10uW`\0\x80\xFD[a\x10\x81\x87\x83\x88\x01a\x0F\xBFV[\x94P` \x91P\x81\x86\x015\x81\x81\x11\x15a\x10\x98W`\0\x80\xFD[a\x10\xA4\x88\x82\x89\x01a\x0F\xBFV[\x94PP`@\x86\x015\x81\x81\x11\x15a\x10\xB9W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x10\xCAW`\0\x80\xFD[\x805\x82\x81\x11\x15a\x10\xDCWa\x10\xDCa\x0FAV[a\x11\x0C\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x0FpV[\x92P\x80\x83R\x88\x84\x82\x84\x01\x01\x11\x15a\x11\"W`\0\x80\xFD[\x80\x84\x83\x01\x85\x85\x017`\0\x84\x82\x85\x01\x01RPP\x80\x92PPP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x11\x90W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x11vV[P`\0\x92\x01\x91\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xB0W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x11\xECW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0CW`\0\x80\xFD[\x806\x03\x82\x13\x15a\r\x9CW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\x12p\x82\x83a\x11\xB7V[`@\x85Ra\x12\x82`@\x86\x01\x82\x84a\x12\x1BV[\x91PPa\x12\x92` \x84\x01\x84a\x11\xB7V[\x85\x83\x03` \x87\x01Ra\x12\xA5\x83\x82\x84a\x12\x1BV[\x96\x95PPPPPPV[`@\x81R`\0a\x12\xC2`@\x83\x01\x86a\x12dV[\x82\x81\x03` \x84\x01Ra\x12\xA5\x81\x85\x87a\x12\x1BV[`@\x80\x82R\x81\x01\x84\x90R`\0```\x05\x86\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x87\x83\x80[\x89\x81\x10\x15a\x13uW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x8C6\x03\x01\x81\x12a\x13SW\x82\x83\xFD[a\x13_\x86\x8D\x83\x01a\x12dV[\x95PP` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a\x12\xF3V[PPPP\x82\x81\x03` \x84\x01Ra\x10=\x81\x85\x87a\x12\x1BV\xFE\xA2dipfsX\"\x12 H\x91\xC7\xFA1\0\xCE2K\xDA) 7lh\xD9H0\xC8\xC5OS<[\x81\x97@\xAA\x05\xE6\x95pdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static UNIVERSALROUTEREXECUTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniversalRouterExecutor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniversalRouterExecutor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniversalRouterExecutor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniversalRouterExecutor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniversalRouterExecutor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniversalRouterExecutor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniversalRouterExecutor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNIVERSALROUTEREXECUTOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                UNIVERSALROUTEREXECUTOR_ABI.clone(),
                UNIVERSALROUTEREXECUTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0xd0f2d8ac) function
        pub fn execute(
            &self,
            order: SignedOrder,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 242, 216, 172], (order, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0xe5135ec6) function
        pub fn execute_batch(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 19, 94, 198], (orders, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2` (0x12261ee7) function
        pub fn permit_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([18, 38, 30, 231], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactor` (0xab572650) function
        pub fn reactor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([171, 87, 38, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x585da628) function
        pub fn reactor_callback(
            &self,
            p0: ::std::vec::Vec<ResolvedOrder>,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 93, 166, 40], (p0, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `universalRouter` (0x35a9e4df) function
        pub fn universal_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 169, 228, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC20` (0x9456fbcc) function
        pub fn withdraw_erc20(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 86, 251, 204], (token, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0x690d8320) function
        pub fn withdraw_eth(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 13, 131, 32], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniversalRouterExecutor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotWhitelisted` with signature `CallerNotWhitelisted()` and selector `0x8c6e5d71`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "CallerNotWhitelisted", abi = "CallerNotWhitelisted()")]
    pub struct CallerNotWhitelisted;
    ///Custom Error type `MsgSenderNotReactor` with signature `MsgSenderNotReactor()` and selector `0x933fe52f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MsgSenderNotReactor", abi = "MsgSenderNotReactor()")]
    pub struct MsgSenderNotReactor;
    ///Custom Error type `NativeTransferFailed` with signature `NativeTransferFailed()` and selector `0xf4b3b1bc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum UniversalRouterExecutorErrors {
        CallerNotWhitelisted(CallerNotWhitelisted),
        MsgSenderNotReactor(MsgSenderNotReactor),
        NativeTransferFailed(NativeTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for UniversalRouterExecutorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CallerNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallerNotWhitelisted(decoded));
            }
            if let Ok(decoded) = <MsgSenderNotReactor as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MsgSenderNotReactor(decoded));
            }
            if let Ok(decoded) = <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniversalRouterExecutorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MsgSenderNotReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for UniversalRouterExecutorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotWhitelisted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MsgSenderNotReactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for UniversalRouterExecutorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotWhitelisted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MsgSenderNotReactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for UniversalRouterExecutorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerNotWhitelisted> for UniversalRouterExecutorErrors {
        fn from(value: CallerNotWhitelisted) -> Self {
            Self::CallerNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<MsgSenderNotReactor> for UniversalRouterExecutorErrors {
        fn from(value: MsgSenderNotReactor) -> Self {
            Self::MsgSenderNotReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for UniversalRouterExecutorErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes),bytes)` and selector `0xd0f2d8ac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "execute", abi = "execute((bytes,bytes),bytes)")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch((bytes,bytes)[],bytes)` and selector `0xe5135ec6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeBatch", abi = "executeBatch((bytes,bytes)[],bytes)")]
    pub struct ExecuteBatchCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permit2` function with signature `permit2()` and selector `0x12261ee7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "permit2", abi = "permit2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `reactor` function with signature `reactor()` and selector `0xab572650`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reactor", abi = "reactor()")]
    pub struct ReactorCall;
    ///Container type for all input parameters for the `reactorCallback` function with signature `reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)` and selector `0x585da628`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "reactorCallback",
        abi = "reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)"
    )]
    pub struct ReactorCallbackCall {
        pub p0: ::std::vec::Vec<ResolvedOrder>,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `universalRouter` function with signature `universalRouter()` and selector `0x35a9e4df`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "universalRouter", abi = "universalRouter()")]
    pub struct UniversalRouterCall;
    ///Container type for all input parameters for the `withdrawERC20` function with signature `withdrawERC20(address,address)` and selector `0x9456fbcc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawERC20", abi = "withdrawERC20(address,address)")]
    pub struct WithdrawERC20Call {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH(address)` and selector `0x690d8320`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawETH", abi = "withdrawETH(address)")]
    pub struct WithdrawETHCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum UniversalRouterExecutorCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        Owner(OwnerCall),
        Permit2(Permit2Call),
        Reactor(ReactorCall),
        ReactorCallback(ReactorCallbackCall),
        TransferOwnership(TransferOwnershipCall),
        UniversalRouter(UniversalRouterCall),
        WithdrawERC20(WithdrawERC20Call),
        WithdrawETH(WithdrawETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniversalRouterExecutorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) = <ReactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reactor(decoded));
            }
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniversalRouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniversalRouter(decoded));
            }
            if let Ok(decoded) = <WithdrawERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawERC20(decoded));
            }
            if let Ok(decoded) = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniversalRouterExecutorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Reactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniversalRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniversalRouterExecutorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniversalRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for UniversalRouterExecutorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for UniversalRouterExecutorCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for UniversalRouterExecutorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for UniversalRouterExecutorCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<ReactorCall> for UniversalRouterExecutorCalls {
        fn from(value: ReactorCall) -> Self {
            Self::Reactor(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for UniversalRouterExecutorCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for UniversalRouterExecutorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniversalRouterCall> for UniversalRouterExecutorCalls {
        fn from(value: UniversalRouterCall) -> Self {
            Self::UniversalRouter(value)
        }
    }
    impl ::core::convert::From<WithdrawERC20Call> for UniversalRouterExecutorCalls {
        fn from(value: WithdrawERC20Call) -> Self {
            Self::WithdrawERC20(value)
        }
    }
    impl ::core::convert::From<WithdrawETHCall> for UniversalRouterExecutorCalls {
        fn from(value: WithdrawETHCall) -> Self {
            Self::WithdrawETH(value)
        }
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `permit2` function with signature `permit2()` and selector `0x12261ee7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Permit2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `reactor` function with signature `reactor()` and selector `0xab572650`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReactorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `universalRouter` function with signature `universalRouter()` and selector `0x35a9e4df`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UniversalRouterReturn(pub ::ethers::core::types::Address);
}
