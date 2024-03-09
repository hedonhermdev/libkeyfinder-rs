#include "keyfinder/libkeyfinder/src/keyfinder.h"
#include "keyfinder/include/key.hpp"
#include "keyfinder/src/lib.rs.h"
#include <memory>

Key find_key(const PcmAudioData &data) {
  // Static because it retains useful resources for repeat use
  static KeyFinder::KeyFinder k;

  // Build an empty audio object
  KeyFinder::AudioData a;

  // Prepare the object for your audio stream
  a.setFrameRate(data.sample_rate);
  a.setChannels(data.channels);
  a.addToSampleCount(data.length);

  // Copy your audio into the object
  for (int i = 0; i < data.length; i++) {
    a.setSample(i, data.samples[i]);
  }

  int key = k.keyOfAudio(a);

  return static_cast<Key>(key);
}
