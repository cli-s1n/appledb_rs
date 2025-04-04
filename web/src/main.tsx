import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.js'
import React from 'react'

createRoot(document.getElementById('root')).render(
  // When using StrictMode, all API calls are duplicated in DEBUG
  <StrictMode>
    <App />
  </StrictMode>,
)
