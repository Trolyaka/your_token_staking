// @flow

import * as BufferLayout from '@solana/buffer-layout';

/**
 * Layout for a public key
 */
export const publicKey = (property = 'publicKey'): Object => {
  return BufferLayout.blob(32, property);
};

/**
 * Layout for a 64bit unsigned value
 */
export const uint64 = (property = 'uint64'): Object => {
  return BufferLayout.blob(8, property);
};
