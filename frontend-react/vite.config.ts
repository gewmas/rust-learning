import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // Allow serving files from one level up (the project root) 
      // where the rust-wasm-lib lives
      allow: ['..']
    }
  }
})
