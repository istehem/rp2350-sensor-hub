<script setup lang="ts">
import 'd3'
import * as O from 'fp-ts/Option'
import type { Option } from 'fp-ts/Option'
import { pipe } from 'fp-ts/function'
import { computed, getCurrentInstance, onMounted, watch } from 'vue'

import { unknownError } from '../assets.ts'
import type { ApiError, Measurement } from '../assets.ts'
import ErrorPanel from '../ErrorPanel.vue'

const properties = defineProps<{
  measurements: Measurement[]
  apiError: Option<ApiError>
  datasetColor: string
  textColor: string
  gridColor: string
}>()

import * as d3 from 'd3'

const title = 'Humidity (%)'

let width = 1152
let height = 288

function createChart(aapl: Measurement[]) {
  const marginTop = 0
  const marginRight = 0
  const marginBottom = 32
  const marginLeft = 32

  const x = d3
    .scaleUtc()
    .domain(d3.extent(aapl, (d) => d.date) as [Date, Date])
    .range([marginLeft, width - marginRight])

  const y = d3
    .scaleLinear<number, number>()
    .domain([0, d3.max(aapl, (d) => d.humidity)!])
    .range([height - marginBottom, marginTop])

  const line = d3
    .line<Measurement>()
    .x((d) => x(d.date)!)
    .y((d) => y(d.humidity))

  const svg = d3
    .create('svg')
    .attr('width', '100%')
    .attr('height', '100%')
    .attr('viewBox', [0, 0, width, height])
    .attr('style', 'max-width: 100%; height: auto; height: intrinsic;')

  svg
    .append('g')
    .attr('transform', `translate(0,${height - marginBottom})`)
    .call(
      d3
        .axisBottom(x)
        .ticks(width / 80)
        .tickSizeOuter(0),
    )

  svg
    .append('g')
    .attr('transform', `translate(${marginLeft},0)`)
    .call(d3.axisLeft(y).ticks(height / 40))
    .call((g) => g.select('.domain').remove())
    .call((g) =>
      g
        .selectAll('.tick line')
        .clone()
        .attr('x2', width - marginLeft - marginRight)
        .attr('stroke', properties.gridColor),
    )
    .call((g) =>
      g
        .append('text')
        .attr('x', width / 2)
        .attr('y', 10)
        .attr('fill', properties.textColor)
        .attr('text-anchor', 'middle')
        .text(title),
    )

  svg
    .append('path')
    .attr('fill', 'none')
    .attr('stroke', properties.datasetColor)
    .attr('stroke-width', 2)
    .attr('d', line(aapl))

  return svg.node()
}

onMounted(async () => {
  const instance = getCurrentInstance()
  const parentElement = instance?.proxy?.$el.parentElement
  width = parentElement?.clientWidth || width
  height = parentElement?.clientHeight || height
})

watch(properties, async () => {
  const container = d3.select('#chart')
  const chart = createChart(properties.measurements || [])
  const node = container.node() as HTMLElement | null
  if (node && chart) {
    container.select('svg').remove()
    node.appendChild(chart)
  }
})

const error = computed(() =>
  pipe(
    properties.apiError,
    O.match(
      () => unknownError,
      (error) => error,
    ),
  ),
)
</script>

<template>
  <ErrorPanel v-if="O.isSome(apiError)" :error="error" />
  <div v-else id="chart" />
</template>
