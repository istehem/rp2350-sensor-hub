<script setup lang="ts">
import * as A from 'fp-ts/Apply'
import * as E from 'fp-ts/Either'
import * as O from 'fp-ts/Option'
import * as S from 'fp-ts/State'
import * as T from 'fp-ts/Task'
import * as TO from 'fp-ts/TaskOption'
import { pipe } from 'fp-ts/function'
import { computed, ref, onMounted } from 'vue'

import * as AS from './appState.ts'
import config from './config.ts'
import type { AppState, Colors, Mode } from './appState.ts'
import TemperatureChart from './charts/TemperatureChart.vue'
import HumidityChart from './charts/HumidityChart.vue'
import ErrorPanel from './ErrorPanel.vue'
import { fetchLatestMeasurement, fetchMeasurements } from './measurementsApi.ts'

const state = ref(AS.initialState)

const updateAppState = (f: (s: AppState) => [unknown, AppState]) => {
  const [, newState] = f(state.value)
  state.value = newState
}

/**
 * This causes a side effect.
 */
const transferStateToVue = (f: (state: AppState) => [unknown, AppState]): T.Task<void> =>
  T.fromIO(() => updateAppState(f))

const getCssColorOrDefault = (color: string, fallbackColor: string): TO.TaskOption<string> =>
  pipe(
    getCssColor(color),
    TO.chain((option) =>
      pipe(
        option,
        O.match(
          () => TO.some(fallbackColor),
          (color) => TO.some(color),
        ),
      ),
    ),
  )

const getCssColor = (color: string): TO.TaskOption<O.Option<string>> =>
  pipe(
    getMode(),
    TO.chain((mode) => getThemeCss(mode)),
    TO.map((themeCss) => findColorFromCss(themeCss, color)),
  )

function findColorFromCss(themeCss: string, color: string): O.Option<string> {
  const varRe = new RegExp(`--${color}\\s*:\\s*([^;]+);?`)
  const m = themeCss.match(varRe)
  const raw = m?.[1]?.trim()
  if (raw) {
    return O.some(raw)
  }
  return O.none
}

const getMode = (): TO.TaskOption<Mode> =>
  pipe(
    TO.tryCatch(() => Promise.resolve(ui('mode'))),
    TO.chain(TO.fromPredicate((mode) => mode === 'dark' || mode == 'light')),
  )

const getThemeCss = (mode: Mode): TO.TaskOption<string> =>
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

const asColors =
  (primary: string) =>
  (secondary: string) =>
  (surfaceVariant: string): Colors => {
    return {
      primary,
      secondary,
      surfaceVariant,
    }
  }

const inverseMode = (mode: Mode): Mode => (mode === 'light' ? 'dark' : 'light')

const setColors = (): T.Task<void> =>
  pipe(
    T.of(asColors),
    T.ap(
      pipe(
        getCssColorOrDefault('primary', AS.initialState.colors.primary),
        TO.getOrElse(() => T.of(AS.initialState.colors.primary)),
      ),
    ),
    T.ap(
      pipe(
        getCssColorOrDefault('secondary', AS.initialState.colors.secondary),
        TO.getOrElse(() => T.of(AS.initialState.colors.secondary)),
      ),
    ),
    T.ap(
      pipe(
        getCssColorOrDefault('surface-variant', AS.initialState.colors.surfaceVariant),
        TO.getOrElse(() => T.of(AS.initialState.colors.surfaceVariant)),
      ),
    ),
    T.chain((colors) => transferStateToVue(AS.setColors(colors))),
  )

const adaptToMode = (): T.Task<void> =>
  pipe(
    A.sequenceT(T.ApplyPar)(
      pipe(
        getMode(),
        T.chain((mode) =>
          pipe(
            mode,
            O.match(
              () => 'dark',
              (mode) => inverseMode(mode),
            ),
            T.of,
          ),
        ),
        T.chain((mode) => transferStateToVue(AS.setMode(mode as Mode))),
      ),
      setColors(),
    ),
    T.map(() => {}),
  )

const setMode = (mode: Mode) => {
  ui('mode', mode)
}

const toggleMode = (): T.Task<void> =>
  pipe(
    getMode(),
    T.chain((mode) =>
      pipe(
        mode,
        O.match(
          () => 'dark',
          (mode) => inverseMode(mode),
        ),
        T.of,
      ),
    ),
    T.chain((mode) => T.of(setMode(mode as Mode))),
  )

async function onToggleModeClicked() {
  await pipe(
    toggleMode(),
    T.chain(() => adaptToMode()),
  )()
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
          (error) => transferStateToVue(AS.setLatestMeasurementApiError(O.some(error))),
          (success) =>
            transferStateToVue(
              S.sequenceArray([
                AS.setLatestMeasurementApiError(O.none),
                AS.setLatestMeasurement(O.some(success)),
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
          (error) => transferStateToVue(AS.setMeasurementsApiError(O.some(error))),
          (success) =>
            transferStateToVue(
              S.sequenceArray([AS.setMeasurementsApiError(O.none), AS.setMeasurements(success)]),
            ),
        ),
      ),
    ),
  )

onMounted(async () => {
  await adaptToMode()()
  poll(handleLatestMeasurement(), config.latestMeasurement.pollEvery)()
  poll(handleMeasurements(), config.measurements.pollEvery)()
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

const toggleModeIcon = computed(() => (state.value.mode === 'light' ? 'dark_mode' : 'light_mode'))
</script>

<template>
  <header class="fixed">
    <nav>
      <button class="circle transparent primary-text" @click="onToggleModeClicked">
        <i>{{ toggleModeIcon }}</i>
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
