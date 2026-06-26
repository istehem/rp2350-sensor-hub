import { test, expect, Page } from '@playwright/test'
import { config } from './constants'

const mockLatestMeasurementSuccess = async (page: Page, temperature: number, humidity: number) => {
  await page.route('**/api/measurements/latest', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        date: '2026-06-23T20:38:43.688746298Z',
        temperature: temperature,
        humidity: humidity,
      }),
    })
  })
}

test('has title', async ({ page }) => {
  await page.goto(config.homeUrl)
  await expect(page).toHaveTitle(/Measurements/)
})

test('has navigation', async ({ page }) => {
  await page.goto(config.homeUrl)
  await expect(page.locator('nav').getByText('Measurements')).toBeVisible()
})

test('mock latest measurement success', async ({ page }) => {
  const temperature = 33.1
  const humidity = 33.2
  mockLatestMeasurementSuccess(page, temperature, humidity)
  await page.goto(config.homeUrl)

  await expect(page.locator('article').getByText(`${temperature}°C`, { exact: true })).toBeVisible()
  await expect(page.locator('article').getByText(`${humidity}%`, { exact: true })).toBeVisible()
})

test('mock latest measurement with error', async ({ page }) => {
  const error = 'this is a server error'
  await page.route('**/api/measurements/latest', async (route) => {
    await route.fulfill({
      status: 500,
      contentType: 'application/json',
      body: JSON.stringify({ message: error }),
    })
  })

  await page.goto(config.homeUrl)

  await expect(page.locator(`article:has(:text-is("${error}"))`)).toBeVisible()
  await expect(page.getByText('Temperature')).toHaveCount(0)
  await expect(page.getByText('Humidity')).toHaveCount(0)
})

test('mock measurements success', async ({ page }) => {
  const temperature = 33.1
  const humidity = 33.2
  await mockLatestMeasurementSuccess(page, temperature, humidity)
  await page.route(/\/api\/measurements(\?.*)?$/, async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify([
        { date: '2026-06-23T20:38:43.688746298Z', temperature: temperature, humidity: humidity },
        { date: '2026-06-23T20:38:53.688746298Z', temperature: temperature, humidity: humidity },
        { date: '2026-06-23T20:39:03.688746298Z', temperature: temperature, humidity: humidity },
      ]),
    })
  })

  await page.goto(config.homeUrl)
  await expect(page.locator('article canvas[role=img]')).toHaveCount(2)
})

test('mock measurements with error', async ({ page }) => {
  await mockLatestMeasurementSuccess(page, 33, 33)
  const error = 'measurements failed to load with server error'
  await page.route(/\/api\/measurements(\?.*)?$/, async (route) => {
    await route.fulfill({
      status: 500,
      contentType: 'application/json',
      body: JSON.stringify({
        message: error,
      }),
    })
  })

  await page.goto(config.homeUrl)
  await expect(page.locator(`article article:has(:text-is("${error}"))`)).toHaveCount(2)
})
