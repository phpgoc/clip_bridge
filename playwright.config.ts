import { defineConfig, devices } from '@playwright/test';

const port = Number(process.env.E2E_PORT ?? 7359);
const baseURL = `http://127.0.0.1:${port}`;

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30_000,
  expect: {
    timeout: 8_000,
  },
  fullyParallel: false,
  workers: 1,
  reporter: [
    ['list'],
    ['html', { open: 'never' }],
  ],
  use: {
    ...devices['Desktop Chrome'],
    baseURL,
    headless: false,
    launchOptions: {
      slowMo: Number(process.env.E2E_SLOW_MO ?? 120),
    },
    screenshot: 'on',
    trace: 'on',
    video: 'on',
  },
  webServer: {
    command: `cargo run -- --bind 127.0.0.1:${port}`,
    url: baseURL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
    stdout: 'pipe',
    stderr: 'pipe',
  },
  projects: [
    {
      name: 'chromium-observable',
      use: {
        browserName: 'chromium',
      },
    },
  ],
  outputDir: 'test-results',
});
