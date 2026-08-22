import { createApp } from 'vue'
import App from './App.vue'
import 'beercss'
import './main.css'
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  PointElement,
  LineElement,
  TimeScale,
  LinearScale,
  Filler,
} from 'chart.js'

import 'chartjs-adapter-date-fns'

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, LinearScale, TimeScale, Filler)
createApp(App).mount('#app')
