<script setup lang="ts">
import * as E from 'fp-ts/Either'
import * as O from 'fp-ts/Option'
import * as S from 'fp-ts/State'
import * as T from 'fp-ts/Task'
import * as TO from 'fp-ts/TaskOption'
import { pipe } from 'fp-ts/function'
import type { Option } from 'fp-ts/Option'
import { computed, ref, onMounted } from 'vue'

import type { ApiError, Measurement } from './assets.ts'
import TemperatureChart from './charts/TemperatureChart.vue'
import HumidityChart from './charts/HumidityChart.vue'
import ErrorPanel from './ErrorPanel.vue'
import { fetchLatestMeasurement, fetchMeasurements } from './measurementsApi.ts'

const switchModeIcon = ref<string>('dark_mode')

interface Colors {
  primary: string
  secondary: string
  surfaceVariant: string
}

interface AppState {
  latestMeasurement: Option<Measurement>
  latestMeasurementApiError: Option<ApiError>
  measurements: Measurement[]
  measurementsApiError: Option<ApiError>
  colors: Colors
}

const initialState: AppState = {
  latestMeasurement: O.none,
  latestMeasurementApiError: O.none,
  measurements: [],
  measurementsApiError: O.none,
  colors: {
    primary: '#cfbcff',
    secondary: '#cbc2db',
    surfaceVariant: '#49454e',
  },
}
const state = ref(initialState)

const setLatestMeasurement = (measurement: Option<Measurement>) =>
  S.modify((s: AppState) => ({ ...s, latestMeasurement: measurement }))

const setLatestMeasurementApiError = (error: Option<ApiError>) =>
  S.modify((s: AppState) => ({ ...s, latestMeasurementApiError: error }))

const setMeasurements = (measurements: Measurement[]) =>
  S.modify((s: AppState) => ({ ...s, measurements: measurements }))

const setMeasurementsApiError = (error: Option<ApiError>) =>
  S.modify((s: AppState) => ({ ...s, measurementsApiError: error }))

const setColors = (colors: Colors) => S.modify((s: AppState) => ({ ...s, colors: colors }))

const updateAppState = (f: (s: AppState) => [unknown, AppState]) => {
  const [, newState] = f(state.value)
  state.value = newState
}

/**
 * This causes a side effect.
 */
const transferStateToVue = (f: (state: AppState) => [unknown, AppState]): T.Task<void> =>
  T.fromIO(() => updateAppState(f))

async function getCssColor(color: string, fallbackColor: string): Promise<string> {
  try {
    const mode = (await ui('mode')) as 'light' | 'dark' | undefined
    const theme = await getTheme(mode || 'dark')()

    if (O.isNone(theme)) {
      return fallbackColor
    }
    const themeCss = pipe(
      theme,
      O.getOrElse(() => ''),
    )

    const varRe = new RegExp(`--${color}\\s*:\\s*([^;]+);?`)
    const m = themeCss.match(varRe)
    const raw = m?.[1]?.trim()
    return raw || fallbackColor
  } catch {
    return fallbackColor
  }
}

const getTheme = (mode: 'light' | 'dark'): TO.TaskOption<string> =>
  pipe(
    TO.tryCatch(() => Promise.resolve(ui('theme'))),
    TO.chain(
      TO.fromPredicate(
        (theme): theme is IBeerCssTheme =>
          typeof theme === 'object' && theme !== null && mode in theme,
      ),
    ),
    TO.map((theme) => theme[mode]),
  )

async function toggleSwitchModeIcon() {
  const mode = await ui('mode')
  if (mode === 'light') {
    switchModeIcon.value = 'dark_mode'
  } else {
    switchModeIcon.value = 'light_mode'
  }
  const primary = await getCssColor('primary', initialState.colors.primary)
  const secondary = await getCssColor('secondary', initialState.colors.secondary)
  const surfaceVariant = await getCssColor('surface-variant', initialState.colors.surfaceVariant)
  updateAppState(
    setColors({
      primary,
      secondary,
      surfaceVariant,
    }),
  )
}

async function flipMode() {
  const mode = await ui('mode')
  if (mode === 'light') {
    ui('mode', 'dark')
  } else {
    ui('mode', 'light')
  }
  await toggleSwitchModeIcon()
}

const poll = (task: T.Task<void>, delayMs: number): T.Task<never> =>
  pipe(
    task,
    T.chain(() => T.delay(delayMs)(poll(task, delayMs))),
  )

const handleLatestMeasurement = (): T.Task<void> =>
  pipe(
    fetchLatestMeasurement(),
    T.chain((latestMeasurement) =>
      pipe(
        latestMeasurement,
        E.match(
          (error) => transferStateToVue(setLatestMeasurementApiError(O.some(error))),
          (success) =>
            transferStateToVue(
              S.sequenceArray([
                setLatestMeasurementApiError(O.none),
                setLatestMeasurement(O.some(success)),
              ]),
            ),
        ),
      ),
    ),
  )

const handleMeasurements = (): T.Task<void> =>
  pipe(
    fetchMeasurements(),
    T.chain((latestMeasurement) =>
      pipe(
        latestMeasurement,
        E.match(
          (error) => transferStateToVue(setMeasurementsApiError(O.some(error))),
          (success) =>
            transferStateToVue(
              S.sequenceArray([setMeasurementsApiError(O.none), setMeasurements(success)]),
            ),
        ),
      ),
    ),
  )

onMounted(async () => {
  toggleSwitchModeIcon()
  poll(handleLatestMeasurement(), 10000)()
  poll(handleMeasurements(), 60000)()
})

const latestMeasurementData = computed(() =>
  pipe(
    state.value.latestMeasurement,
    O.match(
      () => null,
      (measurement) => measurement,
    ),
  ),
)
const latestMeasurementError = computed(() =>
  pipe(
    state.value.latestMeasurementApiError,
    O.match(
      () => null,
      (error) => error,
    ),
  ),
)

const measurements = computed(() => state.value.measurements)
const measurementsError = computed(() => state.value.measurementsApiError)

const colors = computed(() => state.value.colors)
</script>

<template>
  <header class="fixed">
    <nav>
      <button class="circle transparent primary-text" @click="flipMode">
        <i>{{ switchModeIcon }}</i>
      </button>
      <h6 class="max">Measurements</h6>
    </nav>
  </header>
  <main class="responsive">
    <ErrorPanel v-if="latestMeasurementError" :error="latestMeasurementError" />
    <div v-else-if="latestMeasurementData">
      <article>
        <div class="grid shrink-center">
          <div class="s6 m6 l6">
            <h6>Date:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurementData.date.toLocaleDateString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Time:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurementData.date.toLocaleTimeString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Temperature:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurementData.temperature.toFixed(1) }}°C</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Humidity:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurementData.humidity.toFixed(1) }}%</h6>
          </div>
        </div>
      </article>
      <article class="medium">
        <TemperatureChart
          :measurements="measurements"
          :api-error="measurementsError"
          :dataset-color="colors.primary"
          :text-color="colors.secondary"
          :grid-color="colors.surfaceVariant"
        />
      </article>
      <article class="medium">
        <HumidityChart
          :measurements="measurements"
          :api-error="measurementsError"
          :dataset-color="colors.primary"
          :text-color="colors.secondary"
          :grid-color="colors.surfaceVariant"
        />
      </article>
    </div>
    <article v-else class="center-align">
      <progress class="circle small indeterminate" value="50" max="100"></progress>
    </article>
  </main>
</template>

<style scoped></style>
