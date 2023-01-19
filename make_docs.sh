#!/bin/bash
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features
