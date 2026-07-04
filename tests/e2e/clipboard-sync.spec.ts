import { expect, test, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';

async function placeWindow(page: Page, left: number, top: number, width: number, height: number) {
  const session = await page.context().newCDPSession(page);
  const { windowId } = await session.send('Browser.getWindowForTarget');
  await session.send('Browser.setWindowBounds', {
    windowId,
    bounds: { left, top, width, height, windowState: 'normal' },
  });
  await session.detach();
}

async function peerCount(page: Page) {
  return page.evaluate(() => window.__serverClipboard?.peerCount?.() ?? 0);
}

test('left browser sends clipboard and file to right browser over WebRTC data channel', async ({ browser, baseURL }, testInfo) => {
  const room = `e2e-${Date.now()}`;
  const roomUrl = `${baseURL}/${room}`;
  const payload = `clipboard from left page ${Date.now()}`;

  const leftContext = await browser.newContext({
    viewport: { width: 760, height: 720 },
    permissions: ['clipboard-read', 'clipboard-write'],
    recordVideo: { dir: testInfo.outputPath('left-video') },
  });
  const rightContext = await browser.newContext({
    viewport: { width: 760, height: 720 },
    permissions: ['clipboard-read', 'clipboard-write'],
    recordVideo: { dir: testInfo.outputPath('right-video') },
  });

  try {
    await leftContext.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });
    await rightContext.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });

    const left = await leftContext.newPage();
    const right = await rightContext.newPage();

    await test.step('open two visible room pages and wait for p2p channel', async () => {
      await Promise.all([left.goto(roomUrl), right.goto(roomUrl)]);
      await Promise.all([
        expect(left.getByTestId('empty-history')).toBeVisible(),
        expect(right.getByTestId('empty-history')).toBeVisible(),
      ]);

      const iceConfig = await left.evaluate(() => fetch('/clip_bridge_server/config').then((res) => res.json()));
      expect(iceConfig.iceServers).toEqual(
        expect.arrayContaining([
          expect.objectContaining({
            urls: 'stun:127.0.0.1:34780',
          }),
          expect.objectContaining({
            urls: 'stun:stun.miwifi.com:3478',
          }),
          expect.objectContaining({
            urls: 'stun:stun.chat.bilibili.com:3478',
          }),
          expect.objectContaining({
            urls: 'turn:127.0.0.1:34780',
            username: 'e2e',
            credential: 'e2e',
          }),
        ]),
      );

      await Promise.all([
        placeWindow(left, 40, 40, 780, 760),
        placeWindow(right, 860, 40, 780, 760),
      ]);

      await expect.poll(() => peerCount(left)).toBe(1);
      await expect.poll(() => peerCount(right)).toBe(1);
      await Promise.all([
        expect(left.getByTestId('room-count')).toHaveText('2 in room'),
        expect(right.getByTestId('room-count')).toHaveText('2 in room'),
      ]);
      await left.getByTestId('help-open').click();
      await expect(left.getByTestId('help-overlay')).toHaveClass(/show/);
      await expect(left.getByTestId('help-overlay')).toContainText('Click blank space');
      await left.getByTestId('help-close').click();
      await expect(left.getByTestId('help-overlay')).not.toHaveClass(/show/);

      await testInfo.attach('01-p2p-ready-left.png', {
        body: await left.screenshot({ fullPage: true }),
        contentType: 'image/png',
      });
      await testInfo.attach('01-p2p-ready-right.png', {
        body: await right.screenshot({ fullPage: true }),
        contentType: 'image/png',
      });
    });

    await test.step('left blank click reads local clipboard and sends it directly to right page', async () => {
      await left.evaluate(async (text) => {
        await navigator.clipboard.writeText(text);
      }, payload);

      await left.mouse.click(5, 700);

      await expect(left.getByTestId('toast')).toHaveText('Sent via P2P');
      await expect(left.getByTestId('history-row')).toHaveCount(1);
      await expect(right.getByTestId('history-row')).toHaveCount(1);
      await expect(right.getByTestId('history-select')).toBeChecked();

      await left.mouse.click(5, 700);
      await expect(left.getByTestId('toast')).toHaveText('No clipboard changes');
      await expect(right.getByTestId('history-row')).toHaveCount(1);

      await testInfo.attach('02-right-received-p2p.png', {
        body: await right.screenshot({ fullPage: true }),
        contentType: 'image/png',
      });
    });

    await test.step('right page writes the selected p2p text into its local clipboard', async () => {
      await right.evaluate(async () => {
        await navigator.clipboard.writeText('right clipboard before receive');
      });

      await right.getByTestId('history-row').click();

      await expect(right.getByTestId('toast')).toHaveText('Copied to local clipboard');
      await expect
        .poll(() => right.evaluate(() => navigator.clipboard.readText()))
        .toBe(payload);
    });

    await test.step('left page offers a file and right page downloads it over p2p', async () => {
      const fileName = 'p2p-note.txt';
      const fileBody = `file from left page ${Date.now()}`;
      await left.getByTestId('file-input').setInputFiles({
        name: fileName,
        mimeType: 'text/plain',
        buffer: Buffer.from(fileBody, 'utf8'),
      });

      await expect(right.getByTestId('file-card')).toHaveClass(/show/);
      await expect(right.getByTestId('file-name')).toHaveText(fileName);
      await expect(right.getByTestId('file-meta')).toContainText(/direct|TURN relay|checking connection/);
      await expect(right.getByTestId('transport-badge')).toContainText(/direct|TURN relay|checking connection/);
      await expect(right.getByTestId('transport-badge')).toHaveClass(/transport-badge/);

      const downloadPromise = right.waitForEvent('download');
      await right.getByTestId('file-download').click();
      const download = await downloadPromise;
      const savePath = testInfo.outputPath(fileName);
      await download.saveAs(savePath);
      await expect.poll(async () => readFile(savePath, 'utf8')).toBe(fileBody);

      await testInfo.attach('03-right-file-downloaded.png', {
        body: await right.screenshot({ fullPage: true }),
        contentType: 'image/png',
      });
    });

    await test.step('clear only clears the local page', async () => {
      await right.getByTestId('clear').click();

      await expect(right.getByTestId('empty-history')).toBeVisible();
      await expect(left.getByTestId('history-row')).toHaveCount(1);

      await testInfo.attach('04-right-cleared-only-local.png', {
        body: await right.screenshot({ fullPage: true }),
        contentType: 'image/png',
      });
    });
  } finally {
    await leftContext.close();
    await rightContext.close();
  }
});
