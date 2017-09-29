#include "squish.h"
#include <stdint.h>

extern "C" {
void decompress(squish::u8* rgba, void const* block, int flags) {
  squish::Decompress(rgba, block, flags);
}

void DecompressImage(squish::u8* rgba, int width, int height, void const* blocks, int flags) {
  squish::DecompressImage(rgba, width, height, blocks, flags);
}

void getStorageRequirements(int width, int height, int flags) {
  squish::GetStorageRequirements(width, height, flags);
}
}
