# Arm Firmware Interfaces for Realm Management Extension

This crate implements user-friendly types and their encodings from the [Arm Firmware Interfaces for
Realm Management Extension](https://developer.arm.com/documentation/den0149/latest/) (FIRME),
version 1.0.

*Please note that the implemented specification document (DEN0149) is not Release quality, and is
subject to change.*

## Implemented features

- FIRME_SERVICE_VERSION ABI
- FIRME_SERVICE_FEATURES ABI
- FIRME_GM_GPI_SET ABI
- FIRME_GM_GPI_OP_CONTINUE ABI
- FIRME_GM_L1_GPT_CREATE ABI
- FIRME_GM_L1_GPT_DESTROY ABI

## Future plans

* Implementing all calls

## License

The project is MIT and Apache-2.0 dual licensed, see `LICENSE-Apache-2.0` and `LICENSE-MIT`.

## Maintainers

arm-firme is a trustedfirmware.org maintained project. All contributions are ultimately merged by
the maintainers listed below.

* Bálint Dobszay <balint.dobszay@arm.com>
  [balint-dobszay-arm](https://github.com/balint-dobszay-arm)
* Imre Kis <imre.kis@arm.com>
  [imre-kis-arm](https://github.com/imre-kis-arm)
* Sandrine Afsa <sandrine.afsa@arm.com>
  [sandrine-bailleux-arm](https://github.com/sandrine-bailleux-arm)

## Contributing

Please follow the directions of the [Trusted Firmware Processes](https://trusted-firmware-docs.readthedocs.io/en/latest/generic_processes/index.html)

Contributions are handled through [review.trustedfirmware.org](https://review.trustedfirmware.org/q/project:arm-firmware-crates/arm-firme).

## Arm trademark notice

Arm is a registered trademark of Arm Limited (or its subsidiaries or affiliates).

This project uses some of the Arm product, service or technology trademarks, as listed in the
[Trademark List][1], in accordance with the [Arm Trademark Use Guidelines][2].

Subsequent uses of these trademarks throughout this repository do not need to be prefixed with the
Arm word trademark.

[1]: https://www.arm.com/company/policies/trademarks/arm-trademark-list
[2]: https://www.arm.com/company/policies/trademarks/guidelines-trademarks

--------------

*Copyright The arm-firme Contributors.*
