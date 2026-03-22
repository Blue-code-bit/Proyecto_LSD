// Importamos las herramientas necesarias para configurar Vite
import { defineConfig } from 'vite'
// Importamos el plugin de React para que Vite entienda archivos .jsx
import react from '@vitejs/plugin-react'
// Importamos los polyfills para que las librerías de Solana funcionen en el navegador
// (Solana fue diseñada para Node.js, estos polyfills la adaptan al navegador)
import { nodePolyfills } from 'vite-plugin-node-polyfills'

export default defineConfig({
  plugins: [
    // Activamos React
    react(),
    // Activamos los polyfills de Node.js para que Solana funcione en el navegador
    nodePolyfills(),
  ],
})
