import { defineConfig, devices } from '@playwright/test'
import * as dotenv from 'dotenv'

// Load environment variables from .env file
dotenv.config()

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    video: 'on',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'safari',
      use: { ...devices['Desktop Safari'] },
    },
    {
      name: 'iPhone 15',
      use: { ...devices['iPhone 15'] },
    },
    {
      name: 'Pixel 7',
      use: { ...devices['Pixel 7'] },
    },
    {
      name: 'MacBook 13',
      use: {
        viewport: { width: 1280, height: 800 },
        deviceScaleFactor: 2,
      },
    },
  ],

  webServer: {
    command: 'pnpm build && pnpm run preview',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    env: {
      BACKEND_ENDPOINT: process.env.BACKEND_ENDPOINT || 'http://localhost:3001',
      IMAGES_BASE_URL: process.env.IMAGES_BASE_URL || 'http://localhost:3001',
      TYPESENSE_HOST: process.env.TYPESENSE_HOST || 'localhost',
      TYPESENSE_PORT: process.env.TYPESENSE_PORT || '8109',
      TYPESENSE_API_KEY: process.env.TYPESENSE_API_KEY || 'resin_development',
      MESSAGES_NPUB: process.env.MESSAGES_NPUB || '',
      BTCPAY_BASE_URL: process.env.BTCPAY_BASE_URL || 'https://btcpay.resin.estate',
    },
  },
}) 