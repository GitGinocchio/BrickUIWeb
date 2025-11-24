import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'

import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import { cloudflare } from "@cloudflare/vite-plugin"

// Plugin custom per watchare Rust
import { execSync } from 'child_process'

function rustWatcher() {
  return {
    name: 'rust-watcher',
    handleHotUpdate({ file }: { file: string }) {
      if (file.endsWith('.rs')) {
        console.log(`⚡ Rust file changed: ${file}`)
        execSync('cargo install -q worker-build && worker-build --config ./wrangler.toml', { stdio: 'inherit' })
      }
    }
  }
}

// https://vite.dev/config/
export default defineConfig({
	plugins: [
		vue(),
		vueDevTools(),
		cloudflare(),
		rustWatcher()
	],
	resolve: {
		alias: {
			'@': fileURLToPath(new URL('./src', import.meta.url))
		},
	},
})
