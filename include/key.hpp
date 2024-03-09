#include "keyfinder/libkeyfinder/src/keyfinder.h"
#include <vector>

#pragma once
#include "rust/cxx.h"

enum Key {
    A_MAJOR = 0,
    A_MINOR,
    B_FLAT_MAJOR,
    B_FLAT_MINOR,
    B_MAJOR,
    B_MINOR = 5,
    C_MAJOR,
    C_MINOR,
    D_FLAT_MAJOR,
    D_FLAT_MINOR,
    D_MAJOR = 10,
    D_MINOR,
    E_FLAT_MAJOR,
    E_FLAT_MINOR,
    E_MAJOR,
    E_MINOR = 15,
    F_MAJOR,
    F_MINOR,
    G_FLAT_MAJOR,
    G_FLAT_MINOR,
    G_MAJOR = 20,
    G_MINOR,
    A_FLAT_MAJOR,
    A_FLAT_MINOR,
    SILENCE = 24
};

struct PcmAudioData;

Key find_key(const PcmAudioData &);
