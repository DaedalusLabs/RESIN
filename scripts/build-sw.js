import { build } from 'esbuild';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function buildServiceWorker() {
  try {
    await build({
      entryPoints: [resolve(__dirname, '../public/ndk-bundle.js')],
      bundle: true,
      outfile: resolve(__dirname, '../public/ndk-bundle.min.js'),
      format: 'iife',
      platform: 'browser',
      minify: true,
      target: ['es2020'],
      define: {
        'process.env.NODE_ENV': '"production"',
        'global': 'self'
      },
      loader: {
        '.js': 'jsx'
      }
    });
    console.log('Service worker bundle built successfully!');
  } catch (error) {
    console.error('Build failed:', error);
    process.exit(1);
  }
}

buildServiceWorker(); 