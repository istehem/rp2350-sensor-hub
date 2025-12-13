const pino = require('pino')

function createLogger() {
  const isDevelopment = process.env.NODE_ENV === 'development'
  const isTest = process.env.NODE_ENV === 'test'

  return pino({
    level: process.env.LOG_LEVEL || (isDevelopment ? 'debug' : 'info'),
    
    // Pretty output for development
    transport: isDevelopment ? {
      target: 'pino-pretty',
      options: {
        colorize: true,
        ignore: 'pid,hostname',
        translateTime: 'yyyy-mm-dd HH:MM:ss'
      }
    } : undefined,
    
    // Disable in tests unless explicitly needed
    enabled: !isTest,
    
    // Add application context
    base: {
      env: process.env.NODE_ENV,
      version: process.env.APP_VERSION
    }
  })
}

module.exports.createLogger = createLogger
