package util

import "go.uber.org/zap"

var loglevel string
var logger *zap.Logger

func SetLogLevel(level string) {
  loglevel = level
}

func GetLogger() *zap.Logger {
	if logger != nil {
		return logger
	}

	if loglevel == "debug" {
		logger, _ = zap.NewDevelopment()
    return logger
	}
	logger, _ = zap.NewProduction()
  return logger
}
