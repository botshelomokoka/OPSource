import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node'
  },
  resolve: {
    alias: {
      '@core': resolve(__dirname, './src/core'),
      '@shared': resolve(__dirname, './src/core/shared'),
      '@tests': resolve(__dirname, './tests')
    }
  }
});