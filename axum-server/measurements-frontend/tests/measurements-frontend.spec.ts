import { test, expect } from '@playwright/test';
import { config } from './constants';

test('has title', async ({ page }) => {
  await page.goto(config.homeUrl);
  await expect(page).toHaveTitle(/Measurements/);
});

test('has navigation', async ({ page }) => {
  await page.goto(config.homeUrl);
  await expect(page.locator('nav').getByText('Measurements')).toBeVisible();
});

test('mock latest measurement success', async ({ page }) => {
  const temperature = 33.0;
  const humidity = 33.33;
  await page.route('**/api/measurements/latest', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify([
        { "date": "2026-06-23T20:38:43.688746298Z", "temperature": temperature, "humidity": humidity }
      ])
    });
  });

  await page.goto(config.homeUrl);

  await expect(page.locator('article').getByText(temperature.toString())).toBeVisible();
  await expect(page.locator('article').getByText(humidity.toString())).toBeVisible();
});

test('mock latest measurement error', async ({ page }) => {
  const error = "this is a server error";
  await page.route('**/api/measurements/latest', async (route) => {
    await route.fulfill({
      status: 500,
      contentType: 'application/json',
      body: JSON.stringify([
        { "message": error }
      ])
    });
  });

  await page.goto(config.homeUrl);

  await expect(page.locator('article[class*="error"]').getByText(error)).toBeVisible();
  await expect(page.getByText('Temperature')).toHaveCount(0);
  await expect(page.getByText('Humidity')).toHaveCount(0);
});
