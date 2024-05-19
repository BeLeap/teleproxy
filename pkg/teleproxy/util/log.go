package util

import (
	"go.uber.org/zap"
)

var isVerbose bool
var version string
var logger *zap.Logger

func SetVerbosity(verbose bool) {
	isVerbose = verbose
}

func SetVersion(in string) {
  version = in
}

func GetLogger() *zap.Logger {
	if logger != nil {
		return logger
	}

	if isVerbose {
		logger, _ = zap.NewDevelopment()
	} else {
		logger, _ = zap.NewProduction()
	}

	if version != "" {
		logger = logger.With(zap.String("version", version))
	}

	return logger
}
