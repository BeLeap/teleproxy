package util

import (
	"go.uber.org/zap"
)

var isVerbose bool
var logger *zap.Logger

func SetVerbosity(verbose bool) {
	isVerbose = verbose
}

func GetLogger() *zap.Logger {
	if logger != nil {
		return logger
	}

	if isVerbose {
		logger, _ = zap.NewDevelopment()
		return logger
	}
	logger, _ = zap.NewProduction()
	return logger
}
