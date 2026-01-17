import { createApp } from 'vue'
import App from './App.vue'
import 'beercss'
import './main.css'
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale,
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)
createApp(App).mount('#app')
